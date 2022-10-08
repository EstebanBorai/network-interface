pub mod ffi;

use std::collections::HashMap;
use std::mem;
use std::net::{Ipv4Addr, Ipv6Addr};
use std::slice::from_raw_parts;

use libc::{AF_INET, AF_INET6, getifaddrs, ifaddrs, malloc, sockaddr_in, sockaddr_in6, strlen, AF_LINK};

use crate::target::ffi::lladdr;
use crate::{Error, NetworkInterface, NetworkInterfaceConfig, Result};
use crate::utils::{
    NetIfaAddrPtr, ipv4_from_in_addr, ipv6_from_in6_addr, make_ipv4_netmask, make_ipv6_netmask,
};

impl NetworkInterfaceConfig for NetworkInterface {
    fn show() -> Result<Vec<NetworkInterface>> {
        let net_ifa_addr_size = mem::size_of::<NetIfaAddrPtr>();
        let addr = unsafe { malloc(net_ifa_addr_size) as NetIfaAddrPtr };
        let getifaddrs_result = unsafe { getifaddrs(addr) };

        if getifaddrs_result != 0 {
            // If `getifaddrs` returns a value different to `0`
            // then an error has ocurred and we must abort
            return Err(Error::GetIfAddrsError(
                String::from("getifaddrs"),
                getifaddrs_result,
            ));
        }

        let mut network_interfaces: HashMap<String, Vec<NetworkInterface>> = HashMap::new();
        let netifa = addr;

        let has_current = |netifa: *mut *mut ifaddrs| unsafe { !(*netifa).is_null() };

        let mut advance = |network_interface: Option<NetworkInterface>| {
            if let Some(network_interface) = network_interface {
                network_interfaces
                    .entry(network_interface.name.clone())
                    .or_insert(vec![])
                    .push(network_interface);
            }

            unsafe { *netifa = (**netifa).ifa_next };
        };

        let mut mac_addresses: HashMap<String, String> = HashMap::new();

        while has_current(netifa) {
            let netifa_addr = unsafe { (**netifa).ifa_addr };
            let netifa_family = unsafe { (*netifa_addr).sa_family as i32 };

            match netifa_family {
                AF_LINK => {
                    let name = make_netifa_name(&netifa)?;
                    let mac = make_mac_addrs(&netifa);

                    mac_addresses.insert(name, mac);

                    advance(None);
                    continue;
                }
                AF_INET => {
                    let netifa_addr = netifa_addr;
                    let socket_addr = netifa_addr as *mut sockaddr_in;
                    let internet_address = unsafe { (*socket_addr).sin_addr };
                    let name = make_netifa_name(&netifa)?;
                    let netmask = make_ipv4_netmask(&netifa);
                    let addr = ipv4_from_in_addr(&internet_address)?;
                    let broadcast = make_ipv4_broadcast_addr(&netifa)?;
                    let network_interface =
                        NetworkInterface::new_afinet(name.as_str(), addr, netmask, broadcast);

                    advance(Some(network_interface));
                    continue;
                }
                AF_INET6 => {
                    let netifa_addr = netifa_addr;
                    let socket_addr = netifa_addr as *mut sockaddr_in6;
                    let internet_address = unsafe { (*socket_addr).sin6_addr };
                    let name = make_netifa_name(&netifa)?;
                    let netmask = make_ipv6_netmask(&netifa);
                    let addr = ipv6_from_in6_addr(&internet_address)?;
                    let broadcast = make_ipv6_broadcast_addr(&netifa)?;
                    let network_interface =
                        NetworkInterface::new_afinet6(name.as_str(), addr, netmask, broadcast);

                    advance(Some(network_interface));
                    continue;
                }
                _ => {
                    advance(None);
                    continue;
                }
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
fn make_netifa_name(netifa: &NetIfaAddrPtr) -> Result<String> {
    let netifa = *netifa;
    let data = unsafe { (*(*netifa)).ifa_name as *mut u8 };
    let len = unsafe { strlen(data as *const i8) };
    let bytes_slice = unsafe { from_raw_parts(data, len) };
    let string = String::from_utf8(bytes_slice.to_vec()).map_err(Error::from)?;

    Ok(string)
}

/// Retrieves the broadcast address for the network interface provided of the
/// AF_INET family.
///
/// ## References
///
/// https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man3/getifaddrs.3.html
fn make_ipv4_broadcast_addr(netifa: &NetIfaAddrPtr) -> Result<Option<Ipv4Addr>> {
    let netifa = *netifa;
    let ifa_dstaddr = unsafe { (*(*netifa)).ifa_dstaddr };

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
/// https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man3/getifaddrs.3.html
fn make_ipv6_broadcast_addr(netifa: &NetIfaAddrPtr) -> Result<Option<Ipv6Addr>> {
    let netifa = *netifa;
    let ifa_dstaddr = unsafe { (*(*netifa)).ifa_dstaddr };

    if ifa_dstaddr.is_null() {
        return Ok(None);
    }

    let socket_addr = ifa_dstaddr as *mut sockaddr_in6;
    let internet_address = unsafe { (*socket_addr).sin6_addr };
    let addr = ipv6_from_in6_addr(&internet_address)?;

    Ok(Some(addr))
}

fn make_mac_addrs(netifa: &NetIfaAddrPtr) -> String {
    let mut mac = [0; 6];
    let mut ptr = unsafe { lladdr(**netifa) };

    for el in &mut mac {
        *el = unsafe { *ptr };
        ptr = ((ptr as usize) + 1) as *const u8;
    }

    format!(
        "{:02x}:{:02x}:{:02x}:{:02x}:{:02x}:{:02x}",
        mac[0], mac[1], mac[2], mac[3], mac[4], mac[5]
    )
}
