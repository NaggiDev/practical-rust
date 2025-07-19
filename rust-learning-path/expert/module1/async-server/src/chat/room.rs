use std::collections::HashSet;
use tokio::sync::{broadcast, RwLock};
use tracing::{info, warn};
use anyhow::Result;

pub struct ChatRoom {
    name: String,
    max_clients: usize,
    clients: RwLock<HashSet<String>>,
    broadcaster: broadcast::Sender<String>,
}

impl ChatRoom {
    pub fn new(name: String, max_clients: usize) -> Self {
        let (broadcaster, _) = broadcast::channel(1000); // Buffer up to 1000 messages
        
        Self {
            name,
            max_clients,
            clients: RwLock::new(HashSet::new()),
            broadcaster,
        }
    }

    pub async fn join_client(&self, client_id: String) -> Result<broadcast::Receiver<String>> {
        let mut clients = self.clients.write().await;
        
        if clients.len() >= self.max_clients {
            return Err(anyhow::anyhow!("Room is full"));
        }

        if clients.contains(&client_id) {
            return Err(anyhow::anyhow!("Client already in room"));
        }

        clients.insert(client_id.clone());
        let receiver = self.broadcaster.subscribe();
        
        info!("Client {} joined room '{}' ({} clients)", client_id, self.name, clients.len());

        // Broadcast join notification
        let join_msg = serde_json::json!({
            "type": "user_joined",
            "client_id": client_id,
            "room": self.name,
            "message": format!("{} joined the room", client_id),
            "client_count": clients.len()
        });

        // Don't fail if broadcast fails (room might be empty)
        let _ = self.broadcaster.send(join_msg.to_string());

        Ok(receiver)
    }

    pub async fn leave_client(&self, client_id: &str) {
        let mut clients = self.clients.write().await;
        
        if clients.remove(client_id) {
            info!("Client {} left room '{}' ({} clients remaining)", client_id, self.name, clients.len());

            // Broadcast leave notification
            let leave_msg = serde_json::json!({
                "type": "user_left",
                "client_id": client_id,
                "room": self.name,
                "message": format!("{} left the room", client_id),
                "client_count": clients.len()
            });

            // Don't fail if broadcast fails
            let _ = self.broadcaster.send(leave_msg.to_string());
        }
    }

    pub async fn broadcast_message(&self, sender_id: &str, message: &str) -> Result<()> {
        let clients = self.clients.read().await;
        
        if !clients.contains(sender_id) {
            return Err(anyhow::anyhow!("Client not in room"));
        }

        let broadcast_msg = serde_json::json!({
            "type": "message",
            "client_id": sender_id,
            "room": self.name,
            "message": message,
            "timestamp": chrono::Utc::now().to_rfc3339()
        });

        match self.broadcaster.send(broadcast_msg.to_string()) {
            Ok(receiver_count) => {
                info!("Broadcasted message from {} in room '{}' to {} clients", 
                      sender_id, self.name, receiver_count);
            }
            Err(_) => {
                warn!("Failed to broadcast message in room '{}' (no active receivers)", self.name);
            }
        }

        Ok(())
    }

    pub async fn client_count(&self) -> usize {
        self.clients.read().await.len()
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}