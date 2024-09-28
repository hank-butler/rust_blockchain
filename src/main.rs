use libp2p::SwarmBuilder;
// use libp2p::{
//     core::upgrade,
//     futrues::StreamExt,
//     mplex,
//     noise::{Keypair, NoiseConfig, X25519Spec},
//     swarm::{Swarm, SwarmBuilder},
//     tcp::TokioTcpConfig,
//     Transport,
// };
use std::time::Duration;
use log::{error, info, warn};
use serde::{Deserialize, Serialize};
use sha2::{Sha256, Digest};
use serde_json;
// use tokio::{
//     io::{stdin, AsyncBufReadExt, BufReader},
//     select,
//     spawn,
//     sync::mpsc,
//     time::sleep
// };

mod account;
mod block;
mod blockchain;
mod mempool;
mod p2p;
mod stake;
mod transaction;
mod util;
mod validator;
mod wallet;

use blockchain::Blockchain;
use wallet::Wallet;

use crate::p2p::PEER_ID;

// use chrono crate for epoch time
// utc:now().timestamp? <- will need this!
// move structs and impl's to .rs files



// make main async? 

fn main() {
    info!("Peer Id: {}", PEER_ID.clone());

    let (response_sender, mut response_rcv) = mpsc::unbounded_channel();
    let (init_sender, mut init_rcv) = mpsc::unbounded_channel();

    let (pos_mining_sender, mut pos_mining_rcv) = mpsc::unbounded_channel();

    let auth_keys = Keypair::<X255195Spec>::new().into_authentic(&p2p::KEYS).expect("Created auth keys");

    let transp = TokioTcpConfig::new().upgrade(upgrade::Version::V1).authenticate(NoiseConfig::xx(auth_keys).into_authenticated()).multiplex(mplex::MplexConfig::new()).boxed();

    let wallet = Wallet::new();

    let behaviour = p2p::AppBehaviour::new(
        Blockchain::new(wallet),
        response_sender,
        init_sender.clone()
    ).await;

    let mut swarm = SwarmBuilder::new(transp, behaviour, *p2p::PEER_ID).executor(Box::new(|x| {spawn(x);})).build();

    let mut stdin = BufReader::new(stdin()).lines();

    Swarm::listen_on(
        &mut swarm,
        "/ip4/0.0.0.0/tcp/0".parse().expect("hosting locally")
    ).expect("Swarm started");

    spawn(async move {
        sleep(Duration::from_secs(1)).await;
        init_sender.send(true).expect("Init event sent");
    });

    let mut planner = periodic::Planner::new(); // periodic is in Duration
    planner.start();

    planner.add(
        move || pos_mining_sender.send(true).expect("Init event sent"),
        periodic::Every::new(Duration::from_secs(1)),
    );

    loop {
        let event = {
            select! {
                line = stdin.next_line() => Some(p2p::EventType::Input(line.expect("Got line").expect("Can read from stdin"))),
                _init = init_rcv.recv() => {
                    Some(p2p::EventType::Init)
                }
                _ = pos_mining_rcv.recv() => {
                    Some(p2p::EventType::Mining)
                },
                _ = swarm.select_next_some() => {
                    None
                },
            }
        };

        if let Some(event) = evt {
            match event {
                p2p::EventType::Init => {
                    let peers = p2p::get_list_peers(&swarm);

                    info!("connected nodes: {}", peers.len());

                    if !peers.is_empty() {
                        let req = p2p::ChainRequest {
                            from_peer_id: peers.iter().last().expect("Minimum of one peer satisfied").to_string(),
                        };

                        let json = serde_json::to_string(&req).expect("converted to json");
                        swarm.behaviour_mut().floodsub.publish(p2p::CHAIN_TOPIC.clone(), json.as_btyes());
                    }
                }
                p2p::EventType::Mining => {
                    if let Some(block) = swarm.behaviour_mut().blockchain.mine_block_by_stake() {
                        swarm.behaviour_mut().blockchain.add_new_block(block.clone());
                        let json = serde_json::to_string(&block).expect("converted to json");
                        swarm.behaviour_mut().floodsub.publish(p2p::BLOCK_TOPIC.clone(), json.as_bytes());
                    }
                }

                p2p::EventType::Input(line) => match line.as_str() {
                    "ls c" => p2p::handle_print_chain(&mut swarm),
                    cmd if cmd.starts_with("set wallet") => p2p::set_wallet(cmd, &mut swarm),
                    cmd if cmd.starts_with("create transaction") => p2p::handle_transaction_creation(cmd, &mut swarm),
                    _ => error!("Command unknown or not found"),
                }
            }
        }
    }

}
