//! Bitcoin transaction script types.

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
