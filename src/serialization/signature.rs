//! DER formats for ECDSA signatures.

use crate::crypto::ecdsa::signature::Signature;
use crate::serialization::bytes::count_leading_zero_bytes;
use crate::util::byte_string::ByteString;
use crate::util::byte_string::ByteSlice;
use crate::util::hexadecimal::hexadecimal_encode;
use crate::util::number::U256;

/// DER encoding for ECDSA signatures.
///
/// # Format
///
/// 1. `0x30` - marker byte
/// 2. (1 byte) - remaining byte length
/// 3. `0x20` - marker byte
/// 4. (1 byte) - r-value length, including any padding zero byte
/// 5. (variable) - r-value, big-endian; includes a prefixed padding zero byte if the first nonzero
///    byte is greater than or equal to `0x80`, and removes any other leading zero bytes
/// 6. `0x20` - marker byte
/// 7. (1 byte) - s-value length, including any padding zero byte
/// 8. (variable) - s-value, big-endian; includes a prefixed padding zero byte if the first nonzero
///    byte is greater than or equal to `0x80`, and removes any other leading zero bytes
#[derive(Debug)]
#[derive(Clone)]
pub struct SignatureDerFormatBytes {
    bytes: [u8; 72]
}

impl ByteString for SignatureDerFormatBytes {
    fn of(bytes: &[u8]) -> Self {
        let mut buffer = [0_u8; 72];
        let mut iterator = bytes.iter();

        let header_byte = iterator.next().unwrap();

        assert_eq!(*header_byte, 0x30_u8);

        let length_byte = iterator.next().unwrap();
        let length: usize = (*length_byte + 2_u8).into();

        assert!(length <= 72);

        buffer[0..length].clone_from_slice(bytes);

        Self { bytes: buffer }
    }
}

impl ByteSlice for SignatureDerFormatBytes {
    fn bytes(&self) -> &[u8] { &self.bytes[0..self.length().into()] }
}

impl SignatureDerFormatBytes {
    /// Returns the length of the byte representation, given by the length byte.
    ///
    /// Two bytes are added for the first two bytes in the representation; the header byte and the
    /// length byte itself.
    ///
    /// Byte representations have a maximum length of 72 bytes.
    pub fn length(&self) -> u8 {
        let length = self.bytes[1] + 2_u8;

        assert!(length <= 72);
        length
    }
}

impl std::fmt::Display for SignatureDerFormatBytes {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut buffer = [0_u8; 72 * 2];
        let length: usize = self.length().into();

        hexadecimal_encode(self.bytes(), &mut buffer).unwrap();
        write!(f, "<DER format signature {}>", std::str::from_utf8(&buffer[0..(length * 2)]).unwrap())
    }
}

impl From<&Signature> for SignatureDerFormatBytes {
    /// Produces a DER format byte representation of an ECDSA signature.
    fn from(signature: &Signature) -> Self {
        let mut bytes: [u8; 72] = [0; 72];

        let r_bytes: [u8; 32] = signature.r.to_be_bytes();
        let s_bytes: [u8; 32] = signature.s.to_be_bytes();

        let r_byte_leading_zeroes = count_leading_zero_bytes(&r_bytes);
        let s_byte_leading_zeroes = count_leading_zero_bytes(&s_bytes);

        let r_byte_length = 32_u8 - r_byte_leading_zeroes;
        let s_byte_length = 32_u8 - s_byte_leading_zeroes;

        let r_byte_padding: bool = r_bytes[usize::from(r_byte_leading_zeroes)] >= 0x80_u8;
        let s_byte_padding: bool = s_bytes[usize::from(s_byte_leading_zeroes)] >= 0x80_u8;

        let mut length_byte = 4_u8 + r_byte_length + s_byte_length;
        if r_byte_padding { length_byte += 1 }
        if s_byte_padding { length_byte += 1 }

        bytes[0] = 0x30_u8;
        bytes[1] = length_byte;

        bytes[2] = 0x02_u8;
        bytes[3] = r_byte_length + if r_byte_padding { 1 } else { 0 };

        let mut start_index: usize = 4;
        let mut end_index: usize = start_index + usize::from(r_byte_length) + if r_byte_padding { 1 } else { 0 };

        if r_byte_padding {
            bytes[start_index] = 0x00_u8;
            bytes[(start_index + 1)..end_index].clone_from_slice(&r_bytes[usize::from(r_byte_leading_zeroes)..]);
        } else {
            bytes[start_index..end_index].clone_from_slice(&r_bytes[usize::from(r_byte_leading_zeroes)..]);
        }

        bytes[end_index] = 0x02_u8;
        bytes[end_index + 1] = s_byte_length + if s_byte_padding { 1 } else { 0 };

        start_index = end_index + 2;
        end_index = start_index + usize::from(s_byte_length) + if s_byte_padding { 1 } else { 0 };

        if s_byte_padding {
            bytes[start_index] = 0x00_u8;
            bytes[(start_index + 1)..end_index].clone_from_slice(&s_bytes[usize::from(s_byte_leading_zeroes)..]);
        } else {
            bytes[start_index..end_index].clone_from_slice(&s_bytes[usize::from(s_byte_leading_zeroes)..]);
        }

        Self { bytes: bytes }
    }
}

impl From<SignatureDerFormatBytes> for Signature {
    /// Produces an ECDSA signature from a DER format byte representation.
    fn from(format: SignatureDerFormatBytes) -> Self {
        let mut r_bytes: [u8; 33] = [0; 33];
        let mut s_bytes: [u8; 33] = [0; 33];

        let r_byte_length = usize::from(format.bytes()[3]);

        let zeroes: usize = 33 - r_byte_length;

        let iterator = format.bytes().iter().skip(4).take(r_byte_length);

        for (i, byte) in iterator.enumerate() {
            r_bytes[zeroes + i] = *byte;
        }

        let s_byte_length = usize::from(format.bytes()[5 + r_byte_length]);
        let zeroes: usize = 33 - s_byte_length;

        let iterator = format.bytes().iter().skip(6 + r_byte_length).take(s_byte_length);

        for (i, byte) in iterator.enumerate() {
            s_bytes[zeroes + i] = *byte;
        }

        assert_eq!(r_bytes[0], 0_u8);
        assert_eq!(s_bytes[0], 0_u8);

        let mut truncated_r_bytes: [u8; 32] = [0_u8; 32];
        let mut truncated_s_bytes: [u8; 32] = [0_u8; 32];

        truncated_r_bytes.copy_from_slice(&r_bytes[1..]);
        truncated_s_bytes.copy_from_slice(&s_bytes[1..]);

        Self {
            r: U256::from_be_bytes(truncated_r_bytes),
            s: U256::from_be_bytes(truncated_s_bytes),
        }
    }
}
