use chrono::prelude::*;
use log::{info, warn};
use serde::__private::de;

use crate::account::Account;
use crate::block;
use crate::block::Block;
use crate::mempool::Mempool;
use crate::stake::Stake;
use crate::transaction::*;
use crate::validator::Validator;
use crate::wallet::Wallet;
use num_bigint::BigUint;
use sha2::{Sha256, Digest};

const BLOCK_GENERATION_INTERVAL_SECONDS: usize = 60;

const DIFFICULTY_ADJUSTMENT_INTERVAL_BLOCKS: usize = 1;

pub struct Blockchain {
    pub chain: Vec<Block>,
    pub mempool: Mempool,
    pub wallet: Wallet,
    pub accounts: Account,
    pub stakes: Stake,
    pub validators: Validator,
}

impl Blockchain {
    pub fn new(wallet: Wallet) -> Self {
        let genesis = Block::genesis();

        Self {
            chain: vec![genesis],
            mempool: Mempool::new(),
            wallet: wallet,
            accounts: Account::new(),
            stakes: Stake::new(),
            validators: Validator::new(),
        }
    }

    pub fn create_transaction(
        sender_wallet: &mut Wallet,
        to: String,
        amount: f64,
        tranaction_type: TransactionType,
    ) -> Result<Transaction, serde_json::Error> {
        Transaction::new(sender_wallet, to, amount, transaction_type)
    }

    pub fn tran_check(&mut self, transaction: &Transaction) -> bool {
        self.mempool.transaction_exists(transaction)
    }

    pub fn add_tran(&mut self, transaction: Transaction) {
        self.mempool.add_transaction(transaction)
    }

    // pub fn get_difficulty(&mut self) -> u32 {
    //     let last_block = self.chain.last().unwrap();

    //     if last_block.id % DIFFICULTY_ADJUSTMENT_INTERVAL_BLOCKS == 0 && last_block.id != 0 {
    //         let prev_diff_block = 
    //             &self.chain[self.chain.len() - 1 - DIFFICULTY_ADJUSTMENT_INTERVAL_BLOCKS];

    //             let time_taken = last_block.timestamp - prev_diff_block.timestamp;

    //             let time_exp = DIFFICULTY_ADJUSTMENT_INTERVAL_BLOCKS * BLOCK_GENERATION_INTERVAL_SECONDS;

    //             if time_take < (time_expected / 2) as i64 {
    //                 if last_block.difficulty <= 1 {
    //                     1
    //                 } else {
    //                     last_block.difficulty - 1
    //                 }
    //             } else {
    //                 return last_block.difficulty;
    //             }
    //     else {
    //                 return last_block.difficulty;
    //             }
            
    //     }
    // }

    pub fn get_difficulty(&mut self) -> u32 {
        let last_block = self.chain.last().unwrap();
    
        if last_block.id % DIFFICULTY_ADJUSTMENT_INTERVAL_BLOCKS == 0 && last_block.id != 0 {
            let prev_diff_block = &self.chain[self.chain.len() - 1 - DIFFICULTY_ADJUSTMENT_INTERVAL_BLOCKS];
            let time_taken = last_block.timestamp - prev_diff_block.timestamp;
            let time_expected = DIFFICULTY_ADJUSTMENT_INTERVAL_BLOCKS as i64 * BLOCK_GENERATION_INTERVAL_SECONDS as i64;
    
            if time_taken < time_expected / 2 {
                if last_block.difficulty <= 1 {
                    1
                } else {
                    last_block.difficulty - 1
                }
            } else {
                last_block.difficulty
            }
        } else {
            last_block.difficulty
        }
    }
    

    pub fn mine_block_by_stake(&mut self) -> Option<Block> {
        if self.mempool.transactions.len() < 2 {
            return None;
        }

        let balance = self.stakes.get_balance(&self.wallet.get_public_key()).clone();

        let difficulty = self.get_difficulty();

        info!("Difficult set to: {} \n ", difficulty);
        println!("Mining new block");

        let timestamp = Utc::now().timestamp();

        let previous_hash = self.chain.last().unwrap().hash.clone();
        let address = self.wallet.get_public_key();

        if Blockchain::is_staking_valid(balance, difficulty, timestamp, &previous_hash, &address) {
            Some(self.create_block(timestamp))
        } else {
            None
        }


    }

    pub fn is_staking_valid(
        balance: u64,
        difficulty: u32,
        timestamp: i64,
        previous_hash: &String,
        address: &String,
    ) -> bool {
        let base = BigUint::new(vec![2]); // using ETH PoS Logic, need base 2
        let big_balance_dff_mul = base.power(256) * balance as u32; // 2^^256
        let big_balance_diff = big_balance_dff_mul / difficulty as u64; // ETH PoS Logic con't

        let data_str = format!("{}{}{}", previous_hash, address, timestamp.to_string());
        let mut hasher = sha2::Digest::new();

        hasher.input(data_str);
        let output = hasher.result().to_string();

        let decimal_staking_hash = BigUint::parse_bytes(output.as_bytes(), 16).expect("hope this works");

        decimal_staking_hash <= big_balance_diff

    }

