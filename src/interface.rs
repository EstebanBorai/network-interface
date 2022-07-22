//! Network Interface abstraction from commonly used fields for nodes from the
//! linked list provided by system functions like `getifaddrs` and
//! `GetAdaptersAddresses`.
use std::fmt::Debug;
use std::net::{Ipv4Addr, Ipv6Addr};

/// An alias for an `Option` that wraps either a `Ipv4Addr` or a `Ipv6Addr`
/// representing the IP for a Network Interface netmask
pub type Netmask<T> = Option<T>;

/// An alias for a boxed array of bytes used to represent a MAC Address
pub type MacAddress = Box<[u8; 6]>;

/// A system's network interface
#[derive(Debug, Clone)]
pub struct NetworkInterface {
    /// Interface's name
    pub name: String,
    /// Interface's address
    pub addr: Option<Addr>,
    /// MAC Address
    pub mac_addr: Option<MacAddress>,
}

/// Network interface address
#[derive(Debug, Clone, Copy)]
pub enum Addr {
    /// IPV4 Interface from the AFINET network interface family
    V4(V4IfAddr),
    /// IPV6 Interface from the AFINET6 network interface family
    V6(V6IfAddr),
}

/// IPV4 Interface from the AFINET network interface family
#[derive(Debug, Clone, Copy)]
pub struct V4IfAddr {
    /// The IP address for this network interface
    pub ip: Ipv4Addr,
    /// The broadcast address for this interface
    pub broadcast: Option<Ipv4Addr>,
    /// The netmask for this interface
    pub netmask: Netmask<Ipv4Addr>,
}

/// IPV6 Interface from the AFINET6 network interface family
#[derive(Debug, Clone, Copy)]
pub struct V6IfAddr {
    /// The IP address for this network interface
    pub ip: Ipv6Addr,
    /// The broadcast address for this interface
    pub broadcast: Option<Ipv6Addr>,
    /// The netmask for this interface
    pub netmask: Netmask<Ipv6Addr>,
}

impl NetworkInterface {
    pub fn new_afinet(
        name: &str,
        addr: Ipv4Addr,
        netmask: Netmask<Ipv4Addr>,
        broadcast: Option<Ipv4Addr>,
        mac_addr: Option<MacAddress>,
    ) -> NetworkInterface {
        let ifaddr_v4 = V4IfAddr {
            ip: addr,
            broadcast,
            netmask,
        };

        NetworkInterface {
            name: name.to_string(),
            addr: Some(Addr::V4(ifaddr_v4)),
            mac_addr,
        }
    }

    pub fn new_afinet6(
        name: &str,
        addr: Ipv6Addr,
        netmask: Netmask<Ipv6Addr>,
        broadcast: Option<Ipv6Addr>,
        mac_addr: Option<MacAddress>,
    ) -> NetworkInterface {
        let ifaddr_v6 = V6IfAddr {
            ip: addr,
            broadcast,
            netmask,
        };

        NetworkInterface {
            name: name.to_string(),
            addr: Some(Addr::V6(ifaddr_v6)),
            mac_addr,
        }
    }
}
