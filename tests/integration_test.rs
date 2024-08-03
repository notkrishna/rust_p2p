use rust_p2p::node::{Node, Message};
use std::net::{SocketAddr}; // use tokio::net::TcpListener
use tokio::net::{TcpListener, TcpStream};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::sync::mpsc;
use tokio::task;

use std::time::Duration;
use tokio::time::timeout;

#[tokio::test]
async fn test_add_peer(){
    let broadcast_addr: SocketAddr = "127.0.0.1:6200".parse().unwrap();
    let peer_addr: SocketAddr = "127.0.0.1:6201".parse().unwrap();
    
    let node = Node::new(broadcast_addr);

    node.add_peer(peer_addr).await;

    let peers= node.get_peers().await;
    
    assert!(peers.contains(&peer_addr));
}

#[tokio::test]
async fn test_send_message() {
    let broadcast_addr: SocketAddr = "127.0.0.1:6200".parse().unwrap();
    let peer_addr: SocketAddr = "127.0.0.1:6201".parse().unwrap();
    
    let node = Node::new(broadcast_addr);

    node.add_peer(peer_addr).await;

    let peers= node.get_peers().await;
    
    let listener = TcpListener::bind(peer_addr).await.unwrap();

    let node_id = node.id.clone();

    let handle_incoming_connections = tokio::spawn(
        async move {
            let (mut socket, _) = listener.accept().await.unwrap();
            let mut buff = vec![0;1024];
            let x = socket.read(&mut buff).await.unwrap();
            
            // Properly unwrap the Result or it'll bitch
            if let Ok(msg) = serde_json::from_slice(&buff[..x]){
            if let Message:: Text { from, content } = msg {
                // test condition
                assert_eq!(from, node_id); // id should be pub
                assert_eq!(content, "Message recieved!");
            }
        }
        }
    );

    let msg = Message::Text { 
        from: node.id.clone(), 
        content: "Message recieved!".to_string(), 
    };


    match node.send_message(&msg, &peer_addr).await {
        Ok(_) => println!("Message sent!"),
        Err(e) => panic!("Failed to send message: {:?}", e),
    };

    match timeout(Duration::from_secs(2), handle_incoming_connections).await {
        Ok(_) => println!("Message handling completed within time range"),
        Err(_) => panic!("Timeout while waiting for incoming messages"),
    };
}