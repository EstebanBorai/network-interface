use libc::ifaddrs;

extern "C" {
    #[cfg(target_os = "macos")]
    pub fn lladdr(ptr: *mut ifaddrs) -> *const u8;
}
