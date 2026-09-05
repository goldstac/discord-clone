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

## Deployment

### Server (Railway) - FREE
1. Create a [Railway](https://railway.app) account
2. Click "New Project" → "Deploy from GitHub repo"
3. Select this repository
4. Add environment variables:
   - `DATABASE_URL`: `sqlite:/data/discord.db?mode=rwc`
   - `JWT_SECRET`: (generate a random secret)
5. Railway will auto-deploy using the Dockerfile
6. **Free tier includes persistent storage!**

### Server (Render) - $7/month
1. Create a [Render](https://render.com) account
2. Click "New" → "Web Service"
3. Connect your GitHub repository
4. Configure:
   - **Name:** discord-clone-server
   - **Runtime:** Rust
   - **Build Command:** `cd server && cargo build --release --bin discord-clone-server`
   - **Start Command:** `cd server && ./target/release/discord-clone-server`
   - **Plan:** Starter ($7/month) - required for persistent disk
5. Add environment variables:
   - `DATABASE_URL`: `sqlite:/data/discord.db?mode=rwc`
   - `JWT_SECRET`: (generate a random secret)
6. Add a Disk (required for data persistence):
   - **Mount Path:** `/data`
   - **Size:** 1 GB
7. Create Web Service

> **Note:** Render's free tier doesn't support persistent disks. Your database will reset on each deploy. Use Railway for free hosting.

### Client (Vercel) - FREE
1. Create a [Vercel](https://vercel.com) account
2. Click "New Project" → Import this repository
3. Set environment variables:
   - `VITE_API_URL`: `https://your-server.up.railway.app`
   - `VITE_WS_URL`: `wss://your-server.up.railway.app`
4. Deploy

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
