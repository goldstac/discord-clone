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

### Free Options

**Railway (Recommended)**
- Free Trial: $5 credit for 30 days (no card required)
- Free Plan: $1/month credit forever (small apps)
- Includes persistent storage
- Deploy: Connect GitHub → auto-deploys

**Fly.io**
- Free allowance: 3 shared-cpu-1x VMs, 160GB bandwidth
- Deploy with Dockerfile

### Paid Options

**Render ($7/month)**
- Requires paid Starter plan for persistent disks
- Free tier exists but data resets on each deploy

**Railway Hobby ($5/month)**
- $5/month usage credit included
- Good for always-on apps

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
