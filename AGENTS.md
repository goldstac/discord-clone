# Discord Clone

## Project Overview
A Discord clone built with Tauri + Svelte (client) and Axum + SQLite (server). Runs locally on the user's machine.

## Tech Stack
- **Client:** Tauri 2.x, Svelte 4, Tailwind CSS
- **Server:** Rust, Axum, SQLite (sqlx)
- **Auth:** bcrypt + JWT
- **Real-time:** WebSocket

## Versioning
Semantic versioning (v1.0.0)
- **PATCH** (v1.0.X): Bug fixes, small tweaks
- **MINOR** (v1.X.0): New features (auth, channels, etc.)
- **MAJOR** (X.0.0): Breaking changes, major redesigns

## Branches
- `main`: Stable releases, tagged versions
- `dev`: Active development, all features built here

## Commands
```bash
# Start server
cd server && cargo run

# Start client
cd client && npm install && cargo tauri dev
```

## Project Structure
```
discord-clone/
├── server/        # Axum API + WebSocket server
│   ├── src/
│   │   ├── main.rs
│   │   ├── db.rs
│   │   ├── auth.rs
│   │   ├── handlers/
│   │   ├── models.rs
│   │   └── ws.rs
│   └── Cargo.toml
├── client/        # Tauri + Svelte desktop app
│   ├── src-tauri/
│   ├── src/
│   │   ├── lib/components/
│   │   ├── lib/stores/
│   │   └── lib/api.ts
│   ├── App.svelte
│   └── main.ts
├── AGENTS.md      # This file
├── CHANGELOG.md   # Version history
└── README.md      # Project documentation
```

## API Endpoints
| Method | Endpoint | Description |
|--------|----------|-------------|
| POST | `/api/auth/register` | Create account |
| POST | `/api/auth/login` | Login, get JWT |
| GET | `/api/users/me` | Current user profile |
| GET | `/api/servers` | List user's servers |
| POST | `/api/servers` | Create server |
| POST | `/api/servers/:id/join` | Join via invite code |
| GET | `/api/servers/:id/channels` | List channels |
| POST | `/api/servers/:id/channels` | Create channel |
| GET | `/api/channels/:id/messages` | Get message history |
| POST | `/api/channels/:id/messages` | Send message |
| WS | `/ws` | Real-time message stream |
