use std::mem;
use std::net::{Ipv4Addr, Ipv6Addr};
use std::slice::from_raw_parts;

use libc::{getifaddrs, ifaddrs, sockaddr_in, sockaddr_in6, strlen, AF_INET, AF_INET6, malloc};

use crate::interface::Netmask;
use crate::{Error, NetworkInterface, NetworkInterfaceConfig, Result};
use crate::utils::{ipv4_from_in_addr, ipv6_from_in6_addr, make_ipv4_netmask, make_ipv6_netmask};

/// `ifaddrs` struct raw pointer alias
pub type NetIfaAddrPtr = *mut *mut ifaddrs;

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

        let mut network_interfaces: Vec<NetworkInterface> = Vec::new();
        let netifa = addr;

        let has_next = |netifa: NetIfaAddrPtr| {
            if unsafe { (*netifa).is_null() } {
                return false;
            }

            if unsafe { (**netifa).ifa_next.is_null() } {
                return false;
            }

            true
        };

        let mut advance = |network_interface: Option<NetworkInterface>| {
            if let Some(network_interface) = network_interface {
                network_interfaces.push(network_interface);
            }

            unsafe { *netifa = (**netifa).ifa_next };
        };

        while has_next(netifa) {
            let netifa_addr = unsafe { (**netifa).ifa_addr };

            let netifa_family =
                if netifa_addr.is_null() { None }
                else { Some(unsafe { (*netifa_addr).sa_family as i32 }) };

            match netifa_family {
                Some(AF_INET) => {
                    let netifa_addr = netifa_addr;
                    let socket_addr = netifa_addr as *mut sockaddr_in;
                    let internet_address = unsafe { (*socket_addr).sin_addr };
                    let name = make_netifa_name(&netifa)?;
                    let netmask: Netmask<Ipv4Addr> = make_ipv4_netmask(&netifa);
                    let addr = ipv4_from_in_addr(&internet_address)?;
                    let broadcast = make_ipv4_broadcast_addr(&netifa)?;
                    let network_interface =
                        NetworkInterface::new_afinet(name.as_str(), addr, netmask, broadcast);

                    advance(Some(network_interface));
                }
                Some(AF_INET6) => {
                    let netifa_addr = netifa_addr;
                    let socket_addr = netifa_addr as *mut sockaddr_in6;
                    let internet_address = unsafe { (*socket_addr).sin6_addr };
                    let name = make_netifa_name(&netifa)?;
                    let netmask: Netmask<Ipv6Addr> = make_ipv6_netmask(&netifa);
                    let addr = ipv6_from_in6_addr(&internet_address)?;
                    let broadcast = make_ipv6_broadcast_addr(&netifa)?;
                    let network_interface =
                        NetworkInterface::new_afinet6(name.as_str(), addr, netmask, broadcast);

                    advance(Some(network_interface));
                }
                _ => {
                    advance(None);
                }
            }
        }

        Ok(network_interfaces)
    }
}

/// Retrieves the network interface name
fn make_netifa_name(netifa: &NetIfaAddrPtr) -> Result<String> {
    let netifa = *netifa;
    let data = unsafe { (*(*netifa)).ifa_name as *const libc::c_char };
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
fn make_ipv4_broadcast_addr(netifa: &NetIfaAddrPtr) -> Result<Option<Ipv4Addr>> {
    let netifa = *netifa;
    let ifa_dstaddr = unsafe { (*(*netifa)).ifa_ifu };

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
fn make_ipv6_broadcast_addr(netifa: &NetIfaAddrPtr) -> Result<Option<Ipv6Addr>> {
    let netifa = *netifa;
    let ifa_dstaddr = unsafe { (*(*netifa)).ifa_ifu };

    if ifa_dstaddr.is_null() {
        return Ok(None);
    }

    let socket_addr = ifa_dstaddr as *mut sockaddr_in6;
    let internet_address = unsafe { (*socket_addr).sin6_addr };
    let addr = ipv6_from_in6_addr(&internet_address)?;

    Ok(Some(addr))
}

#[cfg(target_os = "linux")]
mod tests {
    #[test]
    fn show_network_interfaces() {
        use super::{NetworkInterface, NetworkInterfaceConfig};

        let network_interfaces = NetworkInterface::show().unwrap();

        assert!(network_interfaces.len() > 1);
    }
}
