use crate::block;
use crate::wallet::Wallet;
use crate::transaction::Transaction;
use serde::{Serialize, Deserialize};
use sha2::{Sha256, Digest};


// need to bring in transaction here
pub struct Block {
    pub id: usize,
    pub hash: String,
    pub previous_hash: String,
    pub timestamp: i64,
    pub transaction: Vec<Transaction>,
    pub signature: String,
    pub difficulty: usize, 
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

        pub fn calculate_hash() {

        }
    }

    pub fn genesis() -> Self {
        let id = 0;
        let timestamp = 1726543804; // Unix time as of 9:30 PM MT on 9/16/2024
        let previous_hash = String::from("genesis"); // no previous hash since genesis block
        let transaction = vec![]; // empty vector to store transactions
        let signature = String::from("genesis");
        let difficulty = 5;

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
            transaction,
            validator,
            signature,
            difficulty,
        }
    }

    pub fn is_valid_block(&mut self, block: Block) -> bool {
        
        let prev_block = self.chain.last().unwrap();

        if block.previous_hash != prev_block.hash {
            warn!("Previous block has wrong hash.")
            return false;
        } else if block.hash!= block::calculate_hash(
            &block.id,
            &block.timestamp,
            &block.previous_hash,
            &block.transaction,
            &block.validator,
            &block.difficulty,
        )
        {
            warn!("block with id: {} has invalid hash", block.id);
            false
        } else if prev_block.id + 1 != block.id {
            warn!("block with id: {} does not have valid signature", block.id);
            false

        } else if !self.verify_leader(&block) {
            warn!("block with id: {} has invalid validator", block.id);
            false
        }
        self.execute_transaction(&block);

        info!("Added new block to current chain at {}", &block.timestamp);

        self.chain.push(block);
        self.mempool.clear();
        true
    }
}

pub fn calculate_hash(
    id: &usize,
    timestamp: &i64,
    previous_hash: &str,
    transaction: &Vec<Transaction>, // work on Transaction
) -> String {

    let hash = serde_json::json!({
        "id": id,
        "previous_hash": previous_hash,
        "transactions": transaction,
        "timestamp", timestamp
    });

    Util::hash(&hash.to_string()) // need Util

}

pub fn hash(data: &String) -> String {
    Digest(data.as_bytes()) // how do I use Digest from sha2?
}