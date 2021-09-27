//! Network Interface abstraction from commonly used fields for nodes from the
//! linked list provided by system functions like `getifaddrs` and
//! `GetAdaptersAddresses`.
use std::net::{Ipv4Addr, Ipv6Addr};

/// A system's network interface
#[derive(Debug)]
pub struct NetworkInterface {
    /// Interface's name
    pub name: String,
    /// Interface's address
    pub addr: Option<Addr>,
}

/// Network interface address
#[derive(Debug)]
pub enum Addr {
    /// IPV4 Interface from the AFINET network interface family
    V4(V4IfAddr),
    /// IPV6 Interface from the AFINET6 network interface family
    V6(V6IfAddr),
}

/// IPV4 Interface from the AFINET network interface family
#[derive(Debug)]
pub struct V4IfAddr {
    /// The IP address for this network interface
    pub ip: Ipv4Addr,
    /// The broadcast address for this interface
    pub broadcast: Option<Ipv4Addr>,
    /// The netmask for this interface
    pub netmask: Option<Ipv4Addr>,
}

/// IPV6 Interface from the AFINET6 network interface family
#[derive(Debug)]
pub struct V6IfAddr {
    /// The IP address for this network interface
    pub ip: Ipv6Addr,
    /// The broadcast address for this interface
    pub broadcast: Option<Ipv6Addr>,
    /// The netmask for this interface
    pub netmask: Option<Ipv6Addr>,
}

impl NetworkInterface {
    pub fn new_afinet(
        name: &str,
        addr: Ipv4Addr,
        netmask: Option<Ipv4Addr>,
        broadcast: Option<Ipv4Addr>,
    ) -> NetworkInterface {
        let ifaddr_v4 = V4IfAddr {
            ip: addr,
            broadcast,
            netmask,
        };

        NetworkInterface {
            name: name.to_string(),
            addr: Some(Addr::V4(ifaddr_v4)),
        }
    }

    pub fn new_afinet6(
        name: &str,
        addr: Ipv6Addr,
        netmask: Option<Ipv6Addr>,
        broadcast: Option<Ipv6Addr>,
    ) -> NetworkInterface {
        let ifaddr_v6 = V6IfAddr {
            ip: addr,
            broadcast,
            netmask,
        };

        NetworkInterface {
            name: name.to_string(),
            addr: Some(Addr::V6(ifaddr_v6)),
        }
    }
}
