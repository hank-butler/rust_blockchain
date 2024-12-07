use rocket::time::format_description::modifier::UnixTimestamp;
use rusqlite;
use rusqlite::{Connection, Result};
use std::path::PathBuf;
use dotenv::dotenv;
use std::env;
use crate::util::hash_input;
use crate::blockchain::block::Block;
use crate::blockchain::blockchain::Blockchain;

pub trait Blockstore {
    fn create(&self) -> Result<()>;
    fn insert(&self, height: u64, block: Block) -> Result<()>;
    fn height(&self, height: u64) -> Result<Option<String>>;
}

pub trait CandidateStore {
    fn create(&self) -> Result<()>;
    fn insert(&self, height: u64, block: Block) -> Result<()>;
    fn height(&self, heihght: u64) -> Result<Option<String>>;
}

pub struct Storage{
    pub path: PathBuf
}

impl Blockstore for Storage {
    fn create(&self) -> Result<()> {
        let conn: Connection = Connection::open(&self.path)?;

        conn.execute(
            "CREATE TABLE IF NOT EXISTS data {
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
        conn.execute(
            "INSERT INTO data (height, block) VALUES (?1, ?2)",
            &[&height.to_string(), &block.to_string()],
        )?;
        Ok(())
    }

    fn height(&self, height: u64) -> Result<Option<String>> {
        let conn: Connection = Connection::open(&self.path)?;
        let mut statement: rusqlite::Statement<'_> = conn
            .prepare("SELECT height, block FROM data WHERE height = ?1 LIMIT 1")?;

        match statement.query_row(&[&height], |row| {
            let block: String = row.get(1)?;
            Ok(block)
        }) {
            Ok(b) => Ok(Some(b)),
            Err(err) => Ok(None)
        }
    }

}


