//! UTXOs (unspent transaction outputs).

use crate::bitcoin::transaction::transaction::TransactionParsingError;
use crate::bitcoin::script::ScriptBytes;
use crate::util::byte_string::ByteString;
use crate::util::byte_string::ByteSlice;
use crate::util::byte_string::ByteVector;
use crate::util::byte_value::i64_little_endian;
use crate::util::varint::read_varint_u64;
use crate::util::varint::varint_u64;

/// Bitcoin UTXO (unspent transaction output).
#[derive(Debug)]
#[derive(Clone)]
pub struct UnspentTransactionOutput {
    /// UTXO amount (of satoshi).
    pub amount: i64,

    /// Script bytes.
    pub script: ScriptBytes,
}

impl std::fmt::Display for UnspentTransactionOutput {
    /// Displays the UTXO.
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "<utxo [{}] [{}]>", self.amount, self.script)
    }
}

impl ByteString for UnspentTransactionOutput {
    /// Parse a UTXO from a sequence of bytes.
    fn of(bytes: &[u8]) -> Self {
        Self::parse_bytes(bytes).unwrap().0
    }
}

impl ByteVector for UnspentTransactionOutput {
    /// Return the sequence of bytes representing this UTXO.
    fn bytes(&self) -> Vec<u8> {
        let mut bytes: Vec<u8> = vec![];

        // Amount field, little-endian.
        bytes.extend_from_slice(&self.amount.to_le_bytes());

        // Variable integer for script length.
        let (varint_bytes, varint_length) = varint_u64(u64::try_from(self.script.bytes().len()).unwrap());

        bytes.extend_from_slice(&varint_bytes[0..varint_length]);

        // The script bytes.
        bytes.extend_from_slice(self.script.bytes());

        bytes
    }
}

impl UnspentTransactionOutput {
    pub fn new(amount: i64, script: ScriptBytes) -> Self {
        Self {
            amount: amount,
            script: script,
        }
    }

    /// Parse a byte string for a UTXO.
    pub fn parse_bytes(bytes: &[u8]) -> Result<(Self, usize), TransactionParsingError> {
        // The overall length of the byte slice.
        let bytes_length = bytes.len();

        // There should be at least enough bytes to read the amount field and the leading byte of a
        // script length variable integer.
        if bytes_length < 9 { return Err(TransactionParsingError::UnexpectedByteLength) }

        // Parse the amount field.
        let amount = i64_little_endian(&bytes[0..8]);

        // Parse the next bytes as a variable integer denoting the script length.
        let (script_length, skip_bytes) =
            read_varint_u64(bytes[8..].iter())
                .ok_or(TransactionParsingError::VariableIntegerError)?;

        // The script bytes start after the preceding variable integer bytes.
        let script_byte_index = 8 + skip_bytes;

        // Calculate the overall length of the UTXO.
        let bytes_read = script_byte_index + usize::try_from(script_length)?;

        Ok((Self { amount: amount, script: ScriptBytes::of(&bytes[script_byte_index..bytes_read]) }, bytes_read))
    }
}
