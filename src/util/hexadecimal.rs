//! Hexadecimal utilities.

use base16ct;

/// Encode a hexadecimal string, writing the resulting bytes to a buffer.
pub fn hexadecimal_encode<'a>(bytes: &[u8], buffer: &'a mut [u8]) -> Result<&'a [u8], base16ct::Error> {
    base16ct::lower::encode(bytes, buffer)
}

/// Decode a hexadecimal string, writing the resulting bytes to a buffer.
pub fn hexadecimal_string<'a>(s: &str, buffer: &'a mut [u8]) -> Result<&'a [u8], base16ct::Error> {
    base16ct::lower::decode(s, buffer)
}
