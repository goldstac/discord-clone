# Discord Clone

A Discord clone built with Tauri + Svelte (client) and Axum + SQLite (server).

## Features
- User registration and login
- Server creation and management
- Channel management (text channels)
- Real-time messaging via WebSocket
- Invite code system
- Discord-like dark theme UI

## Prerequisites
- Rust (latest stable)
- Node.js (v18+)
- Cargo Tauri CLI

## Getting Started

### Start the Server
```bash
cd server
cargo run
```

### Start the Client
```bash
cd client
npm install
cargo tauri dev
```

## Tech Stack
| Component | Technology |
|-----------|-----------|
| Desktop Framework | Tauri 2.x |
| Frontend | Svelte 4 + Tailwind CSS |
| Backend | Rust + Axum |
| Database | SQLite |
| Auth | bcrypt + JWT |
| Real-time | WebSocket |

## Versioning
This project uses semantic versioning (v1.0.0).

## License
MIT
