//! Bitcoin segregated witness transactions.

use crate::bitcoin::transaction::Transaction;
use crate::bitcoin::transaction::TransactionParsingError;
use crate::bitcoin::transaction::Version;
use crate::bitcoin::transaction::TransactionInput;
use crate::bitcoin::transaction::UnspentTransactionOutput;
use crate::bitcoin::transaction::Locktime;
use crate::bitcoin::segwit::wtxid::WitnessTransactionId;
use crate::bitcoin::segwit::field::SegWitField;
use crate::crypto::digest::hash_256;
use crate::util::byte_string::ByteString;
use crate::util::byte_string::ByteVector;
use crate::util::byte_value::ByteValue32;
use crate::util::byte_value::ByteValue4;
use crate::util::varint::read_varint_u64;
use crate::util::varint::varint_u64;

/// A Bitcoin transaction, with witness data.
#[derive(Debug)]
#[derive(Clone)]
pub struct SegWitTransaction {
    transaction: Transaction,
    segwit_fields: Vec<SegWitField>,
}

impl SegWitTransaction {
    /// Compute the witness transaction ID (`wtxid`).
    ///
    /// The witness transaction ID is the "double SHA-256" digest of the transaction byte
    /// representation including segregated witness serialization.
    ///
    /// If all transaction inputs' segregated witness fields are empty, then the normal transaction
    /// ID (`txid`) is returned.
    pub fn wtxid(&self) -> WitnessTransactionId {
        if self.segwit_fields.iter().all(|field| field.is_empty()) {
            return WitnessTransactionId::of(&self.txid().bytes());
        }

        WitnessTransactionId::of(&hash_256(self.bytes()))
    }

    /// Parse a byte string for a segregated witness transaction.
    pub fn parse_bytes(bytes: &[u8]) -> Result<Self, TransactionParsingError> {
        // The overall length of the byte slice.
        let bytes_length = bytes.len();

        // There should be at least enough bytes to read the version bytes, segregated witness
        // marker byte, segregated witness flag byte, and the beginning of the transaction input
        // count variable integer.
        if bytes_length < 7 { return Err(TransactionParsingError::UnexpectedByteLength) }

        // Read the version bytes.
        let version_bytes = Version::of(&bytes[0..4]);

        assert_eq!(bytes[4], 0x00_u8);
        assert_eq!(bytes[5], 0x01_u8);

        // Parse the variable integer, given the leading byte.
        let (mut variable_count, mut skip_bytes) =
            read_varint_u64(bytes[6..].iter())
                .ok_or(TransactionParsingError::VariableIntegerError)?;

        // The first input starts after the preceding variable integer bytes.
        let mut cursor_index = 6 + skip_bytes;

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

        // Segregated witness fields.
        let mut segwit_fields: Vec<SegWitField> = vec![];

        for _ in 0..transaction_inputs.len() {
            let (segwit_field, skip_bytes) = SegWitField::parse_bytes(&bytes[cursor_index..])?;

            segwit_fields.push(segwit_field);
            cursor_index += skip_bytes;
        }

        // There should be a remainder of four bytes, for the locktime.
        if bytes_length > cursor_index + 4 { return Err(TransactionParsingError::UnexpectedByteLength) }

        let locktime = Locktime::of(&bytes[cursor_index..]);

        let transaction = Transaction { version: version_bytes, inputs: transaction_inputs, utxos: utxos, locktime: locktime };

        Ok(
            Self {
                transaction: transaction,
                segwit_fields: segwit_fields
            }
        )
    }
}

impl std::ops::Deref for SegWitTransaction {
    type Target = Transaction;

    fn deref(&self) -> &Self::Target {
        &self.transaction
    }
}

impl ByteString for SegWitTransaction {
    /// Parse a byte string for a transaction.
    fn of(bytes: &[u8]) -> Self {
        SegWitTransaction::parse_bytes(bytes).unwrap()
    }
}

impl ByteVector for SegWitTransaction {
    /// Return the sequence of bytes representing this transaction.
    fn bytes(&self) -> Vec<u8> {
        let mut bytes: Vec<u8> = vec![];

        // Version bytes.
        bytes.extend_from_slice(&self.version.bytes());

        // Segregated witness marker and flag.
        bytes.push(0x00_u8);
        bytes.push(0x01_u8);

        // Number of inputs.
        let (varint_bytes, varint_length) = varint_u64(u64::try_from((*self).inputs.len()).unwrap());

        bytes.extend_from_slice(&varint_bytes[0..varint_length]);

        // Transaction inputs.
        for input in &(*self).inputs {
            bytes.extend_from_slice(&input.bytes());
        }

        // Number of UTXOs.
        let (varint_bytes, varint_length) = varint_u64(u64::try_from((*self).utxos.len()).unwrap());

        bytes.extend_from_slice(&varint_bytes[0..varint_length]);

        // UTXOs.
        for utxo in &(*self).utxos {
            bytes.extend_from_slice(&utxo.bytes());
        }

        // Segregated witness fields and items.
        for field in &self.segwit_fields {
            bytes.extend_from_slice(&field.bytes());
        }

        // Locktime.
        bytes.extend_from_slice(&(*self).locktime.bytes());

        bytes
    }
}
