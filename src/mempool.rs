use rusqlite;
use rusqlite::{Connection, Result};
use std::path::PathBuf;
use crate::blockchain::block::Block;
use crate::blockchain::blockchain::Blockchain;
use crate::util::hash_input;
use std::env;
use dotenv::dotenv;

pub struct Storage {
    pub path: PathBuf
}

pub trait BlockStore {
    fn create(&self) -> Result<()>;
    fn insert(&self, height: u64, block: Block) -> Result<()>;
    fn height(&self, height: u64) -> Result<Option<String>>;
}

pub trait CandidateStore {
    fn create(&self) -> Result<()>;
    fn insert(&self, height: u64, block: Block) -> Result<()>;
    fn height(&self, height: u64) -> Result<Option<String>>;
}

impl BlockStore for Storage {
    fn create(&self) -> Result<()> {
        let conn: Connection = Connection::open(&self.path)?;

        conn.execute(
            "CREATE TABLE IF NOT EXISTS data (
                id INTEGER PRIMARY KEY,
                height INTEGER,
                block TEXT NOT NULL    
            )",
        [],    
        )?;
        Ok(())
    }

    fn insert(&self, height: u64, mut block: Block) -> Result<()> {
        let conn: Connection = Connection::open(&self.path)?;

        conn.execute("INSERT INTO data (height, block) VALUES (?1, ?2)",
        &[&height.to_string(), &block.to_string()],)?;
        Ok(())
    }

    fn height(&self, height: u64) -> Result<Option<String>> {
        let conn: Connection = Connection::open(&self.path)?;

        let mut stmt: rusqlite::Statement<'_> = conn
            .prepare("SELECT height, block FROM data WHERE height = ?1 LIMIT 1")?; // ? propagates error if raised

        match stmt.query_row(&[&height], |row| {
            let block: String = row.get(1)?;
            Ok(block)
        }) {
            Ok(b) => Ok(Some(b)),
            Err(err) => Ok(None),
        }

    }
}

impl CandidateStore for Storage {
    fn create(&self) -> Result<()> {
        let conn: Connection = Connection::open(&self.path)?;

        conn.execute("
            CREATE TABLE IF NOT EXISTS data (
            id INTEGER PRIMARY KEY,
            height INTEGER,
            blocks TEXT NOT NULL    
        )",
        [],
    )?;
    Ok(())
    }

    fn insert(&self, height: u64, block: Block) -> Result<()> {
        let candidates_serialized = match CandidateStore::height(self, height) {
            Ok(Some(data)) => Some(data),
            Ok(None) => None,
            Err(e) => return Err(e),
        };

        let mut is_first_entry: bool = false;
        let mut candidates = match candidates_serialized {
            Some(candidates) => Blockchain::from_string(candidates),
            None => {
                is_first_entry = true;
                Blockchain { blocks: Vec::new()}
            }
        };

        println!("Current Block Candidaets: {}", candidates.blocks.len());

        candidates.add_block(block);

        let conn: Connection = Connection::open(&self.path)?;

        if is_first_entry {
            conn.execute(
                "UPDATE data SET blocks = ?2 WHERE height = ?1",
                &[&height.to_string(), &candidates.to_string()],
            )?;
        }

        Ok(())
    }

    fn height(&self, height: u64) -> Result<Option<String>> {
        let conn: Connection = Connection::open(&self.path)?;

        let mut stmt = conn.prepare("
            SELECT height, blocks FROM data WHERE height = ?1 LIMIT 1
        ")?;

        match stmt.query_row(&[&height], |row| {
            let blocks: String = row.get(1)?;
            Ok(blocks)
        }) {
            Ok(b) => Ok(Some(b)),
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
            Err(e) => Err(e),
        }
    }
}


#[test]
fn test_block_store(){
    use crate::util::genesis_block;
    dotenv().ok();
    let block_db_path = env::var("DEFAULT_BLOCK_DB_PATH").expect("Failed to get Block DB Path");
    let storage = Storage {
        path: PathBuf::from(block_db_path.clone())
    };

    let _ = BlockStore::create(&storage).expect("Failed to create Block Store");
    let genesis_block = genesis_block();
    let _ = BlockStore::insert(&storage, 0, genesis_block);
    let block = BlockStore::height(&storage, 0).unwrap();
    println!("Block: {:?}", &block);
}



#[test]
fn test_candidate_store() -> Result<(), Box<dyn std::error::Error>> {
    use crate::util::{chrono_timestamp, genesis_block, create_validator_set};
    use crate::staking::validator::Validator;
    dotenv().ok();

    let candidate_db_path = env::var("DEFAULT_CANDIDATE_DB_PATH")
        .expect("Failed to get Candidate DB Path");

    let storage = Storage {
        path: PathBuf::from(candidate_db_path.clone())
    };

    CandidateStore::create(&storage).expect("Failed to create Candidate Store");

    let balances: Vec<u64> = vec![25, 50, 75, 100];

    let validators: Vec<Validator> = create_validator_set(balances.len() as u64, balances);

    let genesis_block: Block = genesis_block();

    for validator in validators {
        let new_block = Block::generate(
            genesis_block.clone(),
            hash_input(&chrono_timestamp()),
            validator
        )?;

        CandidateStore::insert(&storage, 1, new_block);

        std::thread::sleep(std::time::Duration::from_millis(1000));
    }

    let pool = CandidateStore::height(&storage, 1)?;
    println!("Current pool: {:?}", &pool);

    Ok(())
}