use crate::transaction::Transaction;

pub mod sample_strategy;
pub mod strategy_manager;

pub trait Strategy {
    fn evaluate(&self, transaction: &Transaction) -> Vec<Transaction>;
}