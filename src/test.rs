#[cfg(test)]
mod tests {
    use std::net::Ipv4Addr;

    #[cfg(feature = "serde")]
    use serde_test::{Configure, Token, assert_ser_tokens};

    #[allow(unused_imports)]
    use crate::{Addr, V4IfAddr, Netmask, NetworkInterface, NetworkInterfaceConfig};

    #[test]
    fn show_network_interfaces() {
        let network_interfaces = NetworkInterface::show().unwrap();

        println!("{:#?}", network_interfaces);
        assert!(network_interfaces.len() > 1);
    }

    #[test]
    #[cfg(feature = "serde")]
    fn network_interface_serialization() {
        let network_interface = NetworkInterface {
            name: String::from("Supercool"),
            addr: Some(Addr::V4(V4IfAddr {
                ip: Ipv4Addr::new(128, 0, 0, 1),
                broadcast: Some(Ipv4Addr::new(127, 12, 84, 222)),
                netmask: Some(Ipv4Addr::new(128, 1, 135, 24)),
            })),
            mac_addr: Some(String::from("84:62:7a:03:bd:01")),
        };

        assert_ser_tokens(
            &network_interface.compact(),
            &[
                Token::Struct {
                    name: "NetworkInterface",
                    len: 3,
                },
                Token::Str("name"),
                Token::Str("Supercool"),
                Token::Str("addr"),
                Token::Some,
                Token::Struct {
                    name: "V4IfAddr",
                    len: 3,
                },
                Token::Str("ip"),
                Token::Tuple { len: 4 },
                Token::U8(128),
                Token::U8(0),
                Token::U8(0),
                Token::U8(1),
                Token::TupleEnd,
                Token::Str("broadcast"),
                Token::Some,
                Token::Tuple { len: 4 },
                Token::U8(127),
                Token::U8(12),
                Token::U8(84),
                Token::U8(222),
                Token::TupleEnd,
                Token::Str("netmask"),
                Token::Some,
                Token::Tuple { len: 4 },
                Token::U8(128),
                Token::U8(1),
                Token::U8(135),
                Token::U8(24),
                Token::TupleEnd,
                Token::StructEnd,
                Token::Str("mac_addr"),
                Token::Some,
                Token::Str("84:62:7a:03:bd:01"),
                Token::StructEnd,
            ],
        );
    }
}
