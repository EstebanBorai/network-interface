#[cfg(windows)]
pub mod hex;
#[cfg(any(
    target_os = "android",
    target_os = "linux",
    target_os = "ios",
    target_os = "macos"
))]
mod unix;

#[cfg(any(
    target_os = "android",
    target_os = "linux",
    target_os = "ios",
    target_os = "macos"
))]
pub use unix::*;
