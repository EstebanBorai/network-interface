#[cfg(any(target_os = "android", target_os = "linux"))]
mod linux;

#[cfg(any(target_os = "android", target_os = "linux"))]
pub use linux::*;

#[cfg(any(target_os = "macos", target_os = "ios"))]
mod apple;

#[cfg(any(target_os = "macos", target_os = "ios"))]
pub use apple::*;

#[cfg(target_os = "windows")]
mod windows;

#[cfg(target_os = "windows")]
pub use self::windows::*;

#[cfg(not(target_os = "windows"))]
mod getifaddrs;

#[cfg(not(target_os = "windows"))]
pub use getifaddrs::*;
