//! Transaction input structure.

use crate::bitcoin::transaction::TransactionId;
use crate::bitcoin::transaction::transaction::TransactionParsingError;
use crate::bitcoin::script::ScriptBytes;
use crate::util::byte_string::ByteString;
use crate::util::byte_string::ByteSlice;
use crate::util::byte_string::ByteVector;
use crate::util::byte_value::ByteValue32;
use crate::util::byte_value::u32_little_endian;
use crate::util::varint::read_varint_u64;
use crate::util::varint::varint_u64;

/// Bitcoin transaction input.
#[derive(Debug)]
#[derive(Clone)]
pub struct TransactionInput {
    /// The Transaction ID of a previous transaction.
    pub txid: TransactionId,

    /// Output index of the previous transaction's UTXO (unspent transaction output).
    pub utxo_index: u32,

    /// Script bytes.
    pub script: ScriptBytes,

    /// Sequence number (`nSequence`).
    pub sequence: u32,
}

impl std::fmt::Display for TransactionInput {
    /// Displays the transaction input.
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "<transaction input [{}:{}] [{}] [{}]>", self.txid, self.utxo_index, self.script, self.sequence)
    }
}

impl ByteString for TransactionInput {
    /// Parse a transaction input from a sequence of bytes.
    fn of(bytes: &[u8]) -> Self {
        Self::parse_bytes(bytes).unwrap().0
    }
}

impl ByteVector for TransactionInput {
    /// Return the sequence of bytes representing this transaction input.
    fn bytes(&self) -> Vec<u8> {
        let mut bytes: Vec<u8> = vec![];

        // Previous transaction ID and previous output index.
        bytes.extend_from_slice(&self.txid.bytes());
        bytes.extend_from_slice(&self.utxo_index.to_le_bytes());

        // Variable integer for script length.
        let (varint_bytes, varint_length) = varint_u64(u64::try_from(self.script.bytes().len()).unwrap());

        bytes.extend_from_slice(&varint_bytes[0..varint_length]);

        // The script bytes.
        bytes.extend_from_slice(self.script.bytes());

        // Sequence number, little-endian.
        bytes.extend_from_slice(&self.sequence.to_le_bytes());

        bytes
    }
}

impl TransactionInput {
    /// Create a transaction input.
    pub fn new(txid: TransactionId, utxo_index: u32, script: ScriptBytes, sequence: u32) -> Self {
        Self {
            txid: txid,
            utxo_index: utxo_index,
            script: script,
            sequence: sequence
        }
    }

    /// Parse a transaction input from a sequence of bytes.
    pub fn parse_bytes(bytes: &[u8]) -> Result<(Self, usize), TransactionParsingError> {
        // The overall length of the byte slice.
        let bytes_length = bytes.len();

        // There should be at least enough bytes to read the transaction ID, previous output
        // index, and the leading byte of a script length variable integer.
        if bytes_length <= 36 { return Err(TransactionParsingError::UnexpectedByteLength) }

        // Read the first 32 bytes as the transaction ID associated with the input.
        let txid = TransactionId::of(&bytes[0..32]);

        // Read the next 4 bytes as the output index (of a previous transaction's UTXO)
        // associated with the input.
        let prev_output_index = u32_little_endian(&bytes[32..36]);

        // Parse the next bytes as a variable integer denoting the script length.
        let (script_length, skip_bytes) =
            read_varint_u64(bytes[36..].iter())
                .ok_or(TransactionParsingError::VariableIntegerError)?;

        // The script bytes start after the preceding variable integer bytes.
        let script_byte_index = 36 + skip_bytes;

        // The sequence integer follows the script bytes.
        let sequence_byte_index = script_byte_index + usize::try_from(script_length)?;

        // There should be at least enough bytes to read the script in whole, given its length,
        // as well as the following sequence integer bytes.
        if bytes_length < (sequence_byte_index + 4) { return Err(TransactionParsingError::UnexpectedByteLength) }

        // Read the script bytes.
        let script_bytes = &bytes[script_byte_index..sequence_byte_index];

        // Calculate the overall length of the transaction input, ending with the sequence integer
        // bytes.
        let bytes_read = sequence_byte_index + 4;

        // Read the last 4 bytes as the sequence integer.
        let sequence = u32_little_endian(&bytes[sequence_byte_index..bytes_read]);

        Ok((Self::new(txid, prev_output_index, ScriptBytes::of(script_bytes), sequence), bytes_read))
    }
}
