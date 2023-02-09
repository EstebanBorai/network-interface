use std::mem;
use crate::{Error, Result};

pub struct IfAddrIterator {
    base: *mut libc::ifaddrs,
    next: *mut libc::ifaddrs,
}

impl Iterator for IfAddrIterator {
    type Item = *mut libc::ifaddrs;

    fn next(&mut self) -> Option<Self::Item> {
        let next = unsafe { (*self.next).ifa_next };
        if next.is_null() {
            None
        } else {
            self.next = next;
            Some(next)
        }
    }
}

impl Drop for IfAddrIterator {
    fn drop(&mut self) {
        unsafe { libc::freeifaddrs(self.base) }
    }
}

pub fn getifaddrs() -> Result<IfAddrIterator> {
    let mut addr = mem::MaybeUninit::<*mut libc::ifaddrs>::uninit();
    match unsafe { libc::getifaddrs(addr.as_mut_ptr()) } {
        0 => Ok(IfAddrIterator {
            base: unsafe { addr.assume_init() },
            next: unsafe { addr.assume_init() },
        }),
        getifaddrs_result => Err(Error::GetIfAddrsError(
            String::from("getifaddrs"),
            getifaddrs_result,
        )),
    }
}
