//! Bitcoin transaction verification.

use crate::bitcoin::transaction::Transaction;
use crate::bitcoin::transaction::TransactionId;
use crate::bitcoin::transaction::fee::transaction_fee::FeeCalculationError;
use crate::bitcoin::script::Script;
use crate::bitcoin::script::ScriptError;
use crate::bitcoin::script::Stack;
use crate::bitcoin::script::ScriptExecutionContext;

#[derive(Debug)]
pub enum TransactionVerificationError {
    NoPreviousTransactionFound(TransactionId),
    NoInputFound(TransactionId, usize),

    FeeCalculationError(FeeCalculationError),

    TransactionScriptError(usize),

    TryFromIntError(std::num::TryFromIntError),
}

impl From<std::num::TryFromIntError> for TransactionVerificationError {
    fn from(error: std::num::TryFromIntError) -> Self {
        Self::TryFromIntError(error)
    }
}

impl From<FeeCalculationError> for TransactionVerificationError {
    fn from(error: FeeCalculationError) -> Self {
        Self::FeeCalculationError(error)
    }
}

impl Transaction {
    /// Verifies the transaction.
    pub fn verify<'a, F>(
        &'a self,
        timestamp: u64,
        block_height: u64,
        transaction_retrieval: F
    ) -> Result<bool, TransactionVerificationError>
    where F: Fn(&'a TransactionId) -> Option<&'a Transaction> {
        self.verify_fee(&transaction_retrieval)?;

        for i in 0..self.inputs.len() {
            let verified = self.verify_input(i, timestamp, block_height, &transaction_retrieval)?;

            if !verified { return Ok(false) }
        }

        Ok(true)
    }

    /// Verifies the fees for a transaction.
    pub fn verify_fee<'a, F>(&'a self, transaction_retrieval: F) -> Result<(), FeeCalculationError>
    where F: Fn(&'a TransactionId) -> Option<&'a Transaction> {
        self.fee(transaction_retrieval)?;

        Ok(())
    }

    /// Verifies a transaction input.
    pub fn verify_input<'a, F>(
        &'a self,
        n: usize,
        timestamp: u64,
        block_height: u64,
        transaction_retrieval: F
    ) -> Result<bool, TransactionVerificationError>
    where F: Fn(&'a TransactionId) -> Option<&'a Transaction> {
        let input = self.inputs.get(n).ok_or(TransactionVerificationError::NoInputFound(self.txid(), n))?;

        let prev_txn = transaction_retrieval(&input.txid).ok_or(TransactionVerificationError::NoPreviousTransactionFound(input.txid))?;
        let utxo_index = usize::try_from(input.utxo_index)?;

        let utxo_script_bytes = &prev_txn.utxos.get(utxo_index).ok_or(TransactionVerificationError::NoInputFound(input.txid, utxo_index))?.script;
        let input_script_bytes = &input.script;

        let input_script = Script::try_from(input_script_bytes).map_err(|_| TransactionVerificationError::TransactionScriptError(n))?;
        let utxo_script = Script::try_from(utxo_script_bytes).map_err(|_| TransactionVerificationError::TransactionScriptError(n))?;

        self.evaluate_script(n, input_script, utxo_script, timestamp, block_height)
            .map_err(|_| TransactionVerificationError::TransactionScriptError(n))
    }

    /// Evaluate transaction input (unlocking) and UTXO (locking) scripts.
    fn evaluate_script<'a>(
        &'a self,
        input_index: usize,
        input_script: Script,
        utxo_script: Script,
        timestamp: u64,
        block_height: u64,
    ) -> Result<bool, ScriptError> {
        let context = ScriptExecutionContext::new(&self, input_index, timestamp, block_height);

        Ok(Stack::new(&input_script, &context).adjoin(&utxo_script)?.evaluate()?)
    }
}
