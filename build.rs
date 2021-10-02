fn main() {
    #[cfg(target_family = "windows")]
    windows::build! {
        Windows::Win32::{
            Networking::WinSock::{SOCKADDR_IN,SOCKADDR_IN6},
            NetworkManagement::IpHelper::{ConvertLengthToIpv4Mask, GetAdaptersAddresses},
        }
    };
}
