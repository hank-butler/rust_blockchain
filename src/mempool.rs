// use log::warn;

use crate::transaction::Transaction;

pub struct Mempool {
    pub transactions: Vec<Transaction>,
}

impl Mempool {
    pub fn new() -> Self {
        Self {
            transactions: vec![],
        }
    }

    pub fn add_transaction(&mut self, transaction: Transaction) {
        self.transactions.push(transaction);
    }

    pub fn transaction_exists(&mut self, transaction: &Transaction) -> bool {
        self.transactions.contains(transaction)
    }

    pub fn clear(&mut self) {
        self.transactions.clear() // learned about .clear() method to remove all elements from a vector
    }
}