//! Bitcoin transaction script types.

mod script_types;

pub mod p2pkh;
pub mod p2sh;

pub use script_types::BitcoinTransactionType;
