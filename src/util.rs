use sha2::{Sha256, Digest};
use hex;
use chrono::Local;
use crate::blockchain::block::Block;
use crate::staking::validator::Validator;
use crate::mempool::{CandidateStore, Storage, BlockStore};
use std::fs;
use std::path::{Path, PathBuf};
use std::error::Error;
use std::fmt;
use rand::Rng;

#[derive(Debug)]
pub enum BlockchainError {
    StorageError(String),
    ValidationError(String),
    BlockNotFound(u64),
    DatabaseError(rusqlite::Error),
}

impl fmt::Display for BlockchainError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            BlockchainError::StorageError(msg) => write!(f, "Storage Error: {}", msg),
            BlockchainError::ValidationError(msg) => write!(f, "Validattion Error: {}", msg),
            BlockchainError::BlockNotFound(height) => write!(f, "Block not found at height: {}", height),
            BlockchainError::DatabaseError(e) => write!(f, "Database error: {}", e),

        }
    }
}

impl Error for BlockchainError {}

pub fn hash_input(input: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(input);
    let result = hasher.finalize();
    hex::encode(result)
}

pub fn chrono_timestamp() -> String {
    Local::now().timestamp().to_string()
}

pub fn genesis_block() -> Block {
    let timestamp = chrono_timestamp();

    Block{
        index: 0,
        timestamp: timestamp.clone(),
        bpm: String::from("0"),
        hash: hash_input(&timestamp),
        prev_hash: None,
        validator: Validator{
            address: "0x00".to_string(),
            stake: 0
        }
    }
}

pub fn create_validator_set(
    n: u64,
    stakes: Vec<u64>
) -> Vec<Validator>{
    let mut validators: Vec<Validator> = Vec::new();
    for i in 0..n{
        validators.push(Validator{
            address: format!("validator_{}", i),
            stake: stakes[i as usize]
        })
    }
    validators
}

pub fn default_validator_set() -> Vec<Validator>{
    let stakes: Vec<u64> = vec![1000, 1000, 1000, 1000, 1000, 1000, 1000, 1000, 1000, 1000];

    create_validator_set(stakes.len() as u64, stakes)
}

pub fn initialize_blockstore_with_genesis(storage: &Storage) -> Result<(), Box<dyn std::error::Error>> {
    // let _ = BlockStore::create(storage);
    // let _ = BlockStore::insert(storage, 0, genesis_block());
    BlockStore::create(storage)?;
    BlockStore::insert(storage, 0, genesis_block())?;
    Ok(())
    

}

pub fn initialize_candidatestore(storage: &Storage) -> Result<(), Box<dyn std::error::Error>> {
    // let _ = CandidateStore::create(storage);
    CandidateStore::create(storage)?;
    Ok(())
}

pub fn get_block_with_height(storage: &Storage, height: &u64) -> Block {
    // let mempool = CandidateStore::height(storage, height.clone()).unwrap().expect("Failed to get mempool");
    // Block::from_string(mempool)
    match BlockStore::height(storage, height.clone()) {
        Ok(Some(block_string)) => Block::from_string(block_string),
        Ok(None) => panic!("No block found at height: {}", height),
        Err(e) => panic!("Error retrieving block: {:?}", e),
    }

}

pub fn purge_dbs(blockstorage: PathBuf, candidatestorage: PathBuf) {
    if Path::new(&blockstorage).exists() {
        match fs::remove_file(blockstorage) {
            Ok(_) => println!("Blockstorage deleted successfully"),
            Err(e) => eprintln!("Error deleting: {:?}", e),
        }
        
    } else {
        println!("Block storage not found");
    }

    if Path::new(&candidatestorage).exists() {
        match fs::remove_file(candidatestorage) {
            Ok(_) => println!("Candidate storage deleted successfully."),
            Err(e) => eprintln!("Error deleting candidate storage, {:?}", e),
        }
    } else {
        println!("Warning: Candidate storage does not exist.");
    }
}

pub fn generate_random_number() -> u64{
    rand::thread_rng().gen()
}



