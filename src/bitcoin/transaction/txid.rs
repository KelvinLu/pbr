//! Bitcoin transaction ID (`txid`).

use crate::util::byte_value::ByteValue32;
use crate::util::hexadecimal::hexadecimal_encode;

/// Transaction ID (`txid`).
#[derive(Clone, Copy)]
#[derive(PartialEq)]
pub struct TransactionId {
    /// Little-endian transaction ID bytes.
    bytes: [u8; 32]
}

impl TransactionId {
    /// Initialize big-endian 32 byte transaction ID.
    pub fn new(bytes: &[u8]) -> Self {
        let mut buffer: [u8; 32] = [0_u8; 32];

        buffer.clone_from_slice(bytes);
        buffer.reverse();

        Self { bytes: buffer }
    }
}

impl ByteValue32 for TransactionId {
    /// Initialize little-endian 32 byte transaction ID.
    fn of(bytes: &[u8]) -> Self {
        let mut buffer: [u8; 32] = [0_u8; 32];

        buffer.clone_from_slice(bytes);

        Self { bytes: buffer }
    }

    /// Return little-endian 32 byte transaction ID.
    fn bytes(&self) -> [u8; 32] {
        self.bytes
    }
}

impl std::fmt::Display for TransactionId {
    /// Displays the transaction ID.
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", std::str::from_utf8(&self.hexadecimal()).unwrap())
    }
}

impl std::fmt::Debug for TransactionId {
    /// Displays the transaction ID.
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

impl TransactionId {
    /// Compute the transaction ID (`txid`), as hexadecimal.
    ///
    /// The hexadecimal is based on a big-endian representation.
    pub fn hexadecimal(&self) -> [u8; 64] {
        let mut buffer: [u8; 64] = [0_u8; 64];
        let mut bytes: [u8; 32] = [0_u8; 32];

        bytes.clone_from_slice(&self.bytes);
        bytes.reverse();

        hexadecimal_encode(&bytes, &mut buffer).unwrap();

        buffer
    }
}
