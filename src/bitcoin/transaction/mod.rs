//! Components of a Bitcoin transaction.

mod transaction;
mod txid;
mod version;
mod locktime;
mod input;
mod utxo;

pub mod fee;
pub mod verify;
pub mod signing;

pub use transaction::Transaction;
pub use transaction::TransactionParsingError;
pub use txid::TransactionId;
pub use version::Version;
pub use locktime::Locktime;
pub use locktime::LocktimeType;
pub use input::TransactionInput;
pub use utxo::UnspentTransactionOutput;
