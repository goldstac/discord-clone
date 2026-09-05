mod auth;
mod db;
mod handlers;
mod ws;

use std::sync::Arc;

use axum::{routing::get, routing::post, Router, Json};
use serde::Serialize;
use tokio::sync::broadcast;
use tower_http::cors::{Any, CorsLayer};

use ws::SharedState;

#[derive(Serialize)]
struct HealthResponse {
    status: String,
    version: String,
}

async fn health() -> Json<HealthResponse> {
    Json(HealthResponse {
        status: "ok".to_string(),
        version: "1.0.0-alpha.1".to_string(),
    })
}

#[tokio::main]
async fn main() {
    let database_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "sqlite:discord.db?mode=rwc".to_string());

    let jwt_secret = std::env::var("JWT_SECRET")
        .unwrap_or_else(|_| "discord-clone-secret-change-in-production".to_string());

    // Set JWT secret for auth module
    auth::set_jwt_secret(&jwt_secret);

    let pool = db::init_db(&database_url)
        .await
        .expect("Failed to initialize database");

    println!("Database initialized successfully");

    let (sender, _receiver) = broadcast::channel::<String>(100);

    let state: SharedState = Arc::new(ws::AppState { pool, sender });

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    let app = Router::new()
        .route("/api/health", get(health))
        .route("/api/auth/register", post(auth::register))
        .route("/api/auth/login", post(auth::login))
        .route("/api/users/me", get(auth::get_me))
        .route(
            "/api/servers",
            get(handlers::list_servers).post(handlers::create_server),
        )
        .route(
            "/api/servers/{id}/channels",
            get(handlers::list_channels).post(handlers::create_channel),
        )
        .route("/api/servers/{id}/join", post(handlers::join_server))
        .route(
            "/api/servers/{id}/invites",
            post(handlers::create_invite),
        )
        .route(
            "/api/channels/{id}/messages",
            get(handlers::list_messages).post(handlers::send_message),
        )
        .route("/ws", get(ws::ws_handler))
        .with_state(state)
        .layer(cors);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .unwrap();

    println!("Server running on http://localhost:3000");

    axum::serve(listener, app).await.unwrap();
}
