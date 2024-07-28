use rust_p2p::node::{Node, Message};
use std::net::SocketAddr;
use tokio::sync::mpsc;
use tokio::task;

#[tokio::test]
async fn test_add_peer(){
    let broadcast_addr: SocketAddr = "127.0.0.1:6200".parse().unwrap();
    let peer_addr: SocketAddr = "127.0.0.1:6201".parse().unwrap();
    
    let node = Node::new(broadcast_addr);

    node.add_peer(peer_addr).await;

    let peers= node.get_peers().await;
    
    assert!(peers.contains(&peer_addr));
}