//! Segregated witness bytes.

use crate::util::byte_string::ByteString;
use crate::util::byte_string::ByteSlice;
use crate::util::hexadecimal::hexadecimal_encode;

#[derive(Clone)]
pub struct SegWitItem {
    bytes: Vec<u8>
}

impl ByteString for SegWitItem {
    /// Create a segregated witness item consisting of some bytes.
    fn of(bytes: &[u8]) -> Self {
        Self { bytes: Vec::from(bytes) }
    }
}

impl ByteSlice for SegWitItem {
    /// Return the sequence of bytes representing this segregated witness item.
    fn bytes(&self) -> &[u8] {
        &self.bytes
    }
}

impl std::fmt::Display for SegWitItem {
    /// Displays the script bytes.
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "<segwit item {:?}>", self)
    }
}

impl std::fmt::Debug for SegWitItem {
    /// Displays the script bytes.
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut buffer = vec![0_u8; self.bytes.len() * 2];

        hexadecimal_encode(&self.bytes, &mut buffer).unwrap();

        write!(f, "{}", std::str::from_utf8(&buffer).unwrap())
    }
}
