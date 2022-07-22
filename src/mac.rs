#[derive(Clone, Debug)]
pub struct MacAddress(Address);

#[derive(Clone, Debug)]
pub enum Address {
    V6(Box<[u8; 6]>),
    V8(Box<[u8; 8]>),
}

impl MacAddress {
    pub fn new(addr: Address) -> Self {
        MacAddress(addr)
    }

    pub fn new_v6(a: u8, b: u8, c: u8, d: u8, e: u8, f: u8) -> Self {
        MacAddress(Address::V6(Box::new([a, b, c, d, e, f])))
    }

    pub fn new_v8(a: u8, b: u8, c: u8, d: u8, e: u8, f: u8, g: u8, h: u8) -> Self {
        MacAddress(Address::V8(Box::new([a, b, c, d, e, f, g, h])))
    }
}
