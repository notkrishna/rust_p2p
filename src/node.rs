
use std::net::{SocketAddr};
use std::sync::Arc;
use std::collections::HashSet;

use serde::{Deserialize, Serialize};
use serde_json;

use uuid::Uuid;
use tokio::sync::{Mutex, mpsc};
use tokio::net::{TcpListener, TcpStream};
use tokio::io::{
    self,
    AsyncReadExt, 
    AsyncWriteExt,
    BufReader,
    AsyncBufReadExt,
};

use log::{
    info,
    warn,
    error
};

use rand::seq::SliceRandom;



#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub enum Message {
    Handshake {
        id: String
    },
    Text {
        from: String,
        content: String
    }
}


impl Message {
    // Implement the `print` method to print the contents of the Message
    pub fn print(&self) {
        match self {
            Message::Handshake { id } => {
                println!("Received Handshake with ID: {}", id);
            }
            Message::Text { from, content } => {
                println!("Received Message from {}: {}", from, content);
            }
        }
    }
}

#[derive(Clone)]
pub struct Node {
    pub id: String,
    // listener: Arc<Mutex<TcpListener>>,
    peers: Arc<Mutex<Vec<SocketAddr>>>,
    pub broadcast_addr: SocketAddr,
    tx: mpsc::Sender<Message>,
    rx: Arc<Mutex<mpsc::Receiver<Message>>>, // Reciever cannot be cloned directly
}

impl Node {
    pub async fn new(broadcast_addr: SocketAddr) -> Result<Self, Box<dyn std::error::Error>> {
        // Why not let new() handle creation of nodes and listening to new peers
        let id = Uuid::new_v4().to_string();

        // Bind then wrap
        // let listener = TcpListener::bind(broadcast_addr).await?;
        // let listener = Arc::new(Mutex::new(listener));

        
        let (tx, mut rx) = mpsc::channel(32);

        // let peers = Arc::new(Mutex::new(HashSet::<SocketAddr>::new()));
        // let peers_clone = peers.clone();

      
       
        Ok(Self {
            id,
            // listener,
            peers: Arc::new(Mutex::new(Vec::new())),
            broadcast_addr,
            tx,
            rx: Arc::new(Mutex::new(rx)),
        })
    }

    // Need Result<..> to debug
    pub async fn add_peer(&self, addr: SocketAddr) -> Result<(), Box<dyn std::error::Error>> {
        let mut peers = self.peers.lock().await;
        if !peers.contains(&addr) {
            peers.push(addr);
            info!("Peer added: {}", addr);
        }

        Ok(())
    }
    
    pub async fn get_peers(&self) -> Vec<SocketAddr> {
        let peers = self.peers.lock().await;
        peers.clone()
    }

    pub async fn send_message(&self, msg: &Message, addr: &SocketAddr) -> Result<(), Box<dyn std::error::Error>> {
        let json = serde_json::to_string(msg)?;
        let mut stream = TcpStream::connect(addr).await?;
        stream.write_all(json.as_bytes()).await?;
        println!("Sending message to {}", addr);
        Ok(())
    }

    pub async fn broadcast_message(&self, msg: &Message) -> Result<(), Box<dyn std::error::Error>> {
        let peers = self.peers.lock().await;
        println!("Printing peers {:?}", peers);
        for peer in peers.iter() {
            self.send_message(msg, peer).await?;
        }
        Ok(())
    }


    // Can't make mpsc work right now

    pub async fn recieve_message(&self, addr:&SocketAddr, tx: mpsc::Sender<Message>) -> Result<(), Box<dyn std::error::Error>> {
        let listener = TcpListener::bind(addr).await?;
        println!("\nListening for messages ... (node.rs)\n");
        loop {
            let (mut stream, peer_addr) = listener.accept().await?;
            println!("Accepted connection from {}", peer_addr);
            
            let mut buff = vec![0;1024];
            let x = stream.read(&mut buff).await?;
            match serde_json::from_slice::<Message>(&buff[..x]) {
                Ok(msg) => {
                    // msg.print();
                match &msg {
                    Message::Handshake { id } => {
                        println!("Handshake recieved from {}", peer_addr);
                        if let Err(e) = self.add_peer(peer_addr).await {
                            eprintln!("Error adding peer: {}", e);
                        };
                    },
                    Message::Text { 
                        from, 
                        content 
                    } => {
                        println!("Message from {}: {}", from, content);

                        tx.send(msg).await?;
                    }
                }
            },
            Err(e) => {
                eprintln!("Error deserializing message: {}", e);
            }
            };
    
        }
    }

    // Choosing gossip protocol over multicast for peer discovery
    pub async fn gossip_message(&self) -> Result<(), Box<dyn std::error::Error>> {
        println!("\nGossiping (node.rs) ...");
        let peers = self.peers.lock().await;
        let sample_size = (peers.len() as f64).ceil() as usize;
        let sample_peers = peers.choose_multiple(&mut rand::thread_rng(), sample_size);

        let msg = &Message::Handshake { id: self.id.clone() };
        println!("Handshake performed by {}...\n", self.broadcast_addr);

        for peer in sample_peers {
            self.send_message(&msg, peer).await?;
        }

        Ok(())
    }

    pub async fn handle_user_input(&self) -> Result<(), Box<dyn std::error::Error>> {
        
        let stdin = io::stdin();
        let mut reader = BufReader::new(stdin).lines();

        println!("\nStart writing your message ...\n");
        while let Some(line) = reader.next_line().await? {
            let msg: Message = Message::Text { 
                from: self.id.clone(), 
                content: line, 
            };
            if let Err(e) = self.broadcast_message(&msg).await{
                eprintln!("Error broadcasting message: {}", e);
            };
        }
        // loop {
        //     tokio::time::sleep(tokio::time::Duration::from_secs(60)).await;
        // }

        Ok(())
    }
    

}