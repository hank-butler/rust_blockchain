use crate::block;
use crate::wallet::Wallet;
use crate::transaction::Transaction;
use serde::{Serialize, Deserialize};
use sha2::{Sha256, Digest};
use log::info;

// need to bring in transaction here
#[derive(Debug)]
pub struct Block {
    pub id: usize,
    pub hash: String,
    pub previous_hash: String,
    pub timestime: i64,
    pub transaction: Vec<Transaction>,
    pub validator: String,
    pub signature: String,
    pub difficulty: u64,
}

impl PartialEq for Block {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id &&self.previous_hash == other.previous_hash
    }
}