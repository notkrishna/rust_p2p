use rust_p2p::node::{Node, Message};
use rust_p2p::gossip::gossip_at_interval;
use std::net::{SocketAddr}; // use tokio::net::TcpListener
use tokio::net::{TcpListener, TcpStream};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::sync::{broadcast, mpsc};
use tokio::task;

use std::time::Duration;
use tokio::time::timeout;

use std::sync::Arc;

// TO DO:
// use mocks

#[tokio::test]
async fn test_gossip_messages() {
    let broadcast_addr = "127.0.0.1:6206".parse().unwrap();
    let peer_addr = "127.0.0.1:6207".parse().unwrap();

    let mut node = Node::new(broadcast_addr)
        .await
        .expect("failed to create a new node");

    node.add_peer(peer_addr).await.expect("Failed to add peer");

    let msg = Message::Text {
        from: node.id.clone(),
        content: "Testing peer to peer".to_string(),
    };

    //dealing with borrowing of moved value: msg with Arc
    let msg_clone = Arc::new(msg);


    let listener = TcpListener::bind(peer_addr).await.expect("Failed to bind to peer address");

    let msg_clone_for_test = Arc::clone(&msg_clone);

    let handle = tokio::spawn(
        async move {
            let (mut socket, _) = listener.accept().await.expect("Failed to accept connection");
            let mut buff = vec![0;1024];
            let n = socket.read(&mut buff).await.expect("Failed to read buffer");
            let msg_recieved: Message = serde_json::from_slice(&buff[..n]).expect("Message parsing failed");
            assert_eq!(msg_recieved, *msg_clone_for_test);
        }
    );

    node.gossip_message()
    .await
    .expect("Failed to broadcast message");



}

