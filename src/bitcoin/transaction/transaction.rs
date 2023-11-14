//! Bitcoin transactions.

use crate::bitcoin::transaction::TransactionId;
use crate::bitcoin::transaction::Version;
use crate::bitcoin::transaction::Locktime;
use crate::bitcoin::transaction::TransactionInput;
use crate::bitcoin::transaction::UnspentTransactionOutput;
use crate::crypto::digest::hash_256;
use crate::util::byte_string::ByteString;
use crate::util::byte_string::ByteVector;
use crate::util::byte_value::ByteValue4;
use crate::util::byte_value::ByteValue32;
use crate::util::varint::read_varint_u64;
use crate::util::varint::varint_u64;

/// A Bitcoin transaction.
#[derive(Debug)]
#[derive(Clone)]
pub struct Transaction {
    /// Version bytes.
    pub version: Version,

    /// Transaction inputs.
    pub inputs: Vec<TransactionInput>,

    /// UTXOs.
    pub utxos: Vec<UnspentTransactionOutput>,

    /// Locktime.
    pub locktime: Locktime,
}

#[derive(Debug)]
pub enum TransactionParsingError {
    UnexpectedByteLength,
    TryFromIntError(std::num::TryFromIntError),
    VariableIntegerError,
}

impl From<std::num::TryFromIntError> for TransactionParsingError {
    fn from(error: std::num::TryFromIntError) -> Self {
        Self::TryFromIntError(error)
    }
}

impl Transaction {
    /// Compute the transaction ID (`txid`).
    ///
    /// The transaction ID is the "double SHA-256" digest of its byte representation, and is
    /// interpreted as a little-endian number.
    pub fn txid(&self) -> TransactionId {
        TransactionId::of(&hash_256(self.bytes()))
    }

    /// Parse a byte string for a transaction.
    pub fn parse_bytes(bytes: &[u8]) -> Result<Self, TransactionParsingError> {
        // The overall length of the byte slice.
        let bytes_length = bytes.len();

        // There should be at least enough bytes to read the version bytes and the beginning of the
        // transaction input count variable integer.
        if bytes_length < 5 { return Err(TransactionParsingError::UnexpectedByteLength) }

        // Read the version bytes.
        let version_bytes = Version::of(&bytes[0..4]);

        // Parse the variable integer, given the leading byte.
        let (mut variable_count, mut skip_bytes) =
            read_varint_u64(bytes[4..].iter())
                .ok_or(TransactionParsingError::VariableIntegerError)?;

        // The first input starts after the preceding variable integer bytes.
        let mut cursor_index = 4 + skip_bytes;

        // Parse each transaction input.
        let mut transaction_inputs = Vec::<TransactionInput>::with_capacity(usize::try_from(variable_count)?);

        while variable_count > 0 {
            variable_count -= 1;

            // Construct the transaction input.
            let (input, skip_bytes) = TransactionInput::parse_bytes(&bytes[cursor_index..])?;

            transaction_inputs.push(input);
            cursor_index += skip_bytes;
        }

        // There should be at least enough bytes to read the beginning of the UTXO count variable
        // integer.
        if bytes_length <= cursor_index { return Err(TransactionParsingError::UnexpectedByteLength) }

        // Parse the variable integer, given the leading byte.
        (variable_count, skip_bytes) =
            read_varint_u64(bytes[cursor_index..].iter())
                .ok_or(TransactionParsingError::VariableIntegerError)?;

        // The first UTXO starts after the preceding variable integer bytes.
        cursor_index += skip_bytes;

        // Parse each transaction output.
        let mut utxos = Vec::<UnspentTransactionOutput>::with_capacity(usize::try_from(variable_count)?);

        // Parse each UTXO.
        while variable_count > 0 {
            variable_count -= 1;

            let (utxo, skip_bytes) = UnspentTransactionOutput::parse_bytes(&bytes[cursor_index..])?;

            utxos.push(utxo);
            cursor_index += skip_bytes;
        }

        // There should be a remainder of four bytes, for the locktime.
        if bytes_length > cursor_index + 4 { return Err(TransactionParsingError::UnexpectedByteLength) }

        let locktime = Locktime::of(&bytes[cursor_index..]);

        Ok(Self { version: version_bytes, inputs: transaction_inputs, utxos: utxos, locktime: locktime })
    }
}

impl ByteString for Transaction {
    /// Parse a byte string for a transaction.
    fn of(bytes: &[u8]) -> Self {
        Transaction::parse_bytes(bytes).unwrap()
    }
}

impl ByteVector for Transaction {
    /// Return the sequence of bytes representing this transaction.
    fn bytes(&self) -> Vec<u8> {
        let mut bytes: Vec<u8> = vec![];

        // Version bytes.
        bytes.extend_from_slice(&self.version.bytes());

        // Number of inputs.
        let (varint_bytes, varint_length) = varint_u64(u64::try_from(self.inputs.len()).unwrap());

        bytes.extend_from_slice(&varint_bytes[0..varint_length]);

        // Transaction inputs.
        for input in &self.inputs {
            bytes.extend_from_slice(&input.bytes());
        }

        // Number of UTXOs.
        let (varint_bytes, varint_length) = varint_u64(u64::try_from(self.utxos.len()).unwrap());

        bytes.extend_from_slice(&varint_bytes[0..varint_length]);

        // UTXOs.
        for utxo in &self.utxos {
            bytes.extend_from_slice(&utxo.bytes());
        }

        // Locktime.
        bytes.extend_from_slice(&self.locktime.bytes());

        bytes
    }
}
