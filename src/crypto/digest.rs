//! Cryptographic digests and hash functions.

pub use sha2::{Digest, Sha256};
pub use hmac::{Hmac, Mac};

/// SHA-256 digest for `data` bytes.
///
/// Returns a 32 byte value.
pub fn sha_256(data: impl AsRef<[u8]>) -> [u8; 32] {
    Sha256::digest(data).into()
}

/// "Double SHA-256" digest for `data` bytes.
///
/// Returns a 32 byte value.
pub fn hash_256(data: impl AsRef<[u8]>) -> [u8; 32] {
    sha_256(sha_256(data))
}
