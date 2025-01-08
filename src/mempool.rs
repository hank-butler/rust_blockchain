use rocket::time::format_description::modifier::UnixTimestamp;
use rusqlite;
use rusqlite::{Connection, Result, OptionalExtension};
use std::path::PathBuf;
use dotenv::dotenv;
use std::env;
use crate::util::hash_input;
use crate::blockchain::block::Block;
use crate::blockchain::blockchain::Blockchain;
use log::{info, error};

pub struct Storage {
    pub path: PathBuf,
}

impl Storage {
    pub fn new() -> Self {
        dotenv().ok();
        let path = env::var("DATABASE_PATH").unwrap_or_else(|_| "blockchain.db".to_string());

        Self {
            path: PathBuf::from(path),
        }
    }

    pub fn initialize(&self) -> Result<()> {
        let conn: Connection = Connection::open(&self.path)?;

        conn.execute("
        CREATE TABLE IF NOT EXISTS data (
        id integer PRIMARY KEY,
        height INTEGER NOT NULL,
        block TEXT NOT NULL
        )",
[],
    )?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS candidates (
        id INTEGER PRIMARY KEY,
        height INTEGER NOT NULL,
        blocks TEXT NOT NULL
    )",
    [],
    )?;

    info!("Database tables created successfully.");
    Ok(())
    
    
    }
}


pub trait Blockstore {
    fn insert(&self, height: u64, block: Block) -> Result<()>;
    fn height(&self, height: u64) -> Result<Option<String>>;
}

pub trait CandidateStore {
    fn insert(&self, height: u64, block: Block) -> Result<()>;
    fn height(&self, height: u64) -> Result<Option<String>>;
}

impl Blockstore for Storage {
    fn insert(&self, height: u64, block: Block) -> Result<()> {
        let conn: Connection = Connection::open(&self.path)?;
        conn.execute(
            "INSERT INTO data (height, block) VALUES (?1, ?2)",
            &[&heigh.to_string(), &block.to_string()],
        )?;

        info!("Block inserted at height  {}", height);
        Ok(())
    }

    fn height(&self, height: u64) -> Result<Option<String>> {
        let conn: Connection = Connection::open(&self.path)?;
        let mut statement = conn.prepare(
            "SELECT block FROM data WHERE height = ?1 LIMIT 1")?;
        statement.query_row([&height], |row| row.get(0)).optional()
    }
}

impl CandidateStore for Storage {
    fn insert(&self, height: u64, block: Block) -> Result<()> {
        let serialized_candidates = self.height(height)?;
        let mut candidates = match serialized_candidates {
            Some(data) => Blockchain::from_string(data),
            None => Blockchain {blocks: Vec::new()},
        };

        candidates.add_block(block);

        let conn: Connection = Connection::open(&self.path)?;

        if serialized_candidates.is_none() {
            conn.execute(
                "INSERT INTO candidates (height, blocks) VALUES (?1, ?2)",
                &[&height.to_string(), &candidates.to_string()],
            )?;
        } else {
            conn.execute(
                "UPDATE candidates SET blocks = ?2 WHERE height = ?1", 
                &[&height.to_string(), &candidates.to_string()],
            )?;
        }
        info!("Candidate block inserted/updated at height {}", height);
        Ok(())

    }

    fn height() -> Result<Option<String>> {
        
    }
}

// libp2p integration on ChatGPT