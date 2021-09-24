mod error;
mod interface;
mod target;

pub use error::*;

use self::interface::NetworkInterface;

pub type Result<T> = std::result::Result<T, error::Error>;

pub trait NetworkInterfaceConfig {
    /// List system's network interfaces configuration
    fn show() -> Result<Vec<NetworkInterface>>;
}
