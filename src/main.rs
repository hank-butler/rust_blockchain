use log::{info, warn};
use serde::{Deserialize, Serialize};
use sha2::{Sha256, Digest};
use serde_json;

pub struct Transaction {
    // need to build out this struct
}

pub struct Wallet {
    // got some work to do here
}

pub struct Block {
    pub id: usize,
    pub hash: String,
    pub previous_hash: String,
    pub timestamp: i64,
    pub transaction: Vec<Transaction> 
}

pub struct Blockchain {
    pub chain: Vec<Block>,
    pub mempool: Mempool,
    pub wallet: Wallet,
    pub stakes: Stake,
    pub validators: Validator,
}

pub struct Chain {
    // Needs to be a vector of Blocks
}

pub struct Mempool {
    // Mempool "memory and pool"
    // mechaninism for storing unconfirmed transactions
}

impl Block {
    pub fn new (
        id: usize,
        previous_hash: String,
        timestamp: i64,
        transaction: Vec<Transaction>,
        difficulty: usize,
        mut validator_wallet: Wallet,
    ) -> Self {
        info!("New block created at {}", timestamp);

        let hash = Block::calculate_hash(&id, &timestamp, &previous_hash, &transaction); // need to work on this.

        let signature = validator_wallet.sign(&hash); // need sign method for validator_wallet

        Self {
            id,
            hash, 
            previous_hash,
            timestamp,
            transaction,
            // validator, need to work on this part
            signature,
            difficulty,
        }
    }
}

pub fn genesis(wallet: Wallet) -> Block {
    info!
}

fn main() {

}
