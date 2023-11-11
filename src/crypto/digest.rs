//! Cryptographic digests and hash functions.

pub use sha2::{Digest as Sha2Digest, Sha256};
pub use ripemd::{Digest as RipemdDigest, Ripemd160};
pub use hmac::{Hmac, Mac};

/// SHA-256 digest for `data` bytes.
///
/// Returns a 32 byte value.
pub fn sha_256(data: impl AsRef<[u8]>) -> [u8; 32] {
    Sha256::digest(data).into()
}

/// RIPEMD-160 digest for `data` bytes.
///
/// Returns a 20 byte value.
pub fn ripemd_160(data: impl AsRef<[u8]>) -> [u8; 20] {
    Ripemd160::digest(data).into()
}

/// "Double SHA-256" digest for `data` bytes.
///
/// Returns a 32 byte value.
pub fn hash_256(data: impl AsRef<[u8]>) -> [u8; 32] {
    sha_256(sha_256(data))
}

/// RIPEMD-160 digest of a SHA-256 digest of `data` bytes.
///
/// Returns a 20 byte value.
pub fn hash_160(data: impl AsRef<[u8]>) -> [u8; 20] {
    ripemd_160(sha_256(data))
}
