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

