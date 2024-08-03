
use std::net::{SocketAddr};
use std::sync::Arc;

use serde::{Deserialize, Serialize};
use serde_json;

use uuid::Uuid;
use tokio::sync::{Mutex, mpsc};
use tokio::net::{TcpListener, TcpStream};
use tokio::io::{AsyncReadExt, AsyncWriteExt};

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
    pub id: String,
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

    pub async fn send_message(&self, msg: &Message, addr: &SocketAddr) -> Result<(), Box<dyn std::error::Error>> {
        let json = serde_json::to_string(msg)?;
        let mut stream = TcpStream::connect(addr).await?;
        stream.write_all(json.as_bytes()).await?;
        Ok(())
    }

    pub async fn broadcast_message(&self, msg: &Message) -> Result<(), Box<dyn std::error::Error>> {
        let peers = self.peers.lock().await;
        for peer in peers.iter() {
            self.send_message(msg, peer).await?;
        }
        Ok(())
    }

    pub async fn recieve_message(&self, addr:&SocketAddr, tx: mpsc::Sender<Message>) -> Result<(), Box<dyn std::error::Error>> {
        let listener = TcpListener::bind(addr).await?;
        loop {
            let (mut stream, peer_addr) = listener.accept().await?;
            let mut buff = vec![0;1024];
            let x = stream.read(&mut buff).await?;
            if let Ok(msg) = serde_json::from_slice(&buff[..x]) {

                match &msg {
                    Message::Handshake { id } => {
                        self.add_peer(peer_addr).await;
                    },
                    Message::Text { 
                        from, 
                        content 
                    } => {
                        tx.send(msg).await?;
                    }
                }
            };
    
        }
    }
}