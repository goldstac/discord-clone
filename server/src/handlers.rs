use axum::{extract::{Path, State}, Json, http::HeaderMap};
use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;

use crate::auth::verify_token;

fn get_user_id(headers: &HeaderMap) -> Result<i64, String> {
    let auth_header = headers
        .get("Authorization")
        .and_then(|v| v.to_str().ok())
        .ok_or("Missing Authorization header")?;

    let token = auth_header
        .strip_prefix("Bearer ")
        .ok_or("Invalid Authorization format")?;

    let claims = verify_token(token)?;
    Ok(claims.sub)
}

#[derive(Serialize)]
pub struct ServerResponse {
    pub id: i64,
    pub name: String,
    pub icon_url: Option<String>,
    pub owner_id: i64,
}

#[derive(Deserialize)]
pub struct CreateServerRequest {
    pub name: String,
}

#[derive(Deserialize)]
pub struct JoinServerRequest {
    pub code: String,
}

#[derive(Serialize)]
pub struct ChannelResponse {
    pub id: i64,
    pub name: String,
    pub channel_type: String,
    pub topic: Option<String>,
    pub position: i32,
}

#[derive(Deserialize)]
pub struct CreateChannelRequest {
    pub name: String,
    #[serde(default = "default_channel_type")]
    pub channel_type: String,
}

fn default_channel_type() -> String {
    "text".to_string()
}

#[derive(Serialize)]
pub struct InviteResponse {
    pub code: String,
    pub server_name: String,
}

pub async fn list_servers(
    State(pool): State<SqlitePool>,
    headers: HeaderMap,
) -> Result<Json<Vec<ServerResponse>>, String> {
    let user_id = get_user_id(&headers)?;

    let servers = sqlx::query_as::<_, (i64, String, Option<String>, i64)>(
        r#"
        SELECT s.id, s.name, s.icon_url, s.owner_id
        FROM servers s
        INNER JOIN server_members sm ON s.id = sm.server_id
        WHERE sm.user_id = ?
        "#,
    )
    .bind(user_id)
    .fetch_all(&pool)
    .await
    .map_err(|e| e.to_string())?;

    let result = servers
        .into_iter()
        .map(|(id, name, icon_url, owner_id)| ServerResponse {
            id,
            name,
            icon_url,
            owner_id,
        })
        .collect();

    Ok(Json(result))
}

pub async fn create_server(
    State(pool): State<SqlitePool>,
    headers: HeaderMap,
    Json(input): Json<CreateServerRequest>,
) -> Result<Json<ServerResponse>, String> {
    let user_id = get_user_id(&headers)?;

    if input.name.len() < 2 || input.name.len() > 100 {
        return Err("Server name must be 2-100 characters".to_string());
    }

    let result = sqlx::query("INSERT INTO servers (name, owner_id) VALUES (?, ?)")
        .bind(&input.name)
        .bind(user_id)
        .execute(&pool)
        .await
        .map_err(|e| e.to_string())?;

    let server_id = result.last_insert_rowid();

    sqlx::query("INSERT INTO server_members (server_id, user_id, role) VALUES (?, ?, 'owner')")
        .bind(server_id)
        .bind(user_id)
        .execute(&pool)
        .await
        .map_err(|e| e.to_string())?;

    sqlx::query("INSERT INTO channels (server_id, name, type) VALUES (?, 'general', 'text')")
        .bind(server_id)
        .execute(&pool)
        .await
        .map_err(|e| e.to_string())?;

    let invite_code = generate_invite_code();
    sqlx::query("INSERT INTO server_invites (server_id, code, created_by) VALUES (?, ?, ?)")
        .bind(server_id)
        .bind(&invite_code)
        .bind(user_id)
        .execute(&pool)
        .await
        .map_err(|e| e.to_string())?;

    Ok(Json(ServerResponse {
        id: server_id,
        name: input.name,
        icon_url: None,
        owner_id: user_id,
    }))
}

pub async fn join_server(
    State(pool): State<SqlitePool>,
    headers: HeaderMap,
    Json(input): Json<JoinServerRequest>,
) -> Result<Json<ServerResponse>, String> {
    let user_id = get_user_id(&headers)?;

    let invite = sqlx::query_as::<_, (i64, i64, i32, Option<i32>)>(
        "SELECT id, server_id, uses, max_uses FROM server_invites WHERE code = ?",
    )
    .bind(&input.code)
    .fetch_optional(&pool)
    .await
    .map_err(|e| e.to_string())?
    .ok_or("Invalid invite code")?;

    let (_invite_id, server_id, uses, max_uses) = invite;

    if let Some(max) = max_uses {
        if uses >= max {
            return Err("Invite code has expired".to_string());
        }
    }

    let already_member = sqlx::query_scalar::<_, i64>(
        "SELECT COUNT(*) FROM server_members WHERE server_id = ? AND user_id = ?",
    )
    .bind(server_id)
    .bind(user_id)
    .fetch_one(&pool)
    .await
    .map_err(|e| e.to_string())?;

    if already_member > 0 {
        return Err("Already a member of this server".to_string());
    }

    sqlx::query("INSERT INTO server_members (server_id, user_id) VALUES (?, ?)")
        .bind(server_id)
        .bind(user_id)
        .execute(&pool)
        .await
        .map_err(|e| e.to_string())?;

    sqlx::query("UPDATE server_invites SET uses = uses + 1 WHERE code = ?")
        .bind(&input.code)
        .execute(&pool)
        .await
        .map_err(|e| e.to_string())?;

    let server = sqlx::query_as::<_, (i64, String, Option<String>, i64)>(
        "SELECT id, name, icon_url, owner_id FROM servers WHERE id = ?",
    )
    .bind(server_id)
    .fetch_one(&pool)
    .await
    .map_err(|e| e.to_string())?;

    Ok(Json(ServerResponse {
        id: server.0,
        name: server.1,
        icon_url: server.2,
        owner_id: server.3,
    }))
}

