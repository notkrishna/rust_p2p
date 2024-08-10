
// Crate because node is a sibling module
use crate::node::Node;
use crate::node::Message;

use std::time::Duration;

pub async fn gossip_at_interval(node: Node) -> Result<(), Box<dyn std::error::Error>> {

    loop {
        let msg = Message::Handshake { 
            id: node.id.clone(), 
        };
    node.gossip_message(&msg).await?;

    tokio::time::sleep(Duration::from_secs(5)).await;
    }
}