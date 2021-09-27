use std::net::{Ipv4Addr, Ipv6Addr};
use libc::{in6_addr, in_addr};

use crate::Result;

/// Creates a `Ipv4Addr` from a (Unix) `in_addr` taking in account
/// the CPU endianess to avoid having twisted IP addresses.
///
/// refer: https://github.com/rust-lang/rust/issues/48819
#[cfg(target_os = "macos")]
pub fn ipv4_from_in_addr(internet_address: &in_addr) -> Result<Ipv4Addr> {
    let mut ip_addr = Ipv4Addr::from(internet_address.s_addr);

    if cfg!(target_endian = "little") {
        // due to a difference on how bytes are arranged on a
        // single word of memory by the CPU, swap bytes based
        // on CPU endianess to avoid having twisted IP addresses
        ip_addr = Ipv4Addr::from(internet_address.s_addr.swap_bytes());
    }

    Ok(ip_addr)
}

/// Creates a `Ipv6Addr` from a (Unix) `in6_addr`
#[cfg(target_os = "macos")]
pub fn ipv6_from_in6_addr(internet_address: &in6_addr) -> Result<Ipv6Addr> {
    let ip_addr = Ipv6Addr::from(internet_address.s6_addr);

    Ok(ip_addr)
}
