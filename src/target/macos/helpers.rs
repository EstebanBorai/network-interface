use libc::ifaddrs;

#[cfg(target_os = "macos")]
extern "C" {
    pub fn lladdr(ptr: *mut ifaddrs) -> *const u8;
}
