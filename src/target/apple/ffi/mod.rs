use libc::ifaddrs;

#[cfg(any(target_os = "macos", target_os = "ios"))]
extern "C" {
    pub fn lladdr(ptr: *mut ifaddrs) -> *const u8;
}
