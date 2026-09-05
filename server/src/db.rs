use sqlx::sqlite::SqlitePoolOptions;
use sqlx::{Pool, Sqlite};

pub type DbPool = Pool<Sqlite>;

pub async fn init_db(database_url: &str) -> Result<DbPool, sqlx::Error> {
    let pool = SqlitePoolOptions::new()
        .connect(database_url)
        .await?;

    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS users (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            username TEXT UNIQUE NOT NULL,
            password_hash TEXT NOT NULL,
            display_name TEXT,
            avatar_url TEXT,
            status TEXT DEFAULT 'online',
            last_seen DATETIME DEFAULT CURRENT_TIMESTAMP,
            created_at DATETIME DEFAULT CURRENT_TIMESTAMP
        );

        CREATE TABLE IF NOT EXISTS servers (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL,
            icon_url TEXT,
            owner_id INTEGER REFERENCES users(id),
            created_at DATETIME DEFAULT CURRENT_TIMESTAMP
        );

        CREATE TABLE IF NOT EXISTS channels (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            server_id INTEGER REFERENCES servers(id) ON DELETE CASCADE,
            name TEXT NOT NULL,
            type TEXT DEFAULT 'text',
            topic TEXT,
            position INTEGER DEFAULT 0,
            created_at DATETIME DEFAULT CURRENT_TIMESTAMP
        );

        CREATE TABLE IF NOT EXISTS messages (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            channel_id INTEGER REFERENCES channels(id) ON DELETE CASCADE,
            user_id INTEGER REFERENCES users(id),
            content TEXT NOT NULL,
            edited_at DATETIME,
            created_at DATETIME DEFAULT CURRENT_TIMESTAMP
        );

        CREATE TABLE IF NOT EXISTS server_members (
            server_id INTEGER REFERENCES servers(id) ON DELETE CASCADE,
            user_id INTEGER REFERENCES users(id) ON DELETE CASCADE,
            role TEXT DEFAULT 'member',
            joined_at DATETIME DEFAULT CURRENT_TIMESTAMP,
            PRIMARY KEY (server_id, user_id)
        );

        CREATE TABLE IF NOT EXISTS server_invites (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            server_id INTEGER REFERENCES servers(id) ON DELETE CASCADE,
            code TEXT UNIQUE NOT NULL,
            created_by INTEGER REFERENCES users(id),
            uses INTEGER DEFAULT 0,
            max_uses INTEGER,
            created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
            expires_at DATETIME
        );
        "#,
    )
    .execute(&pool)
    .await?;

    Ok(pool)
}
