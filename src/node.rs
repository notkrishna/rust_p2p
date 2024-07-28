
use std::net::SocketAddr;
use std::sync::Arc;

use serde::{Deserialize, Serialize};
use uuid::Uuid;
use tokio::sync::Mutex;
use log::{
    info,
    warn,
    error
};

#[derive(Serialize, Deserialize, Debug)]
pub enum Message {
    Handshake {
        id: String
    },
    Text {
        from: String,
        content: String
    }
}

#[derive(Clone)]
pub struct Node {
    id: String,
    peers: Arc<Mutex<Vec<SocketAddr>>>,
    brodcast_addr: SocketAddr,
}

impl Node {
    pub fn new(
        brodcast_addr: SocketAddr, 
    ) -> Self {
        let id = Uuid::new_v4().to_string();
        
        Node {
            id,
            peers: Arc::new(Mutex::new(Vec::new())),
            brodcast_addr,
        }
    }

    pub async fn add_peer(&self, addr: SocketAddr) {
        let mut peers = self.peers.lock().await;
        if !peers.contains(&addr) {
            peers.push(addr);
            info!("Peer added: {}", addr);
        }
    }
    
    pub async fn get_peers(&self) -> Vec<SocketAddr> {
        let peers = self.peers.lock().await;
        peers.clone()
    }
}