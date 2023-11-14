//! Bitcoin script bytes.

use crate::util::byte_string::ByteString;
use crate::util::byte_string::ByteSlice;
use crate::util::hexadecimal::hexadecimal_encode;

/// Bitcoin script bytes.
#[derive(Clone)]
#[derive(PartialEq)]
pub struct ScriptBytes {
    bytes: Vec<u8>
}

impl ByteString for ScriptBytes {
    fn of(bytes: &[u8]) -> Self {
        Self { bytes: Vec::from(bytes) }
    }
}

impl ByteSlice for ScriptBytes {
    fn bytes(&self) -> &[u8] {
        &self.bytes
    }
}

impl std::fmt::Display for ScriptBytes {
    /// Displays the script bytes.
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "<script bytes {:?}>", self)
    }
}

impl std::fmt::Debug for ScriptBytes {
    /// Displays the script bytes.
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut buffer = vec![0_u8; self.bytes.len() * 2];

        hexadecimal_encode(&self.bytes, &mut buffer).unwrap();

        write!(f, "{}", std::str::from_utf8(&buffer).unwrap())
    }
}
