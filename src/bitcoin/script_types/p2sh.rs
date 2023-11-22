//! P2SH (pay to script hash).
//!
//! BIP-16 compliant Bitcoin nodes support P2SH by amending the script evaluation process.
//!
//! Unlocking scripts are specified as scripts whose last element is a data element known as a
//! "redeem script" (prior script elements may be included for use as arguments or additional
//! instructions).
//!
//! Upon successful execution of the locking script pattern, which validates a commitment (the
//! redeem script hash) against the unlocking script's redeem script, the redeem script data
//! element is parsed as an additional, executable script.
//!
//! The reedem script, reified as instructions and data, is then executed. The stack and its
//! contents continue to be used and processed by the redeem script.
//!
//! # Unlocking script pattern
//!
//! - `[{...}]`
//! - _`{data length opcode}`_
//! - _`[<data length argument>]` (optional, depending on data length opcode)_
//! - **`<redeem script bytes>`**
//!
//! # Locking script pattern
//!
//! - `OP_HASH160`
//! - _`0x14`_
//! - **`<20 redeem script hash>`**
//! - `OP_EQUAL`

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
use crate::crypto::digest::hash_160;
use crate::serialization::signature::SignatureDerFormatBytes;
use crate::serialization::bitcoin_address::BitcoinAddress;
use crate::util::byte_string::ByteString;
use crate::util::byte_string::ByteSlice;
use crate::util::number::U256;

impl ScriptBytes {
    /// Indicates that these script bytes match a P2SH locking script template.
    pub fn is_p2sh_locking(&self) -> bool {
        let bytes = self.bytes();

        (bytes.len() == 23) && (bytes[0] == 0xa9_u8) && (bytes[22] == 0x87_u8)
    }

    /// Returns the redeem script hash (20 byte `OP_HASH160`) that this locking script references.
    pub fn locking_redeem_script_hash(&self) -> &[u8] {
        assert!(self.is_p2sh_locking());

        &self.bytes()[2..22]
    }

    /// Creates a locking script for a P2SH UTXO.
    pub fn locking_script_p2sh_redeem_script(redeem_script_bytes: &ScriptBytes) -> Self {
        let mut bytes = [0_u8; 23];

        bytes[0] = 0xa9_u8;
        bytes[1] = 0x14_u8;
        bytes[2..=21].clone_from_slice(&hash_160(redeem_script_bytes.bytes()));
        bytes[22] = 0x87_u8;

        ScriptBytes::of(&bytes)
    }

    /// Creates a locking script for a P2SH UTXO (given a P2SH address).
    pub fn locking_script_p2sh_address(address: &BitcoinAddress) -> Self {
        assert_eq!(address.transaction_type(), BitcoinTransactionType::P2sh);

        let mut bytes = [0_u8; 23];

        bytes[0] = 0xa9_u8;
        bytes[1] = 0x14_u8;
        bytes[1..=20].clone_from_slice(address.hash_bytes());
        bytes[21] = 0x87_u8;

        ScriptBytes::of(&bytes)
    }
}

impl Transaction {
    /// Sign a transaction input against a P2SH UTXO.
    ///
    /// Requires the redeem script (expressed as bytes) to be provided, which will be placed in a
    /// data element.
    ///
    /// Additional script (expressed as bytes) may also be provided, which will be placed before
    /// the redeem script data element in the returned signing transaction input script.
    pub fn redeem_script_input_bytes_p2sh<'a, F>(
        &'a self,
        n: usize,
        additional_script: Option<&ScriptBytes>,
        redeem_script: &ScriptBytes,
        transaction_retrieval: F
    ) -> Result<ScriptBytes, TransactionSigningError>
    where F: Fn(&'a TransactionId) -> Option<&'a Transaction> {
        let input = &self.inputs[n];

        let prev_txn = transaction_retrieval(&input.txid).ok_or(TransactionSigningError::NoPreviousTransactionFound(input.txid))?;
        let utxo_index = usize::try_from(input.utxo_index)?;

        let utxo_script_bytes = &prev_txn.utxos.get(utxo_index).ok_or(TransactionSigningError::NoInputFound(input.txid, utxo_index))?.script;

        if !utxo_script_bytes.is_p2sh_locking() { return Err(TransactionSigningError::LockingScriptNotFound(BitcoinTransactionType::P2sh)) }

        let mut script = Script::new(&[Element::Data(DataElement::of(&redeem_script.bytes()))])?;

        if let Some(script_bytes) = additional_script {
            script = Script::try_from(script_bytes)?.concatenate(&script)?
        }

        Ok(ScriptBytes::from(&script))
    }

    /// Sign a transaction input against a P2SH UTXO.
    ///
    /// Creates a signature against a particular redeem script.
    ///
    /// Returns a script (expressed as bytes) containing the signature as a data element.
    pub fn signed_input_bytes_p2sh<'a>(
        &'a self,
        n: usize,
        secret_e: U256,
        redeem_script: &ScriptBytes,
        sighash: SigHashFlag,
    ) -> Result<ScriptBytes, TransactionSigningError> {
        let commitment = TransactionInputCommitment::RedeemScript(redeem_script.clone());

        let digest = signature_signing_hash(
            self,
            n,
            &commitment,
            sighash,
            &DefaultOpCheckSigDigest {},
        )?;

        let signature = Signature::sign_secp256k1(U256::from_be_bytes(digest), secret_e);
        let signature_bytes = SignatureDerFormatBytes::from(&signature);
        let mut signature_bytes = signature_bytes.bytes().to_vec();

        signature_bytes.push(sighash.byte());

        let script = Script::new(&[Element::Data(DataElement::of(&signature_bytes))])?;

        Ok(ScriptBytes::from(&script))
    }
}
