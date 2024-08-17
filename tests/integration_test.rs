use rust_p2p::node::{Node, Message};
use std::net::{SocketAddr}; // use tokio::net::TcpListener
use tokio::net::{TcpListener, TcpStream};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::sync::{broadcast, mpsc};
use tokio::task;

use std::time::Duration;
use tokio::time::timeout;

// TODO:
// Implement mpsc channel for send and recieve message

#[tokio::test]
async fn test_add_peer(){
    let broadcast_addr: SocketAddr = "127.0.0.1:6200".parse().unwrap();
    let peer_addr: SocketAddr = "127.0.0.1:6201".parse().unwrap();
    
    let mut node = Node::new(broadcast_addr)
    .await
    .expect("Failed to create a node");

    node.add_peer(peer_addr).await;

    let peers= node.get_peers().await;
    
    assert!(peers.contains(&peer_addr));
}

#[tokio::test]
async fn test_send_message() {
    let broadcast_addr: SocketAddr = "127.0.0.1:6202".parse().unwrap();
    let peer_addr: SocketAddr = "127.0.0.1:6203".parse().unwrap();
    
    let mut node = Node::new(broadcast_addr)
        .await
        .expect("Failed to create a node");

    node.add_peer(peer_addr).await;
    
    let peers = node.get_peers().await;
    
    println!("Peers: {:?}", peers);

    
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


#[tokio::test]
async fn test_recieve_message() {
    let broadcast_addr = "127.0.0.1:6204".parse().unwrap();
    let peer_addr = "127.0.0.1:6205".parse().unwrap();

    let (tx, mut rx) = mpsc::channel::<Message>(32);

    let new_node = Node::new(broadcast_addr).await.expect("Failed to create a new node");
    println!("Created a new node");

    if let Err(e) = new_node.add_peer(peer_addr).await {
        eprintln!("Error adding peer");
    }

    let peers = new_node.get_peers().await;
    println!("Peers: {:?}", peers);

    let new_node_id = new_node.id.clone();

    let handle_message_task = tokio::spawn(async move {
        println!("Waiting for messages ...");
        let new_node_id = new_node.id.clone();
        if let Some(msg) = rx.recv().await {
            println!("A message recieved");
            match msg {
                Message::Text { from, content } => {
                    println!("Message recieved from {}: {}", from, content);
                    assert_eq!(from, new_node_id);
                    assert_eq!(content, "Test message");
                }

                Message::Handshake { id } => {
                    println!("Shaking hands");
                    assert_eq!(id, new_node_id);
                }
                
            }
        }

    });


    let handshake_msg = Message::Handshake { id: new_node_id.clone() };
    tx.send(handshake_msg).await.expect("Failed to send handshake message");

    let text_msg = Message::Text { from: new_node_id.clone(), content: "Test message".to_string() };
    tx.send(text_msg).await.expect("Error sending message");

    println!("Message sent!");
    match timeout(Duration::from_secs(5), handle_message_task).await {
        Ok(_) => println!("Test passed"),
        Err(_) => eprintln!("Test failed")
    }


}

// This is a valid test too
// #[tokio::test]
// async fn test_recieve_message() {

//     let broadcast_addr: SocketAddr = "127.0.0.1:6204".parse().unwrap();
//     let peer_addr: SocketAddr = "127.0.0.1:6205".parse().unwrap();
    
//     let node = Node::new(broadcast_addr)
//         .await
//         .expect("Failed to create a node");
//     println!("Node created");

//     if let Err(e) = node.add_peer(peer_addr).await {
//         eprintln!("Failed to add peer: {}", e);
//     }
// //   node.add_peer(peer_addr).await;

//     let peers= node.get_peers().await;
    
//     println!("Peers: {:?}", peers);
//     let listener = TcpListener::bind(peer_addr).await.unwrap();

//     let node_id = node.id.clone();

//     let handle_incoming_connections = tokio::spawn(async move {
//         let (mut socket, _) = listener.accept().await.unwrap();
//         let mut buff = vec![0;1024];
//         let x = socket.read(&mut buff).await.unwrap();

//         let msg_recieved: Result<Message, _> = serde_json::from_slice(&buff[..x]);
//         match msg_recieved {
//             Ok(Message::Text {from, content}) => {
//                 //test condition
//                 println!("Message recieved (Test): {} from peer", content);
//                 assert_eq!(from, node_id);
//                 assert_eq!(content, "Message recieved!");
//             }
//             _ => panic!("Error deserializing message :("),
//         }
//     });

//     //slow bastard needs 5 seconds to start
//     tokio::time::sleep(Duration::from_secs(5)).await;


//     let msg = Message::Text { 
//         from: node.id.clone(), 
//         content: "Message recieved!".to_string(), 
//     };

//     let mut stream = TcpStream::connect(peer_addr).await.unwrap();
//     let json = serde_json::to_string(&msg).unwrap();
//     stream.write_all(json.as_bytes()).await.unwrap();

//     match timeout(Duration::from_secs(5), handle_incoming_connections).await {
//         Ok(_) => println!("Successful!"),
//         Err(_) => panic!("Timeout while waiting for incoming messages"),
//     }

// }
