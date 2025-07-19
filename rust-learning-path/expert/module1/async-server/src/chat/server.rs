use tokio::net::{TcpListener, TcpStream};
use tokio_tungstenite::{accept_async, tungstenite::Message};
use futures_util::{SinkExt, StreamExt};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::{broadcast, RwLock};
use tracing::{info, error, warn};
use anyhow::Result;
use uuid::Uuid;

use super::room::ChatRoom;

pub struct ChatServer {
    config: crate::config::ChatConfig,
    rooms: Arc<RwLock<HashMap<String, Arc<ChatRoom>>>>,
}

impl ChatServer {
    pub fn new(config: crate::config::ChatConfig) -> Self {
        Self {
            config,
            rooms: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub async fn run(&self, mut shutdown: tokio::sync::broadcast::Receiver<()>) -> Result<()> {
        let listener = TcpListener::bind(self.config.bind_addr).await?;
        info!("Chat server listening on {}", self.config.bind_addr);

        loop {
            tokio::select! {
                result = listener.accept() => {
                    match result {
                        Ok((stream, addr)) => {
                            info!("New WebSocket connection from {}", addr);
                            let rooms = Arc::clone(&self.rooms);
                            let config = self.config.clone();
                            
                            tokio::spawn(async move {
                                if let Err(e) = handle_websocket_connection(stream, addr, rooms, config).await {
                                    error!("WebSocket connection error from {}: {}", addr, e);
                                }
                            });
                        }
                        Err(e) => {
                            error!("Failed to accept WebSocket connection: {}", e);
                        }
                    }
                }
                _ = shutdown.recv() => {
                    info!("Chat server shutting down");
                    break;
                }
            }
        }

        Ok(())
    }

    pub async fn get_room_count(&self) -> usize {
        self.rooms.read().await.len()
    }
}

async fn handle_websocket_connection(
    stream: TcpStream,
    addr: std::net::SocketAddr,
    rooms: Arc<RwLock<HashMap<String, Arc<ChatRoom>>>>,
    config: crate::config::ChatConfig,
) -> Result<()> {
    let ws_stream = accept_async(stream).await?;
    let (mut ws_sender, mut ws_receiver) = ws_stream.split();
    
    let client_id = Uuid::new_v4().to_string();
    let mut current_room: Option<Arc<ChatRoom>> = None;
    let mut room_receiver: Option<broadcast::Receiver<String>> = None;

    info!("WebSocket client {} connected from {}", client_id, addr);

    // Send welcome message
    let welcome_msg = serde_json::json!({
        "type": "welcome",
        "client_id": client_id,
        "message": "Welcome to the chat server! Send {\"type\": \"join\", \"room\": \"room_name\"} to join a room."
    });
    
    if let Err(e) = ws_sender.send(Message::Text(welcome_msg.to_string())).await {
        error!("Failed to send welcome message: {}", e);
        return Ok(());
    }

    loop {
        tokio::select! {
            // Handle incoming messages from the client
            msg = ws_receiver.next() => {
                match msg {
                    Some(Ok(Message::Text(text))) => {
                        if let Err(e) = handle_client_message(
                            &text,
                            &client_id,
                            &mut current_room,
                            &mut room_receiver,
                            &rooms,
                            &config,
                            &mut ws_sender,
                        ).await {
                            error!("Error handling client message: {}", e);
                            break;
                        }
                    }
                    Some(Ok(Message::Close(_))) => {
                        info!("Client {} disconnected", client_id);
                        break;
                    }
                    Some(Err(e)) => {
                        error!("WebSocket error for client {}: {}", client_id, e);
                        break;
                    }
                    None => {
                        info!("WebSocket stream ended for client {}", client_id);
                        break;
                    }
                    _ => {} // Ignore other message types
                }
            }
            
            // Handle room broadcasts
            room_msg = async {
                if let Some(ref mut receiver) = room_receiver {
                    receiver.recv().await
                } else {
                    std::future::pending().await
                }
            } => {
                match room_msg {
                    Ok(broadcast_msg) => {
                        if let Err(e) = ws_sender.send(Message::Text(broadcast_msg)).await {
                            error!("Failed to send broadcast message: {}", e);
                            break;
                        }
                    }
                    Err(broadcast::error::RecvError::Closed) => {
                        warn!("Room broadcast channel closed");
                        room_receiver = None;
                    }
                    Err(broadcast::error::RecvError::Lagged(_)) => {
                        warn!("Client {} lagged behind in room messages", client_id);
                    }
                }
            }
        }
    }

    // Clean up: leave current room if any
    if let Some(room) = current_room {
        room.leave_client(&client_id).await;
    }

    Ok(())
}

async fn handle_client_message(
    text: &str,
    client_id: &str,
    current_room: &mut Option<Arc<ChatRoom>>,
    room_receiver: &mut Option<broadcast::Receiver<String>>,
    rooms: &Arc<RwLock<HashMap<String, Arc<ChatRoom>>>>,
    config: &crate::config::ChatConfig,
    ws_sender: &mut futures_util::stream::SplitSink<tokio_tungstenite::WebSocketStream<TcpStream>, Message>,
) -> Result<()> {
    let msg: serde_json::Value = serde_json::from_str(text)?;
    
    match msg.get("type").and_then(|t| t.as_str()) {
        Some("join") => {
            let room_name = msg.get("room")
                .and_then(|r| r.as_str())
                .ok_or_else(|| anyhow::anyhow!("Missing room name"))?;

            // Leave current room if any
            if let Some(room) = current_room.take() {
                room.leave_client(client_id).await;
            }

            // Get or create room
            let room = {
                let mut rooms_guard = rooms.write().await;
                
                if rooms_guard.len() >= config.max_rooms && !rooms_guard.contains_key(room_name) {
                    let error_msg = serde_json::json!({
                        "type": "error",
                        "message": "Maximum number of rooms reached"
                    });
                    ws_sender.send(Message::Text(error_msg.to_string())).await?;
                    return Ok(());
                }

                rooms_guard.entry(room_name.to_string())
                    .or_insert_with(|| Arc::new(ChatRoom::new(room_name.to_string(), config.max_clients_per_room)))
                    .clone()
            };

            // Try to join the room
            match room.join_client(client_id.to_string()).await {
                Ok(receiver) => {
                    *current_room = Some(room.clone());
                    *room_receiver = Some(receiver);
                    
                    let success_msg = serde_json::json!({
                        "type": "joined",
                        "room": room_name,
                        "message": format!("Successfully joined room '{}'", room_name)
                    });
                    ws_sender.send(Message::Text(success_msg.to_string())).await?;
                }
                Err(e) => {
                    let error_msg = serde_json::json!({
                        "type": "error",
                        "message": format!("Failed to join room: {}", e)
                    });
                    ws_sender.send(Message::Text(error_msg.to_string())).await?;
                }
            }
        }
        
        Some("message") => {
            let message_text = msg.get("message")
                .and_then(|m| m.as_str())
                .ok_or_else(|| anyhow::anyhow!("Missing message text"))?;

            if let Some(room) = current_room {
                room.broadcast_message(client_id, message_text).await?;
            } else {
                let error_msg = serde_json::json!({
                    "type": "error",
                    "message": "You must join a room before sending messages"
                });
                ws_sender.send(Message::Text(error_msg.to_string())).await?;
            }
        }
        
        Some("leave") => {
            if let Some(room) = current_room.take() {
                room.leave_client(client_id).await;
                *room_receiver = None;
                
                let success_msg = serde_json::json!({
                    "type": "left",
                    "message": "Successfully left the room"
                });
                ws_sender.send(Message::Text(success_msg.to_string())).await?;
            }
        }
        
        _ => {
            let error_msg = serde_json::json!({
                "type": "error",
                "message": "Unknown message type. Supported types: join, message, leave"
            });
            ws_sender.send(Message::Text(error_msg.to_string())).await?;
        }
    }

    Ok(())
}