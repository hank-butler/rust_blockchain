mod blockchain;
mod staking;
mod util;
mod mempool;

use rocket::http::uri::Path;
use util::{chrono_timestamp, default_validator_set, genesis_block, get_block_with_height, initialize_blockstore_with_genesis, initialize_candidatestore, purge_dbs};
use dotenv::dotenv;
use mempool::{Storage, BlockStore, CandidateStore,};
use std::{env, path::PathBuf};
use blockchain::{block::Block, blockchain::Blockchain};
use staking::validator::{Validator, Vote, get_validator_weight, get_candidate_pool};
use util::{hash_input, initialize_blockstore_with_genesis, initialize_candidatestore};




fn main() {
    dotenv().ok();

    let block_db_path = String = env::var("DEFAULT_BLOCK_DB_PATH").expect("Failed to get block db path");

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
}