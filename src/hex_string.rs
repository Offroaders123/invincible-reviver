use std::fmt::Write;

pub trait HexString {
    fn to_hex_lowercase(&self) -> String;
}

impl HexString for [u8] {
    fn to_hex_lowercase(&self) -> String {
        let mut hex_string: String = String::with_capacity(self.len() * 2);
        for byte in self {
            write!(&mut hex_string, "{:02x}", byte).unwrap();
        }
        hex_string
    }
}