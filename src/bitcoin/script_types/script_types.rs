//! Bitcoin transaction script types.

use crate::bitcoin::script::ScriptBytes;

/// Represents a Bitcoin transaction script type.
#[derive(Debug)]
#[derive(Clone, Copy)]
#[derive(PartialEq)]
pub enum BitcoinTransactionType {
    /// Unknown transaction type.
    Unknown,

    /// P2PKH (pay to public key hash).
    P2pkh,

    /// P2SH (pay to script hash).
    P2sh,
}

impl ScriptBytes {
    /// Heuristically determines a locking script's transaction type.
    pub fn locking_script_type(&self) -> BitcoinTransactionType {
        if self.is_p2pkh_locking() { return BitcoinTransactionType::P2pkh }
        if self.is_p2sh_locking() { return BitcoinTransactionType::P2sh }

        return BitcoinTransactionType::Unknown;
    }
}
