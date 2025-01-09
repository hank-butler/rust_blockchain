use sha2::{Sha256, Digest};
use hex;
use chrono::{DateTime, Local};
use crate::blockchain::block::Block;
use crate::blockchain::blockchain::Blockchain;
use crate::staking::validator::Validator;
use crate::mempool::{CandidateStore, Storage, BlockStore};
use std::fs;
use std::path::{Path, PathBuf};
use rand::Rng;

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

pub fn initialize_blockstore_with_genesis(storage: &Storage){
    let _ = BlockStore::create(storage);
    let _ = BlockStore::insert(storage, 0, genesis_block());
}

pub fn initialize_candidatestore(storage: &Storage) {
    let _ = CandidateStore::create(storage);
}

pub fn get_block_with_height(storage: &Storage, height: &u64) -> Block {
    let mempool = CandidateStore::height(storage, height.clone()).unwrap().expect("Failed to get mempool");
    Block::from_string(mempool)

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







// use ed25519_dalek::{ed25519::Error, PublicKey, Signature, Verifier, SigningKey};
// use hex::FromHexError;
// use log::{warn, info};
// use sha2::{Sha256, Digest};
// use uuid::Uuid;

// pub struct Util;

// pub enum VerifySigErr {
//     DecodeStrError(FromHexError),
//     DecodeHexError(ed25519_dalek::ed25519::Error)
// }

// impl From<FromHexError> for VerifySigErr{
//     fn from (err: FromHexError) -> Self {
//         VerifySigErr::DecodeStrError(err)
//     }
// }

// impl Util {
//     pub fn id() -> Uuid {
//         Uuid::new_v4()
//     }

//     pub fn verify_signature(
//         from_public_key: &String,
//         message: &String,
//         from_signature: &String,
//     ) -> Result<bool, VerifySigErr> {
//         let public_key = hex::decode(from_public_key);
//         let dalek_public_key = PublicKey::from_bytes(&public_key)?;
//         // let dalek_public_key = SigningKey::(&public_key)?;

//         let signature = hex::decode(from_signature)?;
//         let dalek_sig = &Signature::from_bytes(&signature)?;

//         Ok(dalek_public_key
//             .verify(message.as_bytes(), dalek_sig)
//             .is_ok())
//     }

//     pub fn hash(data: &String) -> String {
        
//         let mut hasher = Sha256::default();

//         hasher.input(data);

//         let output = hasher.result().to_string();

//         output
//     }
// }