use std::collections::HashMap;

use std::net::{Ipv4Addr, Ipv6Addr};
use std::slice::from_raw_parts;

use libc::{
    sockaddr_in, sockaddr_in6, strlen, AF_INET, AF_INET6, if_nametoindex, sockaddr_ll, AF_PACKET,
};

use crate::target::getifaddrs;
use crate::{Error, NetworkInterface, NetworkInterfaceConfig, Result};
use crate::utils::{ipv4_from_in_addr, ipv6_from_in6_addr, make_ipv4_netmask, make_ipv6_netmask};

impl NetworkInterfaceConfig for NetworkInterface {
    fn show() -> Result<Vec<NetworkInterface>> {
        let mut network_interfaces: HashMap<String, Vec<NetworkInterface>> = HashMap::new();
        let mut mac_addresses: HashMap<String, String> = HashMap::new();

        for netifa in getifaddrs()? {
            let netifa_addr = unsafe { (*netifa).ifa_addr };
            let netifa_family = if netifa_addr.is_null() {
                None
            } else {
                Some(unsafe { (*netifa_addr).sa_family as i32 })
            };

            let network_interface = match netifa_family {
                Some(AF_PACKET) => {
                    let name = make_netifa_name(netifa)?;
                    let mac = make_mac_addrs(netifa);
                    mac_addresses.insert(name, mac);
                    None
                }
                Some(AF_INET) => {
                    let socket_addr = netifa_addr as *mut sockaddr_in;
                    let internet_address = unsafe { (*socket_addr).sin_addr };
                    let name = make_netifa_name(netifa)?;
                    let index = netifa_index(netifa);
                    let netmask = make_ipv4_netmask(netifa);
                    let addr = ipv4_from_in_addr(&internet_address)?;
                    let broadcast = make_ipv4_broadcast_addr(netifa)?;
                    Some(NetworkInterface::new_afinet(
                        name.as_str(),
                        addr,
                        netmask,
                        broadcast,
                        index,
                    ))
                }
                Some(AF_INET6) => {
                    let socket_addr = netifa_addr as *mut sockaddr_in6;
                    let internet_address = unsafe { (*socket_addr).sin6_addr };
                    let name = make_netifa_name(netifa)?;
                    let index = netifa_index(netifa);
                    let netmask = make_ipv6_netmask(netifa);
                    let addr = ipv6_from_in6_addr(&internet_address)?;
                    let broadcast = make_ipv6_broadcast_addr(netifa)?;
                    Some(NetworkInterface::new_afinet6(
                        name.as_str(),
                        addr,
                        netmask,
                        broadcast,
                        index,
                    ))
                }
                _ => None,
            };

            if let Some(network_interface) = network_interface {
                network_interfaces
                    .entry(network_interface.name.clone())
                    .or_default()
                    .push(network_interface);
            }
        }

        for (netifa_name, mac_addr) in mac_addresses {
            if let Some(netifas) = network_interfaces.get_mut(netifa_name.as_str()) {
                netifas.iter_mut().for_each(|netifa| {
                    netifa.mac_addr = Some(mac_addr.clone());
                });
            }
        }

        Ok(network_interfaces
            .into_values()
            .flat_map(|x| x.into_iter())
            .collect())
    }
}

/// Retrieves the network interface name
fn make_netifa_name(netifa: *mut libc::ifaddrs) -> Result<String> {
    let data = unsafe { (*netifa).ifa_name as *const libc::c_char };
    let len = unsafe { strlen(data) };
    let bytes_slice = unsafe { from_raw_parts(data as *const u8, len) };

    match String::from_utf8(bytes_slice.to_vec()) {
        Ok(s) => Ok(s),
        Err(e) => Err(Error::ParseUtf8Error(e)),
    }
}

/// Retrieves the broadcast address for the network interface provided of the
/// AF_INET family.
///
/// ## References
///
/// https://man7.org/linux/man-pages/man3/getifaddrs.3.html
fn make_ipv4_broadcast_addr(netifa: *mut libc::ifaddrs) -> Result<Option<Ipv4Addr>> {
    let ifa_dstaddr = unsafe { (*netifa).ifa_ifu };

    if ifa_dstaddr.is_null() {
        return Ok(None);
    }

    let socket_addr = ifa_dstaddr as *mut sockaddr_in;
    let internet_address = unsafe { (*socket_addr).sin_addr };
    let addr = ipv4_from_in_addr(&internet_address)?;

    Ok(Some(addr))
}

/// Retrieves the broadcast address for the network interface provided of the
/// AF_INET6 family.
///
/// ## References
///
/// https://man7.org/linux/man-pages/man3/getifaddrs.3.html
fn make_ipv6_broadcast_addr(netifa: *mut libc::ifaddrs) -> Result<Option<Ipv6Addr>> {
    let ifa_dstaddr = unsafe { (*netifa).ifa_ifu };

    if ifa_dstaddr.is_null() {
        return Ok(None);
    }

    let socket_addr = ifa_dstaddr as *mut sockaddr_in6;
    let internet_address = unsafe { (*socket_addr).sin6_addr };
    let addr = ipv6_from_in6_addr(&internet_address)?;

    Ok(Some(addr))
}

fn make_mac_addrs(netifa: *mut libc::ifaddrs) -> String {
    let netifa_addr = unsafe { (*netifa).ifa_addr };
    let socket_addr = netifa_addr as *mut sockaddr_ll;
    let mac_array = unsafe { (*socket_addr).sll_addr };
    let addr_len = unsafe { (*socket_addr).sll_halen };
    let real_addr_len = std::cmp::min(addr_len as usize, mac_array.len());
    let mac_slice = unsafe { std::slice::from_raw_parts(mac_array.as_ptr(), real_addr_len) };

    mac_slice
        .iter()
        .map(|x| format!("{:02x}", x))
        .collect::<Vec<_>>()
        .join(":")
}

/// Retreives the name for the the network interface provided
///
/// ## References
///
/// https://man7.org/linux/man-pages/man3/if_nametoindex.3.html
fn netifa_index(netifa: *mut libc::ifaddrs) -> u32 {
    let name = unsafe { (*netifa).ifa_name as *const libc::c_char };

    unsafe { if_nametoindex(name) }
}
