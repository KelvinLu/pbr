//! Bitcoin transaction versioning.

use crate::util::byte_value::ByteValue4;
use crate::util::byte_value::u32_little_endian;
use crate::util::hexadecimal::hexadecimal_encode;

/// Bitcoin transaction version bytes.
#[derive(Debug)]
#[derive(Clone, Copy)]
pub struct Version {
    /// Version bytes.
    value: u32
}

impl ByteValue4 for Version {
    /// Initialize little-endian unsigned 4 byte integer.
    fn of(bytes: &[u8]) -> Self {
        Self { value: u32_little_endian(bytes) }
    }

    /// Return little-endian unsigned 4 byte integer.
    fn bytes(&self) -> [u8; 4] {
        self.value.to_le_bytes()
    }
}

impl std::fmt::Display for Version {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut buffer = [0_u8; 4 * 2];

        hexadecimal_encode(&self.bytes(), &mut buffer).unwrap();

        write!(f, "<version bytes {}>", std::str::from_utf8(&buffer).unwrap())
    }
}

impl Version {
    pub fn value(&self) -> u32 {
        self.value
    }
}
