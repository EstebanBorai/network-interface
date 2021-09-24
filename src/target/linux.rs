use std::mem;
use std::net::{Ipv4Addr, Ipv6Addr};
use std::slice::from_raw_parts;

use libc::{
    getifaddrs, ifaddrs, sockaddr_in, sockaddr_in6, strlen, AF_INET, AF_INET6, malloc, in_addr,
    in6_addr,
};

use crate::{Error, NetworkInterface, NetworkInterfaceConfig, Result};

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

            let netifa_family = unsafe { (*netifa_addr).sa_family as i32 };

            match netifa_family {
                AF_INET => {
                    let netifa_addr = netifa_addr;
                    let socket_addr = netifa_addr as *mut sockaddr_in;
                    let internet_address = unsafe { (*socket_addr).sin_addr };
                    let name = make_netifa_name(&netifa)?;
                    let netmask = make_ipv4_netmask(&netifa)?;
                    let addr = make_ipv4_addr(&internet_address)?;
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
                    let netmask = make_ipv6_netmask(&netifa)?;
                    let addr = make_ipv6_addr(&internet_address)?;
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

/// Retrieves the Netmask from a `ifaddrs` instance for a network interface
/// from the AF_INET (IPv4) family.
fn make_ipv4_netmask(netifa: &NetIfaAddrPtr) -> Result<Ipv4Addr> {
    let netifa = *netifa;
    let sockaddr = unsafe { (*(*netifa)).ifa_netmask };
    let socket_addr = sockaddr as *mut sockaddr_in;
    let internet_address = unsafe { (*socket_addr).sin_addr };

    make_ipv4_addr(&internet_address)
}

/// Retrieves the Netmask from a `ifaddrs` instance for a network interface
/// from the AF_INET6 (IPv6) family.
fn make_ipv6_netmask(netifa: &NetIfaAddrPtr) -> Result<Ipv6Addr> {
    let netifa = *netifa;
    let sockaddr = unsafe { (*(*netifa)).ifa_netmask };
    let socket_addr = sockaddr as *mut sockaddr_in6;
    let internet_address = unsafe { (*socket_addr).sin6_addr };

    make_ipv6_addr(&internet_address)
}

/// Creates a `Ipv4Addr` from a `in_addr`
fn make_ipv4_addr(internet_address: &in_addr) -> Result<Ipv4Addr> {
    let mut ip_addr = Ipv4Addr::from(internet_address.s_addr);

    if cfg!(target_endian = "little") {
        // due to a difference on how bytes are arranged on a
        // single word of memory by the CPU, swap bytes based
        // on CPU endianess to avoid having twisted IP addresses
        //
        // refer: https://github.com/rust-lang/rust/issues/48819
        ip_addr = Ipv4Addr::from(internet_address.s_addr.swap_bytes());
    }

    Ok(ip_addr)
}

/// Creates a `Ipv6Addr` from a `in6_addr`
fn make_ipv6_addr(internet_address: &in6_addr) -> Result<Ipv6Addr> {
    let ip_addr = Ipv6Addr::from(internet_address.s6_addr);

    Ok(ip_addr)
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
    let addr = make_ipv4_addr(&internet_address)?;

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
    let addr = make_ipv6_addr(&internet_address)?;

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
