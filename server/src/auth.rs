use axum::{extract::State, http::HeaderMap, Json, http::StatusCode};
use bcrypt::{hash, verify, DEFAULT_COST};
use chrono::{Duration, Utc};
use jsonwebtoken::{encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use std::sync::OnceLock;

use crate::ws::SharedState;

static JWT_SECRET: OnceLock<String> = OnceLock::new();

pub fn set_jwt_secret(secret: &str) {
    let _ = JWT_SECRET.set(secret.to_string());
}

fn get_jwt_secret() -> &'static str {
    JWT_SECRET.get().map(|s| s.as_str()).unwrap_or("discord-clone-secret-change-in-production")
}

#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub error: String,
}

fn err(msg: &str) -> (StatusCode, Json<ErrorResponse>) {
    (StatusCode::BAD_REQUEST, Json(ErrorResponse { error: msg.to_string() }))
}

#[derive(Debug, Deserialize)]
pub struct RegisterRequest {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct AuthResponse {
    pub token: String,
    pub user: UserResponse,
}

#[derive(Debug, Serialize)]
pub struct UserResponse {
    pub id: i64,
    pub username: String,
    pub display_name: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: i64,
    pub username: String,
    pub exp: usize,
}

pub async fn register(
    State(state): State<SharedState>,
    Json(input): Json<RegisterRequest>,
) -> Result<(StatusCode, Json<AuthResponse>), (StatusCode, Json<ErrorResponse>)> {
    if input.username.len() < 3 || input.username.len() > 32 {
        return Err(err("Username must be 3-32 characters"));
    }
    if input.password.len() < 6 {
        return Err(err("Password must be at least 6 characters"));
    }

    let existing = sqlx::query("SELECT id FROM users WHERE username = ?")
        .bind(&input.username)
        .fetch_optional(&state.pool)
        .await
        .map_err(|e| err(&e.to_string()))?;

    if existing.is_some() {
        return Err(err("Username already taken"));
    }

    let password_hash = hash(&input.password, DEFAULT_COST)
        .map_err(|e| err(&e.to_string()))?;

    let result = sqlx::query("INSERT INTO users (username, password_hash) VALUES (?, ?)")
        .bind(&input.username)
        .bind(&password_hash)
        .execute(&state.pool)
        .await
        .map_err(|e| err(&e.to_string()))?;

    let user_id = result.last_insert_rowid();

    let token = create_token(user_id, &input.username)
        .map_err(|e| err(&e.to_string()))?;

    Ok((StatusCode::CREATED, Json(AuthResponse {
        token,
        user: UserResponse {
            id: user_id,
            username: input.username,
            display_name: None,
        },
    })))
}

pub async fn login(
    State(state): State<SharedState>,
    Json(input): Json<LoginRequest>,
) -> Result<(StatusCode, Json<AuthResponse>), (StatusCode, Json<ErrorResponse>)> {
    let user = sqlx::query_as::<_, (i64, String, Option<String>)>(
        "SELECT id, username, display_name FROM users WHERE username = ?",
    )
    .bind(&input.username)
    .fetch_optional(&state.pool)
    .await
    .map_err(|e| err(&e.to_string()))?;

    let (user_id, username, display_name) = user.ok_or_else(|| err("Invalid username or password"))?;

    let password_hash = sqlx::query_scalar::<_, String>("SELECT password_hash FROM users WHERE id = ?")
        .bind(user_id)
        .fetch_one(&state.pool)
        .await
        .map_err(|e| err(&e.to_string()))?;

    verify(&input.password, &password_hash)
        .map_err(|_| err("Invalid username or password"))?;

    let token = create_token(user_id, &username)
        .map_err(|e| err(&e.to_string()))?;

    Ok((StatusCode::OK, Json(AuthResponse {
        token,
        user: UserResponse {
            id: user_id,
            username,
            display_name,
        },
    })))
}

pub async fn get_me(
    State(state): State<SharedState>,
    headers: HeaderMap,
) -> Result<Json<UserResponse>, (StatusCode, Json<ErrorResponse>)> {
    let auth_header = headers
        .get("Authorization")
        .and_then(|v| v.to_str().ok())
        .ok_or_else(|| err("Missing Authorization header"))?;

    let token = auth_header
        .strip_prefix("Bearer ")
        .ok_or_else(|| err("Invalid Authorization format"))?;

    let claims = verify_token(token)
        .map_err(|e| err(&e))?;

    let user = sqlx::query_as::<_, (i64, String, Option<String>)>(
        "SELECT id, username, display_name FROM users WHERE id = ?",
    )
    .bind(claims.sub)
    .fetch_optional(&state.pool)
    .await
    .map_err(|e| err(&e.to_string()))?;

    let (id, username, display_name) = user.ok_or_else(|| err("User not found"))?;

    Ok(Json(UserResponse {
        id,
        username,
        display_name,
    }))
}

pub fn create_token(user_id: i64, username: &str) -> Result<String, String> {
    let exp = Utc::now()
        .checked_add_signed(Duration::hours(24))
        .expect("valid timestamp")
        .timestamp() as usize;

    let claims = Claims {
        sub: user_id,
        username: username.to_string(),
        exp,
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(get_jwt_secret().as_bytes()),
    )
    .map_err(|e| e.to_string())
}

pub fn verify_token(token: &str) -> Result<Claims, String> {
    let data = jsonwebtoken::decode::<Claims>(
        token,
        &DecodingKey::from_secret(get_jwt_secret().as_bytes()),
        &Validation::default(),
    )
    .map_err(|e| e.to_string())?;

    Ok(data.claims)
}
