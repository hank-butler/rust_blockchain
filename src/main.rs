use log::{info, warn};
use serde::{Deserialize, Serialize};
use sha2::{Sha256, Digest};
use serde_json;

// use chrono crate for epoch time
// utc:now().timestamp? <- will need this!
// move structs and impl's to .rs files
pub struct Transaction {
    // need to build out this struct
    pub id: usize,
    pub transaction_type: TransactionType, // need to make traits?,
    pub transaction_input: TransactionInput,
    pub transaction_output: TransactionOutput,
}

pub struct Wallet {
    pub key_pair: String,
}

pub struct Block {
    pub id: usize,
    pub hash: String,
    pub previous_hash: String,
    pub timestamp: i64,
    pub transaction: Vec<Transaction>,
    pub signature: String,
    pub difficulty: usize, 
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
    info!("Creation of Genesis Block");
    Block::new(0, 
        String::from("genesis"), 
        timestamp? // could be 0, but will need to get epoch time, 
        vec![], // instantiate empty vector to store transactions
        difficulty, //hardcode?,
        wallet,
    )
}

fn main() {

}
