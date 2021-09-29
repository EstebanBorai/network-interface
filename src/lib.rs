mod error;
mod interface;
mod target;

#[cfg(any(target_os = "linux", target_os = "macos"))]
mod utils;

pub use error::*;

use self::interface::NetworkInterface;

pub type Result<T> = std::result::Result<T, error::Error>;

pub trait NetworkInterfaceConfig {
    /// List system's network interfaces configuration
    fn show() -> Result<Vec<NetworkInterface>>;
}
