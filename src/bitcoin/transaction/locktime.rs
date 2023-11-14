//! Bitcoin transaction time lock (`nLockTime`).

use crate::util::byte_value::ByteValue4;
use crate::util::byte_value::u32_little_endian;

/// Bitcoin transaction time lock (`nLockTime`).
#[derive(Debug)]
#[derive(Clone, Copy)]
pub struct Locktime {
    /// Value of `nLockTime`.
    value: u32,
}

impl ByteValue4 for Locktime {
    /// Initialize little-endian unsigned 4 byte integer.
    fn of(bytes: &[u8]) -> Self {
        Self { value: u32_little_endian(bytes) }
    }

    /// Return little-endian unsigned 4 byte integer.
    fn bytes(&self) -> [u8; 4] {
        self.value.to_le_bytes()
    }
}

impl std::fmt::Display for Locktime {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let hint = match self.interpretation() {
            LocktimeType::BlockHeight => format!("block height {}", self.value),
            LocktimeType::UnixTimestamp => format!("timestamp {}", self.value),
        };

        write!(f, "<locktime [{}]>", hint)
    }
}

/// Interpretation of the time lock value.
///
/// - `nLockTime < 500000000`: block height
/// - otherwise: a Unix timestamp
#[derive(Debug)]
#[derive(PartialEq)]
pub enum LocktimeType {
    BlockHeight,
    UnixTimestamp
}

impl Locktime {
    /// Returns the interpretation of the time lock value.
    pub fn interpretation(&self) -> LocktimeType {
        match self.value {
            value if value < 500000000_u32 => LocktimeType::BlockHeight,
            _ => LocktimeType::UnixTimestamp,
        }
    }

    /// Returns `value` (to be interpreted as a block height).
    pub fn block_height(&self) -> u32 {
        assert_eq!(self.interpretation(), LocktimeType::BlockHeight);

        self.value
    }

    /// Returns `value` (to be interpreted as a timestamp).
    pub fn timestamp(&self) -> u32 {
        assert_eq!(self.interpretation(), LocktimeType::UnixTimestamp);

        self.value
    }
}
