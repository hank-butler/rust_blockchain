mod blockchain;
mod staking;
mod util;
mod mempool;

use chrono::round;
use rocket::http::uri::Path;
use util::{hash_input, chrono_timestamp, default_validator_set, genesis_block, get_block_with_height, initialize_blockstore_with_genesis, initialize_candidatestore, purge_dbs};
use dotenv::dotenv;
use mempool::{Storage, BlockStore, CandidateStore,};
use std::{env, path::PathBuf};
use blockchain::{block::Block, blockchain::Blockchain};
use staking::validator::{Validator, Vote, get_validator_weight, get_candidate_pool};




fn main() {
    dotenv().ok();

    let block_db_path: String = env::var("DEFAULT_BLOCK_DB_PATH").expect("Failed to get block db path");

    let candidate_db_path: String = env::var("DEFAULT_CANDIDATE_DB_PATH").expect("failed to get candidate db path");

    purge_dbs(PathBuf::from(&block_db_path), PathBuf::from(&candidate_db_path));;

    let blocktime: u64 = env::var("DEFAULT_BLOCK_TIME").expect("Failed to get block time").parse().expect("Failed to parse block time");

    let validators: Vec<Validator> = default_validator_set();

    let block_storage = Storage {
        path: PathBuf::from(block_db_path)
    };

    let candidate_storage = Storage{
        path: PathBuf::from(candidate_db_path)
    };

    initialize_blockstore_with_genesis(&block_storage);

    initialize_candidatestore(&candidate_storage);

    let mut height: u64 = 1;

    let mut round_participants: Vec<&Validator> = Vec::new();

    loop {
        let prev_block: Block = get_block_with_height(&block_storage, &(height-1));

        if &chrono_timestamp().parse::<u64>().unwrap() > &(prev_block.timestamp.parse::<u64>().unwrap() + blocktime) {
            for validator in &validators {
                if round_participants.contains(&validator) {
                    continue;
                }

                let block: Block = Block::generate(prev_block.clone(), hash_input(&chrono_timestamp()), validator.clone());

                let _ = CandidateStore::insert(&candidate_storage, height, block);

                println!("{}", format!("Block added to pool by {} for round {}", &validator.address, &height));

                round_participants.push(&validator);
            };

            let pool: Blockchain = get_candidate_pool(&candidate_storage, height);

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