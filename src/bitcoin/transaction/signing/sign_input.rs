//! Bitcoin transaction input signing.

use crate::bitcoin::script::ScriptCreationError;
use crate::bitcoin::script_types::BitcoinTransactionType;
use crate::bitcoin::transaction::TransactionId;

#[derive(Debug)]
pub enum TransactionSigningError {
    NoPreviousTransactionFound(TransactionId),
    NoInputFound(TransactionId, usize),

    LockingScriptNotFound(BitcoinTransactionType),

    ScriptCreationError(ScriptCreationError),

    TryFromIntError(std::num::TryFromIntError),
}

impl From<std::num::TryFromIntError> for TransactionSigningError {
    fn from(error: std::num::TryFromIntError) -> Self {
        Self::TryFromIntError(error)
    }
}

impl From<ScriptCreationError> for TransactionSigningError {
    fn from(error: ScriptCreationError) -> Self {
        Self::ScriptCreationError(error)
    }
}
