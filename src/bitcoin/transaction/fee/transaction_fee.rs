//! Bitcoin transaction fees.

use crate::bitcoin::transaction::Transaction;
use crate::bitcoin::transaction::TransactionId;

/// Errors thrown when calculating transaction fees.
#[derive(Debug)]
pub enum FeeCalculationError {
    NegativeFee,
    TransactionNotFound,
    UtxoNotFound,
}

impl Transaction {
    /// Calculates the transaction fee.
    ///
    /// The transaction fee is the difference between the sum amounts of the set of transaction
    /// inputs and the set of UTXOs.
    pub fn fee<'a, F>(&'a self, transaction_retrieval: F) -> Result<i64, FeeCalculationError>
    where F: Fn(&'a TransactionId) -> Option<&'a Transaction> {
        let mut input_amount: i64 = 0;

        for input in &self.inputs {
            let prev_txn = transaction_retrieval(&input.txid).ok_or(FeeCalculationError::TransactionNotFound)?;

            let utxo = &prev_txn.utxos.get(usize::try_from(input.utxo_index).unwrap()).ok_or(FeeCalculationError::UtxoNotFound)?;

            input_amount += utxo.amount;
        }

        let output_amount: i64 = self.utxos.iter().map(|utxo| utxo.amount).sum();

        if input_amount >= output_amount {
            Ok(input_amount - output_amount)
        } else {
            Err(FeeCalculationError::NegativeFee)
        }
    }
}
