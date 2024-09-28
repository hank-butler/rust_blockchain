use crate::util::Util;
use crate::block;
use crate::wallet::Wallet;
use crate::transaction::Transaction;
use serde::{Serialize, Deserialize};
// use serde_derive::{Deserialize, Serialize};
use sha2::{Sha256, Digest};
use log::info;

// need to bring in transaction here
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Block {
    pub id: usize,
    pub hash: String,
    pub previous_hash: String,
    pub timestamp: i64,
    pub transaction: Vec<Transaction>,
    pub validator: String,
    pub signature: String,
    pub difficulty: u32,
}

impl PartialEq for Block {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id &&self.previous_hash == other.previous_hash
    }
}

impl Block {
    pub fn new(
        id: usize,
        previous_hash: String,
        timestamp: i64,
        transaction: Vec<Transaction>,
        difficulty: u32,
        mut validator_wallet: Wallet,
    )
 -> Self {
    let validator = validator_wallet.get_public_key();
    let hash = block::calculate_hash(
        &id,
        &timestamp,
        &previous_hash,
        &transaction,
        &validator,
        &difficulty,
    );
    let signature = validator_wallet.sign(&hash);
    
    Self {
        id,
        hash,
        previous_hash,
        timestamp,
        transaction,
        validator,
        signature,
        difficulty,
    }
    }

    pub fn genesis() -> Self {
        let id = 0; // first black gets id of 0
        let timestamp = 000000000; // placeholder for now
        let previous_hash = String::from("genesis"); // hardcoding since first block has no previous hash
        let transaction = vec![];
        let validator = String::from("genesis");
        let signature = String::from("genesis");
        let difficulty = 5; // arbitrary hardcoded with 5

        let hash = block::calculate_hash(
            &id,
            &timestamp,
            &previous_hash,
            &transaction,
            &validator,
            &difficulty,
        );

        Self {
            id,
            hash,
            previous_hash,
            timestamp,
            transaction,
            validator,
            signature,
            difficulty
        }
    }

    pub fn verify_block_signature(block: &Block) -> bool {
        info!("Checking for block signature verification");

        let hash = block::calculate_hash(
            &block.id,
            &block.timestamp,
            &block.previous_hash,
            &block.transaction,
            &block.validator,
            &block.difficulty,
        );

        Util::verify_signature(&block.validator, &hash, &block.signature).is_ok()
    }

}

pub fn calculate_hash (
    id: &usize,
    timestamp: &i64,
    previous_hash: &str,
    transaction: &Vec<Transaction>,
    validator: &String,
    difficulty: &u32,
) -> String {
    info!("Calculating hash");

    let hash = serde_json::json!(
        {"id": id,
        "previous_hash": previous_hash,
        "transactions": transaction,
        "timestamp": timestamp,
        "validator": validator,
        "difficulty": difficulty,}
    );

    Util::hash(&hash.to_string())
}