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
                block TEXT NOT NULL
                )",
        [],
    )?;
    Ok(())
    }

    fn insert(&self, height: u64, block: Block) -> Result<()> {

        let candidates_serialized: Option<String> = CandidateStore::height(self, height).expect("Failed to get mempool from Candidate Store");

        let mut is_first_entry: bool = bool::default();
        let mut candidates: Blockchain = match candidates_serialized {
            Some(candidates) => {
                Blockchain::from_string(candidates)
            },
            None => {
                is_first_entry = true;
                Blockchain{
                    blocks: Vec::new()
                }
            }
        };

        println!("Current Block Candidates: {:?}", &candidates.blocks.len());

        candidates.add_block(block);

        let conn: Connection = Connection::open(&self.path)?;

        if is_first_entry {
            conn.execute (
                "INSERT INTO data (height, blocks) VALUES (?1, ?2)",
                &[&height.to_string(), &candidates.to_string()],
            )?;
        } else {
            conn.execut(
                "UPDATE data SET blocks = ?2 WHERE height = ?1",
                &[&height.to_string(), &candidates.to_string()]
            )?;
        }
        Ok(())

    }
}