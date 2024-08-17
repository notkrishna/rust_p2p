
use log::info;
use rust_p2p::gossip;
use rust_p2p::gossip::gossip_at_interval;
use rust_p2p::node::Node;
use rust_p2p::node::Message;

use std::env;
use std::net::SocketAddr;
use tokio::sync::mpsc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    let args: Vec<String> = env::args().collect();

    let broadcast_addr: SocketAddr = match args.get(1) {
        Some(addr) => addr.parse::<SocketAddr>()?,
        None => "127.0.0.1:8080".parse()?,
    };

    let node = Node::new(broadcast_addr).await?;
    print!("\nNode created with id {} and address {}\n", node.id, node.broadcast_addr);

    let peer_addr: SocketAddr = "127.0.0.1:8081".parse()?;
    let peer_addr2: SocketAddr = "127.0.0.1:8082".parse()?;
    
    if let Err(e) = node.add_peer(peer_addr).await {
        eprintln!("Error adding peer: {}", e);
    }

    if let Err(e) = node.add_peer(peer_addr2).await {
        eprintln!("Error adding peer: {}", e);
    }
    // Will look at mpsc later
    let (tx, mut rx) = mpsc::channel::<Message>(32);


    let node_clone = node.clone();
    tokio::spawn(async move {
        println!("Gossiping ...");
        if let Err(e) = gossip_at_interval(node_clone).await {
            eprintln!("Error while gossiping: {}", e);
        }
    });

    let node_clone = node.clone();
    tokio::spawn(
        async move {
            println!("Listening for messages ...");
            if let Err(e) = node_clone.recieve_message(&node_clone.broadcast_addr, tx).await {
                eprintln!("Error occured while recieving messages: {}", e);
            }

        }
    );

    let node_clone = node.clone();
    tokio::spawn(
        async move {
            if let Err(e) = node_clone.handle_user_input().await {
                eprintln!("Error occured handling user input: {}", e);
            }
        }
    );

    let node_clone = node.clone();
    tokio::spawn(async move {
        println!("Gossiping ...");
        if let Err(e) = gossip_at_interval(node_clone).await {
            eprintln!("Error while gossiping: {}", e);
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
