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

### Start the Client (Desktop App)
```bash
cd client
npm install
cargo tauri dev
```

### Start the Client (Web Only)
```bash
cd client
npm install
npm run dev
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

## API Endpoints
| Method | Endpoint | Description |
|--------|----------|-------------|
| GET | `/api/health` | Health check |
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
| WS | `/ws?token=<jwt>` | Real-time WebSocket |

## Versioning
This project uses semantic versioning (v1.0.0).

## License
MIT
