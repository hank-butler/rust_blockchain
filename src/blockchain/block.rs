use core::time;

use serde::{Serialize, Deserialize};
use serde_json;
// use crate::utils // <- add to this

#[derive(Serialize, Deserialize, Clone)]
pub struct Block {
    pub index: u64,
    pub timestamp: String,
    pub bpm: String,
    pub hash: String,
    pub prev_hash: Option<String>,
    pub validator: Validator
}

impl Block {
    pub fn new(
        index: u64,
        timestamp: &str,
        bpm: String,
        prev_hash: Option<&String>,
        validator: Validator
    ) -> Block {
        let new_block_hash: String = Block::block_hash_from_params(
            index,
            &timestamp,
            bpm.clone(),
            prev_hash
        );
        Block {
            index: index,
            timestamp: timestamp.to_string(),
            bpm: bpm,
            hash: new_block_hash,
            prev_hash: prev_hash.clone(),
            validator: validator
        }
    }

    pub fn generate(
        prev_block: Block, 
        bpm: String, 
        validator: Validator
    ) -> Block {
        let timestamp: String = chrono_timestamp();
        let new_block_hash: String = Block::block_hash_from_params(prev_block.index + 1, &timestamp, bpm.clone(), Some(&prev_block.hash));
        Block {
            index: prev_block.index + 1,
            timestamp: timestamp,
            bpm: bpm,
            hash: new_block_hash,
            prev_hash: Some(prev_block.hash),
            validator: validator
        }
    }

    pub fn validate(
        block: Block,
        prev_block: Block
    ) -> bool {
        if prev_block.index +1 != block.index {
            return false;
        }
        if prev_block.hash != prev_block.prev_hash.unwrap() {
            return false;
        }

        if block.prev_hash.is_none() && prev_block.index + 1 != 1 {
            return false;
        }

        if Block::block_hash_from_instance(&block) != block.hash {
            return false
        }

        return true
    }

    pub fn to_string(&mut self) -> String {
        serde_json::to_string(self).expect("Block unable to be serialized")
    }

    pub fn from_string(block: String) -> Block {
        serde_json::from_str(&block).expect("Failed to deserialize block")
    }

    pub fn block_hash_from_instance(&self) -> String {
        hash_input(&format!("{}{}{}{}", &self.index, &self.timestamp, &self.bpm, &self.prev_hash.as_ref().unwrap()))
    }

    pub fn block_hash_from_params(index: u64, timestamp: &str, bpm: String, prev_hash: Option<&String>) -> String {
        hash_input(&format!("{}{}{}{}", index, timestamp, bpm, prev_hash))
    }




}







// use crate::util::Util;
// use crate::block;
// use crate::wallet::Wallet;
// use crate::transaction::Transaction;
// use serde::{Serialize, Deserialize};
// // use serde_derive::{Deserialize, Serialize};
// use sha2::{Sha256, Digest};
// use log::info;

// // need to bring in transaction here
// #[derive(Serialize, Deserialize, Debug, Clone)]
// pub struct Block {
//     pub id: usize,
//     pub hash: String,
//     pub previous_hash: String,
//     pub timestamp: i64,
//     pub transaction: Vec<Transaction>,
//     pub validator: String,
//     pub signature: String,
//     pub difficulty: u32,
// }

// impl PartialEq for Block {
//     fn eq(&self, other: &Self) -> bool {
//         self.id == other.id &&self.previous_hash == other.previous_hash
//     }
// }

// impl Block {
//     pub fn new(
//         id: usize,
//         previous_hash: String,
//         timestamp: i64,
//         transaction: Vec<Transaction>,
//         difficulty: u32,
//         mut validator_wallet: Wallet,
//     )
//  -> Self {
//     let validator = validator_wallet.get_public_key();
//     let hash = block::calculate_hash(
//         &id,
//         &timestamp,
//         &previous_hash,
//         &transaction,
//         &validator,
//         &difficulty,
//     );
//     let signature = validator_wallet.sign(&hash);
    
//     Self {
//         id,
//         hash,
//         previous_hash,
//         timestamp,
//         transaction,
//         validator,
//         signature,
//         difficulty,
//     }
//     }

//     pub fn genesis() -> Self {
//         let id = 0; // first black gets id of 0
//         let timestamp = 000000000; // placeholder for now
//         let previous_hash = String::from("genesis"); // hardcoding since first block has no previous hash
//         let transaction = vec![];
//         let validator = String::from("genesis");
//         let signature = String::from("genesis");
//         let difficulty = 5; // arbitrary hardcoded with 5

//         let hash = block::calculate_hash(
//             &id,
//             &timestamp,
//             &previous_hash,
//             &transaction,
//             &validator,
//             &difficulty,
//         );

//         Self {
//             id,
//             hash,
//             previous_hash,
//             timestamp,
//             transaction,
//             validator,
//             signature,
//             difficulty
//         }
//     }

//     pub fn verify_block_signature(block: &Block) -> bool {
//         info!("Checking for block signature verification");

//         let hash = block::calculate_hash(
//             &block.id,
//             &block.timestamp,
//             &block.previous_hash,
//             &block.transaction,
//             &block.validator,
//             &block.difficulty,
//         );

//         Util::verify_signature(&block.validator, &hash, &block.signature).is_ok()
//     }

// }

// pub fn calculate_hash (
//     id: &usize,
//     timestamp: &i64,
//     previous_hash: &str,
//     transaction: &Vec<Transaction>,
//     validator: &String,
//     difficulty: &u32,
// ) -> String {
//     info!("Calculating hash");

//     let hash = serde_json::json!(
//         {"id": id,
//         "previous_hash": previous_hash,
//         "transactions": transaction,
//         "timestamp": timestamp,
//         "validator": validator,
//         "difficulty": difficulty,}
//     );

//     Util::hash(&hash.to_string())
// }