pub async fn list_channels(
    State(pool): State<SqlitePool>,
    headers: HeaderMap,
    Path(server_id): Path<i64>,
) -> Result<Json<Vec<ChannelResponse>>, String> {
    let user_id = get_user_id(&headers)?;

    let is_member = sqlx::query_scalar::<_, i64>(
        "SELECT COUNT(*) FROM server_members WHERE server_id = ? AND user_id = ?",
    )
    .bind(server_id)
    .bind(user_id)
    .fetch_one(&pool)
    .await
    .map_err(|e| e.to_string())?;

    if is_member == 0 {
        return Err("Not a member of this server".to_string());
    }

    let channels = sqlx::query_as::<_, (i64, String, String, Option<String>, i32)>(
        "SELECT id, name, type, topic, position FROM channels WHERE server_id = ? ORDER BY position",
    )
    .bind(server_id)
    .fetch_all(&pool)
    .await
    .map_err(|e| e.to_string())?;

    let result = channels
        .into_iter()
        .map(|(id, name, channel_type, topic, position)| ChannelResponse {
            id,
            name,
            channel_type,
            topic,
            position,
        })
        .collect();

    Ok(Json(result))
}

pub async fn create_channel(
    State(pool): State<SqlitePool>,
    headers: HeaderMap,
    Path(server_id): Path<i64>,
    Json(input): Json<CreateChannelRequest>,
) -> Result<Json<ChannelResponse>, String> {
    let user_id = get_user_id(&headers)?;

    let is_member = sqlx::query_scalar::<_, i64>(
        "SELECT COUNT(*) FROM server_members WHERE server_id = ? AND user_id = ?",
    )
    .bind(server_id)
    .bind(user_id)
    .fetch_one(&pool)
    .await
    .map_err(|e| e.to_string())?;

    if is_member == 0 {
        return Err("Not a member of this server".to_string());
    }

    if input.name.len() < 1 || input.name.len() > 100 {
        return Err("Channel name must be 1-100 characters".to_string());
    }

    let max_pos = sqlx::query_scalar::<_, Option<i32>>(
        "SELECT MAX(position) FROM channels WHERE server_id = ?",
    )
    .bind(server_id)
    .fetch_one(&pool)
    .await
    .map_err(|e| e.to_string())?;

    let position = max_pos.unwrap_or(0) + 1;

    let result = sqlx::query(
        "INSERT INTO channels (server_id, name, type, position) VALUES (?, ?, ?, ?)",
    )
    .bind(server_id)
    .bind(&input.name)
    .bind(&input.channel_type)
    .bind(position)
    .execute(&pool)
    .await
    .map_err(|e| e.to_string())?;

    let channel_id = result.last_insert_rowid();

    Ok(Json(ChannelResponse {
        id: channel_id,
        name: input.name,
        channel_type: input.channel_type,
        topic: None,
        position,
    }))
}

pub async fn create_invite(
    State(pool): State<SqlitePool>,
    headers: HeaderMap,
    Path(server_id): Path<i64>,
) -> Result<Json<InviteResponse>, String> {
    let user_id = get_user_id(&headers)?;

    let is_member = sqlx::query_scalar::<_, i64>(
        "SELECT COUNT(*) FROM server_members WHERE server_id = ? AND user_id = ?",
    )
    .bind(server_id)
    .bind(user_id)
    .fetch_one(&pool)
    .await
    .map_err(|e| e.to_string())?;

    if is_member == 0 {
        return Err("Not a member of this server".to_string());
    }

    let server_name = sqlx::query_scalar::<_, String>("SELECT name FROM servers WHERE id = ?")
        .bind(server_id)
        .fetch_one(&pool)
        .await
        .map_err(|e| e.to_string())?;

    let code = generate_invite_code();

    sqlx::query("INSERT INTO server_invites (server_id, code, created_by) VALUES (?, ?, ?)")
        .bind(server_id)
        .bind(&code)
        .bind(user_id)
        .execute(&pool)
        .await
        .map_err(|e| e.to_string())?;

    Ok(Json(InviteResponse {
        code,
        server_name,
    }))
}

fn generate_invite_code() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    let ts = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    format!("{:x}", ts)
}
