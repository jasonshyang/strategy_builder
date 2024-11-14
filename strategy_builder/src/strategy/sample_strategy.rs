use crate::transaction::Transaction;
use super::Strategy;

pub struct SampleStrategy;

impl Strategy for SampleStrategy {
    fn evaluate(&self, transaction: &Transaction) -> Vec<Transaction> {
        // Evaluate the transaction
        // Return a vector of transactions
        vec![Transaction::new(transaction.date, transaction.amount)]
    }
}