use serde::{Serialize, Deserialize};
use crate::blockchain::blockchain::Blockchain;
// use serde_json;
use crate::util::{hash_input, chrono_timestamp,generate_random_number};
use crate::blockchain::block::Block;
use crate::mempool::{CandidateStore, Storage};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]

pub struct Validator {
    pub address: String,
    pub stake: u64
}

pub struct Vote {
    pub block: Block,
    pub stake: u64
}

pub fn get_validator_weight(stake: u64, total_votes: u64) -> u128 {
    let seed = generate_random_number();
    let weight: u128 = (seed as u128 * stake as u128) / total_votes as u128;

    println!("Weight: {}, \n stake: {}, \n total votes: {}, \n seed: {}", weight, stake, total_votes, seed);
    return weight;
}

pub fn get_candidate_pool(storage: &Storage, height: u64) -> Blockchain {
    let serialized_pool = CandidateStore::height(storage, height.clone()).unwrap().expect("Failed to get mempool");
    Blockchain::from_string(serialized_pool)
}


// use crate::transaction::Transaction;

// pub struct Validator {
//     pub accounts: Vec<String>, // may need to be a wallet?
// }

// impl Validator {
//     pub fn new() -> Self {
//         Self {
//             accounts: vec![], // example of PoS I was using hardcoded accounts
//         }
//     }

//     pub fn update(
//         &mut self,
//         transaction: &Transaction,
//         amount: usize) -> bool {
//             if transaction.transaction_output.amount >= amount 
//             && transaction.transacount_output.to == "0".to_string() {
//                 self.accounts.push(transaction.transaction_input.from.to_strong());
//                 true
//             }
//             false
//         }
// }
