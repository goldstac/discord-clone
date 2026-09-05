use std::sync::Arc;

use axum::{
    extract::{
        ws::{Message, WebSocket},
        Query, State, WebSocketUpgrade,
    },
    response::IntoResponse,
};
use futures_util::{SinkExt, StreamExt};
use serde::{Deserialize, Serialize};
use tokio::sync::broadcast;

use crate::auth;

#[derive(Clone)]
pub struct AppState {
    pub pool: sqlx::SqlitePool,
    pub sender: broadcast::Sender<String>,
}

pub type SharedState = Arc<AppState>;

#[derive(Debug, Deserialize)]
pub struct WsParams {
    pub token: String,
}

#[derive(Debug, Serialize)]
pub struct WsMessage {
    pub user_id: i64,
    pub username: String,
    pub content: String,
    pub channel_id: i64,
    pub timestamp: String,
}

pub async fn ws_handler(
    ws: WebSocketUpgrade,
    Query(params): Query<WsParams>,
    State(state): State<SharedState>,
) -> impl IntoResponse {
    match auth::verify_token(&params.token) {
        Ok(claims) => {
            let user_id = claims.sub;
            let username = claims.username.clone();
            ws.on_upgrade(move |socket| handle_socket(socket, state, user_id, username))
        }
        Err(_) => {
            ws.on_upgrade(|mut socket| async move {
                let _ = socket
                    .send(Message::Text("Invalid token".into()))
                    .await;
                let _ = socket.close().await;
            })
        }
    }
}

async fn handle_socket(
    socket: WebSocket,
    state: SharedState,
    user_id: i64,
    username: String,
) {
    let (mut sender, mut receiver) = socket.split();

    let mut rx = state.sender.subscribe();

    let welcome = serde_json::json!({
        "type": "connected",
        "user_id": user_id,
        "username": username
    });
    let _ = sender.send(Message::Text(welcome.to_string().into())).await;

    let broadcast_sender = state.sender.clone();
    let username_clone = username.clone();

    let mut send_task = tokio::spawn(async move {
        while let Ok(msg) = rx.recv().await {
            if sender.send(Message::Text(msg.into())).await.is_err() {
                break;
            }
        }
    });

    let mut recv_task = tokio::spawn(async move {
        while let Some(Ok(Message::Text(text))) = receiver.next().await {
            if let Ok(input) = serde_json::from_str::<serde_json::Value>(&text) {
                let content = input["content"].as_str().unwrap_or("");
                let channel_id = input["channel_id"].as_i64().unwrap_or(0);

                if content.is_empty() || channel_id == 0 {
                    continue;
                }

                let timestamp = chrono::Utc::now()
                    .format("%Y-%m-%dT%H:%M:%SZ")
                    .to_string();

                // Save to database
                let _ = sqlx::query(
                    "INSERT INTO messages (channel_id, user_id, content, created_at) VALUES (?, ?, ?, ?)"
                )
                .bind(channel_id)
                .bind(user_id)
                .bind(content)
                .bind(&timestamp)
                .execute(&state.pool)
                .await;

                let ws_msg = WsMessage {
                    user_id,
                    username: username_clone.clone(),
                    content: content.to_string(),
                    channel_id,
                    timestamp,
                };

                if let Ok(json) = serde_json::to_string(&ws_msg) {
                    let _ = broadcast_sender.send(json);
                }
            }
        }
    });

    tokio::select! {
        _ = &mut send_task => recv_task.abort(),
        _ = &mut recv_task => send_task.abort(),
    }
}
