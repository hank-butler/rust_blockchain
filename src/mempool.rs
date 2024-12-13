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


// pub fn get_candidate_pool(storage: &Storage, height: &u64) -> Blockchain {
//     let serialized_pool = CandidateStore::height(storage, height.clone()).unwrap().expect("Failed to get mempool");

//     Blockchain::from_string(serialized_pool)
// }

// pub trait Blockstore {
//     fn create(&self) -> Result<()>;
//     fn insert(&self, height: u64, block: Block) -> Result<()>;
//     fn height(&self, height: u64) -> Result<Option<String>>;
// }

// pub trait CandidateStore {
//     fn create(&self) -> Result<()>;
//     fn insert(&self, height: u64, block: Block) -> Result<()>;
//     fn height(&self, heihght: u64) -> Result<Option<String>>;
// }

// pub struct Storage{
//     pub path: PathBuf
// }

// impl Blockstore for Storage {
//     fn create(&self) -> Result<()> {
//         let conn: Connection = Connection::open(&self.path)?;

//         conn.execute(
//             "CREATE TABLE IF NOT EXISTS data {
//             id INTEGER PRIMARY KEY,
//             height INTEGER,
//             block TEXT NOT NULL
//             )",
//         [],
//         )?;

//         Ok(())
//     }

//     fn insert(&self, height: u64, mut block: Block) -> Result<()> {
//         let conn: Connection = Connection::open(&self.path)?;
//         conn.execute(
//             "INSERT INTO data (height, block) VALUES (?1, ?2)",
//             &[&height.to_string(), &block.to_string()],
//         )?;
//         Ok(())
//     }

//     fn height(&self, height: u64) -> Result<Option<String>> {
//         let conn: Connection = Connection::open(&self.path)?;
//         let mut statement: rusqlite::Statement<'_> = conn
//             .prepare("SELECT height, block FROM data WHERE height = ?1 LIMIT 1")?;

//         match statement.query_row(&[&height], |row| {
//             let block: String = row.get(1)?;
//             Ok(block)
//         }) {
//             Ok(b) => Ok(Some(b)),
//             Err(err) => Ok(None)
//         }
//     }

// }


// impl CandidateStore for Storage {
//     fn create(&self) -> Result<()> {
//         let conn: Connection = Connection::open(&self.path)?;
//         conn.execute(
//             "CREATE TABLE IF NOT EXISTS data (
//             id INTEGER PRIMARY KEY,
//             height INTEGER,
//             blocks TEXT NOT NULL
        
//         )",
//         [],
//         )?;

//         Ok(())
//     }

//     fn insert(&self, height: u64, block: Block) -> Result<()> {
//         let candidates_serialized: Option<String> = CandidateStore::height(self, height).expect("Failed to get parameters");
//         let mut is_first_entry: bool = bool::default();
//         let mut candidates: Blockchain = match candidates_serialized {
//             Some(candidates) => {
//                 Blockchain::from_string(candidates)
//             },
//             None => {
//                 is_first_entry = true;
//                 Blockchain{
//                     blocks: Vec::new()
//                 }
//             }
//         };

//         println!("Current Candidates: {:?}", &candidates.blocks.len());

//         candidates.add_block(block);

//         let conn: Connection = Connection::open(&self.path)?;
//         if is_first_entry {
//             conn.execute(
//                 "INSERT INTO data (height, blocks) VALUES (?1, ?2)",
//                 &[&height.to_string(), &candidates.to_string()],
//             )?;
//         } else {
//             conn.execute(
//                 "UPDATE data SET blocks = ?2 WHERE height = ?1",
//                 &[&height.to_string(), &candidates.to_string()],
//             )?;
//         }
//         Ok(())
//     }

//     fn height(&self, height: u64) -> Result<Option<String>> {
//         let conn: Connection = Connection::open(&self.path)?;
//         let mut statement: rusqlite::Statement<'_> = conn
//             .prepare("SELECT height, blocks FROM data WHERE height =?1 LIMIT 1")?;

//         match statement.query_row(&[&height], |row| {
//             let blocks: String = row.get(1)?;
//             Ok(blocks)
//         }) {
//             Ok(b) => Ok(Some(b)),
//             Err(err) => Ok(None),
//         }
//     }
// }