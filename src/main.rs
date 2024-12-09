mod blockchain;
mod staking;
mod util;
mod mempool;

use util::{chrono_timestamp, default_validator_set, genesis_block, get_block_with_height, initialize_blockstore_with_genesis, initialize_candidatestore, purge_dbs};
use dotenv::dotenv;
use mempool::{Storage, Blockstore, CandidateStore, get_candidate_pool};
use std::{env, path::PathBuf};
use blockchain::{block::Block, blockchain::Blockchain};
use staking::validator::{Validator, Vote, get_validator_weight};
use util::{hash_input};




fn main() {
    dotenv().ok(); // checking for .env
    // set up env variables here
    let block_db_path: String = env::var("DEFAULT_BLOCK_DB_PATH").expect("Failed to get path");
    let candidate_db_path: String = env::var("DEFAULT_CANDIDATE_DB_PATH").expect("Failed to get path");

    purge_dbs(PathBuf::from(&block_db_path), PathBuf::from(&candidate_db_path));

    // let blocktime: u64 = env::var(DEFAULT_BLOCK_TIME).expect("Failed to get block time");
    let blocktime = 60;



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

    let mut participants: Vec<&Validator> = Vec::new();

    // loop through block proposals and validates new blocks every minute

    loop {
        let previous_block: Block = get_block_with_height(&block_storage, &(height-1));

        if &chrono_timestamp().parse::<u64>().unwrap() > &(previous_block.timestamp.parse::<u64>().unwrap() + blocktime){

            // validator proposes new blocks
            // loop through each validator in the validators vector
            for validator in &validators {
                if participants.contains(&validator){
                    continue;
                }
                // generate a new block with random bpm, placeholder for payload/data
                let block: Block = Block::generate(previous_block.clone(), hash_input(&chrono_timestamp()), validator.clone());

                // put block in mempool
                let _ = CandidateStore::insert(&candidate_storage, height, block);
                participants.push(&validator);
            };

            // Throws error if mempool is empty
            let pool: Blockchain = get_candidate_pool(&candidate_storage, &height);

            if pool.blocks.len() > 0 {
                let mut votes: Vec<Vote> = Vec::new();
                let mut total_votes: u64 = 0;
                let mut round_weights: Vec<(Block, u128)> = Vec::new();

                for block in pool.blocks{
                    votes.push(Vote { block: block.clone(), stake: block.clone().validator.stake });
                    total_votes += &block.validator.stake;
                };

                for vote in &votes {
                    round_weights.push((vote.block.clone(), get_validator_weight(vote.block.validator.stake, total_votes)));


                }

                let lotto_winner: Block = {
                    round_weights.iter().max_by_key(|&(_block, value)| value).unwrap().0.clone()
                };

                let _ = Blockstore::insert(&block_storage, height, lotto_winner);
            
                height += 1;
                participants = Vec::new();
            }


        }
    }

}






// mod accounts;
// mod blockchain;
// mod staking;


// use std::time::Duration;
// use log::{error, info, warn};
// use serde::{Deserialize, Serialize};
// use sha2::{Sha256, Digest};
// use serde_json;
// use accounts::account::Account;
// use accounts::wallet::Wallet;
// use blockchain::block::Block;
// use blockchain::transaction::Transaction;
// use staking::stake::Stake;


// use blockchain::Blockchain;
// use wallet::Wallet;


// use chrono crate for epoch time
// utc:now().timestamp? <- will need this!
// move structs and impl's to .rs files



// // make main async? 

// fn main() {
//     println!("woohoo restarting!");
// }

// async fn main() {
//     info!("Peer Id: {}", PEER_ID.clone());

//     let (response_sender, mut response_rcv) = mpsc::unbounded_channel();
//     let (init_sender, mut init_rcv) = mpsc::unbounded_channel();

//     let (pos_mining_sender, mut pos_mining_rcv) = mpsc::unbounded_channel();

//     let auth_keys = Keypair::<X255195Spec>::new().into_authentic(&p2p::KEYS).expect("Created auth keys");

//     let transp = TokioTcpConfig::new().upgrade(upgrade::Version::V1).authenticate(NoiseConfig::xx(auth_keys).into_authenticated()).multiplex(mplex::MplexConfig::new()).boxed();

//     let wallet = Wallet::new();

//     let behaviour = p2p::AppBehaviour::new(
//         Blockchain::new(wallet),
//         response_sender,
//         init_sender.clone()
//     ).await;

//     let mut swarm = SwarmBuilder::new(transp, behaviour, *p2p::PEER_ID).executor(Box::new(|x| {spawn(x);})).build();

//     let mut stdin = BufReader::new(stdin()).lines();

//     Swarm::listen_on(
//         &mut swarm,
//         "/ip4/0.0.0.0/tcp/0".parse().expect("hosting locally")
//     ).expect("Swarm started");

//     spawn(async move {
//         sleep(Duration::from_secs(1)).await;
//         init_sender.send(true).expect("Init event sent");
//     });

//     let mut planner = periodic::Planner::new(); // periodic is in Duration
//     planner.start();

//     planner.add(
//         move || pos_mining_sender.send(true).expect("Init event sent"),
//         periodic::Every::new(Duration::from_secs(1)),
//     );

//     loop {
//         let event = {
//             select! {
//                 line = stdin.next_line() => Some(p2p::EventType::Input(line.expect("Got line").expect("Can read from stdin"))),
//                 _init = init_rcv.recv() => {
//                     Some(p2p::EventType::Init)
//                 }
//                 _ = pos_mining_rcv.recv() => {
//                     Some(p2p::EventType::Mining)
//                 },
//                 _ = swarm.select_next_some() => {
//                     None
//                 },
//             }
//         };

//         if let Some(event) = evt {
//             match event {
//                 p2p::EventType::Init => {
//                     let peers = p2p::get_list_peers(&swarm);

//                     info!("connected nodes: {}", peers.len());

//                     if !peers.is_empty() {
//                         let req = p2p::ChainRequest {
//                             from_peer_id: peers.iter().last().expect("Minimum of one peer satisfied").to_string(),
//                         };

//                         let json = serde_json::to_string(&req).expect("converted to json");
//                         swarm.behaviour_mut().floodsub.publish(p2p::CHAIN_TOPIC.clone(), json.as_btyes());
//                     }
//                 }
//                 p2p::EventType::Mining => {
//                     if let Some(block) = swarm.behaviour_mut().blockchain.mine_block_by_stake() {
//                         swarm.behaviour_mut().blockchain.add_new_block(block.clone());
//                         let json = serde_json::to_string(&block).expect("converted to json");
//                         swarm.behaviour_mut().floodsub.publish(p2p::BLOCK_TOPIC.clone(), json.as_bytes());
//                     }
//                 }

//                 // p2p::EventType::Input(line) => match line.as_str() {
//                 //     "ls c" => p2p::handle_print_chain(&mut swarm),
//                 //     cmd if cmd.starts_with("set wallet") => p2p::set_wallet(cmd, &mut swarm),
//                 //     cmd if cmd.starts_with("create transaction") => p2p::handle_transaction_creation(cmd, &mut swarm),
//                 //     _ => error!("Command unknown or not found"),
//                 // }
//             }
//         }
//     }

// }
