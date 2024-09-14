use log::{info, warn};
use serde::{Deserialize, Serialize};
use sha2::{Sha256, Digest};
use serde_json;
mod block;


// use chrono crate for epoch time
// utc:now().timestamp? <- will need this!
// move structs and impl's to .rs files
pub struct Transaction {
    // need to build out this struct
    pub id: usize,
    pub transaction_type: TransactionType, // need to make traits?,
    pub transaction_input: TransactionInput,
    pub transaction_output: TransactionOutput,
    // sender, receiver, timestamp, amount
}

pub struct Wallet {
    pub key_pair: String,
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
