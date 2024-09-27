use chrono::prelude::*;
use log::{info, warn};

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

    pub fn get_difficulty(&mut self) -> u32 {
        let last_block = self.chain.last().unwrap();

        if last_block.id % DIFFICULTY_ADJUSTMENT_INTERVAL_BLOCKS == 0 && last_block.id != 0 {
            let prev_diff_block = 
                &self.chain[self.chain.len() - 1 - DIFFICULTY_ADJUSTMENT_INTERVAL_BLOCKS];

                let time_taken = last_block.timestamp - prev_diff_block.timestamp;

                let time_exp = DIFFICULTY_ADJUSTMENT_INTERVAL_BLOCKS * BLOCK_GENERATION_INTERVAL_SECONDS;

                if time_take < (time_expected / 2) as i64 {
                    if last_block.difficulty <= 1 {
                        1
                    } else {
                        last_block.difficulty - 1
                    }
                } else {
                    last_block.difficulty
                } else {
                    last_block.difficulty
                }
            
        }
    }

    pub fn mine_block_by_stake(&mut self) -> Option<Block> {
        if self.mempool.transactions.len() < 2 {
            None
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

    
}
