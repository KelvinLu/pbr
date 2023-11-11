//! Sequences of bytes.

/// Represents a sequence of bytes that can be initialized.
pub trait ByteString {
    /// Initialize from a sequence of bytes.
    fn of(bytes: &[u8]) -> Self;
}

/// Represents a sequence of bytes (as a slice).
pub trait ByteSlice {
    /// The sequence of bytes.
    fn bytes(&self) -> &[u8];
}

/// Represents a sequence of bytes (as an allocated vector).
pub trait ByteVector {
    /// The sequence of bytes.
    fn bytes(&self) -> Vec<u8>;
}
