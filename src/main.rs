mod blockchain;
mod staking;
mod util;
mod mempool;

use util::{hash_input, chrono_timestamp, default_validator_set, get_block_with_height, initialize_blockstore_with_genesis, initialize_candidatestore, purge_dbs};
use dotenv::dotenv;
use mempool::{Storage, BlockStore, CandidateStore,};
use std::{env, path::PathBuf};
use std::error::Error;
use blockchain::{block::Block, blockchain::Blockchain};
use staking::validator::{Validator, Vote, get_validator_weight, get_candidate_pool};

struct Config {
    blocktime: u64,
    block_db_path: String,
    candidate_db_path: String,
    validator_count: u64,
}

impl Config {
    fn from_env() -> Result<Self, util::BlockchainError> {
        Ok(Config {
            blocktime: env::var("DEFAULT_BLOCK_TIME")
                .map_err(|_| util::BlockchainError::StorageError("Missing block time config".to_string()))?
                .parse()
                .map_err(|_| util::BlockchainError::StorageError("Invalid block time format".to_string()))?,
            block_db_path: env::var("DEFAULT_BLOCK_DB_PATH")
                .map_err(|_| util::BlockchainError::StorageError("Missing block DB path".to_string()))?,
            candidate_db_path: env::var("DEFAULT_CANDIDATE_DB_PATH")
            .map_err(|_| util::BlockchainError::StorageError("Missing candidate db path".to_string()))?,
            validator_count: 10,    
        })
    }
}


fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();

    let config = Config::from_env()?;

    let block_storage = Storage {
        path: PathBuf::from(&config.block_db_path)
    };

    let candidate_storage = Storage {
        path: PathBuf::from(&config.candidate_db_path)
    };

    if let Err(e) = initialize_blockstore_with_genesis(&block_storage) {
        eprintln!("Failed to initialize block storage: {:?}", e);
        return Err(Box::new(util::BlockchainError::StorageError(
            "Failed to initialize block storage".to_string()
        )));
    }

    purge_dbs(PathBuf::from(&config.block_db_path), PathBuf::from(&config.candidate_db_path));

    let validators: Vec<Validator> = default_validator_set();

    

    // initialize_blockstore_with_genesis(&block_storage);

    // initialize_candidatestore(&candidate_storage);

    if let Err(e) = initialize_blockstore_with_genesis(&block_storage) {
        eprintln!("Failed to initialize block storage: {:?}", e);
        return Err(Box::new(util::BlockchainError::StorageError(
            "Failed to initialize block storage".to_string()
        )));
    }

    if let Err(e) = initialize_candidatestore(&candidate_storage) {
        eprintln!("Failed to initialize candidate storage: {:?}", e);
        return Err(Box::new(util::BlockchainError::StorageError(
            "Failed to initialize candidate storage".to_string()
        )));
    }

    let mut height: u64 = 1;

    let mut round_participants: Vec<&Validator> = Vec::new();

    loop {
        let prev_block: Block = get_block_with_height(&block_storage, &(height-1));

        if &chrono_timestamp().parse::<u64>().unwrap() > &(prev_block.timestamp.parse::<u64>().unwrap() + config.blocktime) {
            for validator in &validators {
                if round_participants.contains(&validator) {
                    continue;
                }

                let block: Block = Block::generate(prev_block.clone(), hash_input(&chrono_timestamp()), validator.clone())?;

                let _ = CandidateStore::insert(&candidate_storage, height, block);

                println!("{}", format!("Block added to pool by {} for round {}", &validator.address, &height));

                round_participants.push(&validator);
            };

            let pool = match get_candidate_pool(&candidate_storage, height) {
                Ok(pool) => pool,
                Err(e) => {
                    eprintln!("Error getting candidate pool: {}", e);
                    Blockchain { blocks: Vec::new() }
                }
            };

            println!("Blocks proposed: {}", &pool.blocks.len());

            if pool.blocks.len() > 0 {
                let mut votes: Vec<Vote> = Vec::new();

                let mut total_votes: u64 = 0;

                let mut round_weights: Vec<(Block, u128)> = Vec::new();

                for block in pool.blocks{
                    votes.push(Vote { 
                        block: block.clone(), stake: block.clone().validator.stake 
                    });
                    total_votes += &block.validator.stake;
                };

                for vote in &votes {
                    round_weights.push((vote.block.clone(), get_validator_weight(vote.block.validator.stake, total_votes)));
                }

                let lotto_winner: Block = {
                    round_weights.iter().max_by_key(|&(_block, value)| value).unwrap().0.clone()
                };

                println!("{}", format!("Winner: {}, timestamp: {}", &lotto_winner.validator.address, &lotto_winner.timestamp));


                let _ = BlockStore::insert(&block_storage, height, lotto_winner);

                height += 1;

                round_participants = Vec::new();

            }
        }
    }
}