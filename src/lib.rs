mod error;
mod interface;
mod target;
mod test;

#[cfg(any(target_os = "linux", target_os = "macos"))]
mod utils;

pub use error::*;
pub use interface::*;

pub type Result<T> = std::result::Result<T, error::Error>;

pub trait NetworkInterfaceConfig {
    /// List system's network interfaces configuration
    fn show() -> Result<Vec<interface::NetworkInterface>>;
}
