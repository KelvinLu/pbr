//! Values expressed as bytes.

/// Represents a value represented as 4 bytes.
pub trait ByteValue4 {
    /// Initialize from a sequence of bytes.
    fn of(bytes: &[u8]) -> Self;

    /// Return a sequence of bytes.
    fn bytes(&self) -> [u8; 4];
}

/// Represents a value represented as 32 bytes.
pub trait ByteValue32 {
    /// Initialize from a sequence of bytes.
    fn of(bytes: &[u8]) -> Self;

    /// Return a sequence of bytes.
    fn bytes(&self) -> [u8; 32];
}

#[inline]
pub fn u32_little_endian(bytes: &[u8]) -> u32 {
    let mut buffer: [u8; 4] = [0_u8; 4];

    buffer.clone_from_slice(bytes);

    u32::from_le_bytes(buffer)
}

#[inline]
pub fn i64_little_endian(bytes: &[u8]) -> i64 {
    let mut buffer: [u8; 8] = [0_u8; 8];

    buffer.clone_from_slice(bytes);

    i64::from_le_bytes(buffer)
}
