
use log::{
    info,
    error
};
use log::LevelFilter;
use rust_p2p::gossip;
use rust_p2p::gossip::gossip_at_interval;
use rust_p2p::node::Node;
use rust_p2p::node::Message;
use simplelog::*;

use std::env;
use std::fs;
use std::net::SocketAddr;
use tokio::sync::mpsc;

use serde::Deserialize;

#[derive(Deserialize)]
struct Config {
    broadcast: BroadcastConfig,
    peers: PeersConfig,
}

#[derive(Deserialize)]
struct BroadcastConfig {
    address: String,
}

#[derive(Deserialize)]
struct PeersConfig {
    addresses: Vec<String>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // env_logger::init();

    let log_file = fs::File::create("app.log").expect("Failed to create log file");

    CombinedLogger::init(
        vec![
        WriteLogger::new(LevelFilter::Info, Default::default(), log_file),
        ]
    ).expect("Failed to initialize logger");

    info!("Starting Rust P2P");

    let config_content = fs::read_to_string("config.toml")?;
    let config: Config = toml::from_str(&config_content)?;

    let broadcast_addr = config.broadcast.address.parse()?;
    
    let node = Node::new(broadcast_addr).await?;


    // let args: Vec<String> = env::args().collect();

    // let broadcast_addr: SocketAddr = match args.get(1) {
    //     Some(addr) => addr.parse::<SocketAddr>()?,
    //     None => "127.0.0.1:8080".parse()?,
    // };

    // let node = Node::new(broadcast_addr).await?;
    info!("\nNode created with id {} and address {}\n", node.id, node.broadcast_addr);

    for peer in config.peers.addresses {
        let peer_addr: SocketAddr = peer.parse()?;
        if let Err(e) = node.add_peer(peer_addr).await {
            error!("Error adding peer: {}", e);
        }
    }

    // let peer_addr: SocketAddr = "127.0.0.1:8081".parse()?;
    // let peer_addr2: SocketAddr = "127.0.0.1:8082".parse()?;
    
    // if let Err(e) = node.add_peer(peer_addr).await {
    //     error!("Error adding peer: {}", e);
    // }

    // if let Err(e) = node.add_peer(peer_addr2).await {
    //     error!("Error adding peer: {}", e);
    // }
    // Will look at mpsc later
    let (tx, mut rx) = mpsc::channel::<Message>(32);


    let node_clone = node.clone();
    tokio::spawn(async move {
        info!("Gossiping ...");
        if let Err(e) = gossip_at_interval(node_clone).await {
            error!("Error while gossiping: {}", e);
        }
    });

    let node_clone = node.clone();
    tokio::spawn(
        async move {
            info!("Listening for messages ...");
            if let Err(e) = node_clone.recieve_message(&node_clone.broadcast_addr, tx).await {
                error!("Error occured while recieving messages: {}", e);
            }

        }
    );

    let node_clone = node.clone();
    tokio::spawn(
        async move {
            if let Err(e) = node_clone.handle_user_input().await {
                error!("Error occured handling user input: {}", e);
            }
        }
    );

    let node_clone = node.clone();
    tokio::spawn(async move {
        info!("Gossiping ...");
        if let Err(e) = gossip_at_interval(node_clone).await {
            error!("Error while gossiping: {}", e);
        }
    });

    while let Some(msg) = rx.recv().await {
        info!("Running recv loop ...");
        match msg {
            Message::Text { 
                from, 
                content 
            } => {
                info!("Recieved message from {}:{}", from, content);
            }
            _=>(),
        }
    }

    Ok(())
}
