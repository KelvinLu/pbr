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

use crate::bitcoin::script::ScriptBytes;
use crate::bitcoin::script_types::BitcoinTransactionType;
use crate::crypto::digest::hash_160;
use crate::serialization::bitcoin_address::BitcoinAddress;
use crate::util::byte_string::ByteString;
use crate::util::byte_string::ByteSlice;

impl ScriptBytes {
    /// Indicates that these script bytes match a P2SH locking script template.
    pub fn is_p2sh_locking(&self) -> bool {
        let bytes = self.bytes();

        (bytes.len() == 22) && (bytes[0] == 0xa9_u8) && (bytes[21] == 0x87_u8)
    }

    /// Returns the redeem script hash (20 byte `OP_HASH160`) that this locking script references.
    pub fn locking_redeem_script_hash(&self) -> &[u8] {
        assert!(self.is_p2sh_locking());

        &self.bytes()[1..21]
    }

    /// Creates a locking script for a P2SH UTXO.
    pub fn locking_script_p2sh_redeem_script(redeem_script_bytes: &ScriptBytes) -> Self {
        let mut bytes = [0_u8; 22];

        bytes[0] = 0xa9_u8;
        bytes[1..=20].clone_from_slice(&hash_160(redeem_script_bytes.bytes()));
        bytes[21] = 0x87_u8;

        ScriptBytes::of(&bytes)
    }

    /// Creates a locking script for a P2SH UTXO (given a P2SH address).
    pub fn locking_script_p2sh_address(address: &BitcoinAddress) -> Self {
        assert_eq!(address.transaction_type(), BitcoinTransactionType::P2sh);

        let mut bytes = [0_u8; 22];

        bytes[0] = 0xa9_u8;
        bytes[1..=20].clone_from_slice(address.hash_bytes());
        bytes[21] = 0x87_u8;

        ScriptBytes::of(&bytes)
    }
}
