//! Bitcoin witness transaction ID (`wtxid`).

use crate::bitcoin::transaction::TransactionId;
use crate::util::byte_value::ByteValue32;

/// Witness transaction ID (`wtxid`).
#[derive(Clone, Copy)]
#[derive(PartialEq)]
pub struct WitnessTransactionId {
    bytes: TransactionId,
}

impl WitnessTransactionId {
    /// Initialize big-endian 32 byte witness transaction ID.
    pub fn new(bytes: &[u8]) -> Self {
        Self { bytes: TransactionId::new(bytes) }
    }
}

impl ByteValue32 for WitnessTransactionId {
    /// Initialize little-endian 32 byte witness transaction ID.
    fn of(bytes: &[u8]) -> Self {
        Self { bytes: TransactionId::of(bytes) }
    }

    /// Return little-endian 32 byte witness transaction ID.
    fn bytes(&self) -> [u8; 32] {
        self.bytes.bytes()
    }
}

impl std::fmt::Display for WitnessTransactionId {
    /// Displays the witness transaction ID.
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", std::str::from_utf8(&self.bytes.hexadecimal()).unwrap())
    }
}

impl std::fmt::Debug for WitnessTransactionId {
    /// Displays the witness transaction ID.
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.bytes)
    }
}
