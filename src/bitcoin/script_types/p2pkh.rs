//! P2PKH (pay to public key hash).
//!
//! # Unlocking script pattern
//!
//! - _`{data length opcode}`_
//! - _`[<data length argument>]` (optional, depending on data length opcode)_
//! - **`<DER signature format bytes>`**
//! - _`0x21`_
//! - **`<SEC point format bytes>`**
//!
//! # Locking script pattern
//!
//! - `OP_DUP`
//! - `OP_HASH160`
//! - _`0x14`_
//! - **`<20 byte public key hash>`**
//! - `OP_EQUALVERIFY`
//! - `OP_CHECKSIG`

use crate::bitcoin::transaction::Transaction;
use crate::bitcoin::transaction::TransactionId;
use crate::bitcoin::transaction::signing::TransactionSigningError;
use crate::bitcoin::script::signature_signing_hash;
use crate::bitcoin::script::TransactionInputCommitment;
use crate::bitcoin::script::DefaultOpCheckSigDigest;
use crate::bitcoin::script::SigHashFlag;
use crate::bitcoin::script::Script;
use crate::bitcoin::script::ScriptBytes;
use crate::bitcoin::script::Element;
use crate::bitcoin::script::DataElement;
use crate::bitcoin::script_types::BitcoinTransactionType;
use crate::crypto::ecdsa::signature::Signature;
use crate::crypto::secp256k1::Secp256k1Point;
use crate::crypto::digest::hash_160;
use crate::serialization::signature::SignatureDerFormatBytes;
use crate::serialization::point::CompressedPointSecFormatBytes;
use crate::serialization::point::UncompressedPointSecFormatBytes;
use crate::serialization::bitcoin_address::BitcoinAddress;
use crate::util::byte_string::ByteString;
use crate::util::byte_string::ByteSlice;
use crate::util::number::U256;

impl ScriptBytes {
    /// Indicates that these script bytes match a P2PKH locking script template.
    pub fn is_p2pkh_locking(&self) -> bool {
        let bytes = self.bytes();

        (bytes.len() == 25) && (bytes[0] == 0x76_u8) && (bytes[1] == 0xa9_u8) && (bytes[2] == 0x14_u8) && (bytes[23] == 0x88_u8) && (bytes[24] == 0xac_u8)
    }

    /// Returns the public key hash (20 byte `OP_HASH160`) of an elliptic curve point byte
    /// representation that this locking script references.
    pub fn locking_public_key_hash(&self) -> &[u8] {
        assert!(self.is_p2pkh_locking());

        &self.bytes()[3..23]
    }

    /// Creates a locking script for a P2PKH UTXO (given a compressed point byte representation).
    pub fn locking_script_p2pkh_compressed_point(point_bytes: &CompressedPointSecFormatBytes) -> Self {
        let mut bytes = [0_u8; 25];

        bytes[0] = 0x76_u8;
        bytes[1] = 0xa9_u8;
        bytes[2] = 0x14_u8;
        bytes[3..=22].clone_from_slice(&hash_160(point_bytes.bytes()));
        bytes[23] = 0x88_u8;
        bytes[24] = 0xac_u8;

        ScriptBytes::of(&bytes)
    }

    /// Creates a locking script for a P2PKH UTXO (given an uncompressed point byte
    /// representation).
    pub fn locking_script_p2pkh_uncompressed_point(point_bytes: &UncompressedPointSecFormatBytes) -> Self {
        let mut bytes = [0_u8; 25];

        bytes[0] = 0x76_u8;
        bytes[1] = 0xa9_u8;
        bytes[2] = 0x14_u8;
        bytes[3..=22].clone_from_slice(&hash_160(point_bytes.bytes()));
        bytes[23] = 0x88_u8;
        bytes[24] = 0xac_u8;

        ScriptBytes::of(&bytes)
    }

    /// Creates a locking script for a P2PKH UTXO (given a P2PKH address).
    pub fn locking_script_p2pkh_address(address: &BitcoinAddress) -> Self {
        assert_eq!(address.transaction_type(), BitcoinTransactionType::P2pkh);

        let mut bytes = [0_u8; 25];

        bytes[0] = 0x76_u8;
        bytes[1] = 0xa9_u8;
        bytes[2] = 0x14_u8;
        bytes[3..=22].clone_from_slice(address.hash_bytes());
        bytes[23] = 0x88_u8;
        bytes[24] = 0xac_u8;

        ScriptBytes::of(&bytes)
    }
}

impl Transaction {
    /// Sign a transaction input against a P2PKH UTXO.
    pub fn signed_input_bytes_p2pkh<'a, F>(
        &'a self,
        n: usize,
        secret_e: U256,
        sighash: SigHashFlag,
        transaction_retrieval: F
    ) -> Result<ScriptBytes, TransactionSigningError>
    where F: Fn(&'a TransactionId) -> Option<&'a Transaction> {
        let input = &self.inputs[n];

        let prev_txn = transaction_retrieval(&input.txid).ok_or(TransactionSigningError::NoPreviousTransactionFound(input.txid))?;
        let utxo_index = usize::try_from(input.utxo_index)?;

        let utxo_script_bytes = &prev_txn.utxos.get(utxo_index).ok_or(TransactionSigningError::NoInputFound(input.txid, utxo_index))?.script;

        if !utxo_script_bytes.is_p2pkh_locking() { return Err(TransactionSigningError::LockingScriptNotFound(BitcoinTransactionType::P2pkh)) }

        let commitment = TransactionInputCommitment::P2pkhLockingScript(utxo_script_bytes.clone());

        let digest = signature_signing_hash(
            self,
            n,
            &commitment,
            sighash,
            &DefaultOpCheckSigDigest {},
        )?;

        let signature = Signature::sign_secp256k1(U256::from_be_bytes(digest), secret_e);
        let point = secret_e * Secp256k1Point::generator_point();

        let signature_bytes = SignatureDerFormatBytes::from(&signature);
        let point_bytes = CompressedPointSecFormatBytes::from(&point.into());

        let mut signature_bytes = signature_bytes.bytes().to_vec();

        signature_bytes.push(sighash.byte());

        let script = Script::new(&[
            Element::Data(DataElement::of(&signature_bytes)),
            Element::Data(DataElement::of(&point_bytes.bytes())),
        ])?;

        Ok(ScriptBytes::from(&script))
    }
}
