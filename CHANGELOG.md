# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Planned for Alpha 2
- Server creation dialog
- Channel creation dialog
- Members sidebar
- User profiles
- Better avatars with usernames
- Message timestamps with dates

### Planned for Beta
- Backend server (Axum + SQLite)
- User authentication (register/login)
- Real-time messaging via WebSocket
- Invite code system
- File attachments

## [v1.0.0-alpha.1] - 2026-09-04

### Added
- **UI Infrastructure**
  - Tauri 2.x + Svelte 4 project setup
  - Vite build configuration
  - Discord dark theme (#313338, #5865f2)
  - Responsive layout (server sidebar, channel list, chat area)

- **Components**
  - `ServerSidebar.svelte` - Server icons with hover effects
  - `ChannelList.svelte` - Channel list with active states
  - `ChatArea.svelte` - Messages display + input field

- **State Management**
  - Svelte stores for reactive state
  - Server/channel switching
  - Message history per channel
  - Working message input (Enter/Send)

- **Features**
  - Clickable servers (switch between My Server, Gaming, Music)
  - Clickable channels (switch between different channels)
  - Send messages that appear in chat
  - Empty channel state handling
  - User panel with online status

### Known Limitations
- No backend server (data is local/static)
- No user authentication
- No real-time sync between clients
- Placeholder data only
- No file attachments
- No voice/video chat

## [v1.0.0] - 2026-09-04

### Added
- Project scaffold and initial files
- Git workflow setup (main/dev branches)
- AGENTS.md with project documentation
- README.md with setup instructions
- CHANGELOG.md for version tracking
- .gitignore for Rust/Node/Tauri
- GNU GPL v3 License
