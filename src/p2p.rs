use crate::{
    block::Block,
    blockchain::Blockchain,
    transaction,
    transaction::Transaction,
    wallet::Wallet,
};

use libp2p::{
    floodsub::{Floodsub, FloodsubEvent, Topic},
    identity,
    mdns::{Mdns, MdnsEvent},
    swarm::{NetworkBehaviourEventProcess, Swarm},
    NetworkBehaviour,
    PeerId
};

use log::{info, warn};
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use tokio::sync::mpsc;

pub static KEYS: Lazy<identity::Keypair> = Lazy::new(identity::Keypair::generate_ed25519);
pub static PEER_ID: Lazy<PeerId> = Lazy::new(|| PeerId::from(KEYS.public()));
pub static CHAIN_TOPIC: Lazy<Topic> = Lazy::new(|| Topic::new("chains"));
pub static BLOCK_TOPIC: Lazy<Topic> = Lazy::new(|| Topic::new("blocks"));
pub static TRANSACTION_TOPIC: Lazy<Topic> = Lazy::new(|| Topic::new("transactions"));

pub struct ChainResponse {
    pub blocks: Vec<Block>,
    pub transactions: Vec<Transaction>,
    pub receiver: String,
}

pub struct ChainRequest {
    pub from_peer_id: String,
}

pub enum EventType {
    Input(String),
    Init,
    Mining
}

pub struct AppBehaviour {
    pub floodsub: Floodsub,
    pub mdns: Mdns,
    pub response_sender: mpsc::UnboundedSender<ChainResponse>,
    pub init_sender: mpsc::UnboundedSender<bool>,
    pub blockchain: Blockchain,
}

impl AppBehaviour {
    pub async fn new(
        blockchain: Blockchain,
        response_sender: mpsc::UnboundedSender<ChainResponse>,
        init_sender: mpsc::UnboundedSender<bool>,
    ) -> Self {
        let mut behaviour = Self {
            blockchain,
            floodsub: Floodsub::new(*PEER_ID),
            mdns: Mdns::new(Default::default()).await.expect("Can create mdns"),
            response_sender,
            init_sender
        };
        behaviour.floodsub.subscribe(CHAIN_TOPIC.clone());
        behaviour.floodsub.subscribe(BLOCK_TOPIC.clone());
        behaviour.floodsub.subscribe(TRANSACTION_TOPIC.clone());

        behaviour
    }
}

impl NetworkBehaviourEventProcess<FloodsubEvent> for AppBehaviour {
    fn inject_event(&mut self, event: FloodsubEvent) {
        if let FloodsubEvent::Message(msg) = event {
            if let Ok(resp) = serde_json::from_slice::<ChainResponse>(&msg.data) {
                if resp.receiver == PEER_ID.to_string() {
                    info!("Response from: {}", msg.source);

                    self.blockchain.replace_chain(&resp.blocks);
                    self.blockchain.mempool.transactions = resp.transactions.iter_into().filter(|x| Transaction::verify_transaction(x).is_ok()).collect();
                }
            } else if let Ok(resp) = serde_json::from_slice::<ChainRequest>(&msg.data) {
                info!("Sending local chain and mempool to: {}", msg.source.to_string());
                let peer_id = resp.from_peer_id;

                if PEER_ID.to_string() == peer_id {
                    let json = serde_json::to_string(&ChainResponse {
                        blocks: self.blockchain.chain.clone(),
                        transactions: self.blockchain.mempool.transactions.clone(),
                        receiver: msg.source.to_string(),
                    }).expect("Response converted to json");

                    self.floodsub.publish(CHAIN_TOPIC.clone(), json.as_bytes());
                }
            } else if let Ok(block) = serde_json::from_slice::<Block>(&msg.data) {
                info!("received new block {}", block);
                if self.blockchain.chain.last().unwrap().id < block.id && self.blockchain.is_valid_block(block.clone()) {
                    info!("Relaying new block");
                    let json = serde_json::to_string(&block).expect("Response converted to json");
                    self.floodsub.publish(BLOCK_TOPIC.clone(), json.as_bytes());
                }
            } else if let Ok(transaction) = serde_json::from_slice::<Transaction>(&msg.data) {
                info!("Received new transaction from: {}", msg.source.to_string());

                if !self.blockchain.tran_check(&transaction) && Transaction::verify_transaction(&transaction).is_ok() {
                    info!("Relaying new valid transaction");
                    let json = serde_json::to_string(&transaction).expect("Response converted to json");
                    self.floodsub.publich(TRANSACTION_TOPIC.clone(), json.as_bytes());
                    self.blockchain.add_tran(transaction);
                }
            }
        }
    }
}

impl NetworkBehaviourEventProcess<MdnsEvent> for AppBehaviour {
    fn inject_event(&mut self, event: MdnsEvent) {
        match event {
            MdnsEvent::Discovered(discovered_list) => {
                for (peer, addr) in discovered_list {
                    self.floodsub.add_note_to_partial_view(peer);
                }
            }
            MdnsEvent::Expired(expired_list) => {
                for (peer, addr) in expired_list {
                    if !self.mdns.has_node(&peer) {
                        self.floodsub.remove_note_from_partial_view(&peer);
                    }
                }
            }
        }
    }
}

