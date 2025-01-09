mod blockchain;
mod staking;
mod util;
mod mempool;

use util::{chrono_timestamp, default_validator_set, genesis_block, get_block_with_height, initialize_blockstore_with_genesis, initialize_candidatestore, purge_dbs};
use dotenv::dotenv;
use mempool::{Storage, BlockStore, CandidateStore,};
use std::{env, path::PathBuf};
use blockchain::{block::Block, blockchain::Blockchain};
use staking::validator::{Validator, Vote, get_validator_weight, get_candidate_pool};
use util::{hash_input};




fn main() {
    
}