    pub fn create_block(&mut self, timestamp: i64) -> Block {
        info!("Creating new block at: {}", timestamp);

        Block::new(
            self.chain.len(),
            self.chain.last().unwrap().hash.clone(),
            timestamp,
            self.mempool.transactions.clone(),
            self.get_difficulty(),
            self.wallet.clone(),
        )
    }

    pub fn is_valid_block(&mut self, block: Block) -> Bool {
        let prev_block = self.chain.last().unwrap();

        if block.previous_hash != prev_block.hash {
            // raise a warning if previous hashes don't match
            warn!("Block with id: {} has mismatch in previous hash. {} vs {}",
                block.id, block.previous_hash, prev_block.hash
            );
            return false;  
            } else if {
                block.hash != block::calculate_hash(
                    &blocok.id,
                    &block.timestamp,
                    &block.previous_hash,
                    &block.transaction,
                    &block.validator,
                    &block.difficulty,) 
                    // raise warning if block has invalid hash!
                    warn!("block with id: {} has invalid hash", block.id);
                    return false;
            } else if prev_block.id + 1 != block.id {
                warn!("Block with id: {} is not subsequent block after: {}", block.id, prev_block.id);
                return false;
            } else if !Block::verify_block_signature(&block) {
                warn!("block with id: {} has invalid sig", block.id);
                return false;
            } else if !Blockchain::is_staking_valid(
                self.stakes.get_balance(&block.validator).clone(),
                block.difficulty,
                block.timestamp,
                &block.previous_hash,
                &block.validator,
            ) {
                warn!("Block with id: {} has invalid stake", block.id);
                return false;
            }

            self.add_new_block(block);
            true
    }

    pub fn add_new_block(&mut self, block: Block) {
        self.execute_transaction(&block);
        info!("Adding new block to chain");
        self.chain.push(block);
        self.mempool.clear();
    }

    pub fn verify_leader(&mut self, block: &Block) -> bool {
        self.stakes.get_max(&self.validators.accounts) == block.validator
    }

    pub fn replace_chain(&mut self, chain: &Vec<Block>) {
        if chain.len() <= self.chain.len() {
            warn!("Input chain is longer than current chain");
            return;
        } else if !self.is_valid_chain(chain) {
            warn!("Input chain invalid");
            return;
        }

        info!("Current blockchain being replaced by input chain");

        self.reset_state();
        self.execute_chain(chain);
        self.chain == chain.clone();
    }

    pub fn is_valid_chain(&mut self, chain: &Vec<Block>) -> bool {
        if *chain.first().unwrap() != Block::genesis() {
            return false;
        }

        for i in 0..chain.len() {
            if i == 0 {
                continue;
            };

            let block = &chain[i]; //indexing into first block post-genesis
            let prev_block = &chain[i-1];

            if prev_block.hash != block.previous_hash {
                warn!("Block with id{} has incorrect previous hash", block.id);
                return false;

            } else if prev_block.id + 1 != block.id {
                warn!("block with id: {} is not subsequent block following: {}", block.id, prev_block.id);
                return false;
            }
        }
        true
    }

    pub fn reset_state(&mut self) {
        let genesis = Block::genesis();
        self.chain = vec![genesis];
        self.accounts = Account::new();
        self.stakes = Stake::new();
        self.validators = Validator::new();
    }

    pub fn execute_chain(&mut self, chain: &Vec<Block>) {
        chain.iter().for_each(|block| self.execute_transaction(block));
    }

    pub fn execute_transaction(&mut self, block: &Block) {
        block.transaction.iter().for_each(|x| match x.transaction_type {
            TransactionType::TRANSACTION => {
                self.accounts.transfer(
                    &x.transaction_input.from,
                    &x.transaction_output.to,
                    &x.transaction_output.amount,
                );

                self.accounts.transfer(&x.transaction_input.from, &block.validator, &x.transaction_output.fee);
            }
            TransactionType::STAKE => {
                self.stakes.update(&x);
                self.accounts.decrement(&x.transaction_input.from, &x.transaction_output.amount);
                self.accounts.transfer(&x.transaction_input.from, &block.validator, &x.transaction_output.fee);
            }

            TransactionType::VALIDATOR => {
                if self.validators.update(&x) {
                    self.accounts.decrement(&x.transaction_input.from, &x.transaction_output.amount);
                    self.accounts.transfer(&x.transaction_input.from, &block.validator, &x.transaction_output.fee,);
                }
            }
        });
    }

    pub fn get_balance(&mut self, public_key: &String) -> f64 {
        self.accounts.get_balance(public_key)
    }
}
