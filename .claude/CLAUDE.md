# Rustash Project Guide

## Overview

Rustash is a rewrite of [stashapp/stash](https://github.com/stashapp/stash) using Tauri v2 + Rust + React + TailwindCSS instead of the original Go + React + Bootstrap stack.

The original stash code is located in the `stash/` directory (via symlink) for reference only. The current rustash project is a fresh Tauri v2 scaffold.

## Tech Stack

| Layer | Technology |
|-------|------------|
| Desktop Framework | Tauri v2 |
| Backend | Rust (src-tauri/) |
| Frontend | React 18 + TypeScript |
| Styling | TailwindCSS v4 |
| Build | Vite 6 + bun |
| Database | SQLite (via rusqlite/diesel) |
| API | Tauri Commands (not GraphQL) |

## Core Principles

1. **Minimal-change principle**: Each PR does exactly one thing, with the fewest lines of code possible
2. **Correctness first**: Every step must compile and run without bugs
3. **Incremental rewrite**: Build the skeleton first, then fill in features layer by layer
4. **Reference, don't copy**: Understand stash's architectural intent, reimplement using Rust idioms

## Project Structure

```
rustash/
├── stash/                  # Original stash project (symlink, reference only)
├── src/                    # React frontend source
├── src-tauri/              # Rust backend source
│   └── src/
│       ├── main.rs
│       ├── lib.rs
│       ├── commands/       # Tauri commands
│       ├── db/             # Database layer
│       ├── models/         # Data models
│       ├── services/       # Business logic
│       └── config/         # Configuration management
├── docs/                   # Project documentation
│   ├── plan.md             # Detailed task plan
│   └── architecture.md     # Architecture design
└── .claude/
    ├── CLAUDE.md           # This file
    └── settings.json       # Claude Code settings
```

## Development Commands

```bash
bun install              # Install frontend dependencies
bun run dev              # Start dev server
bun run tauri dev        # Start Tauri dev mode
bun run build            # Build frontend
bun run tauri build      # Build production package
cd src-tauri && cargo test  # Run Rust tests
cd src-tauri && cargo clippy # Rust lint
```

## Rewrite Strategy

### Phase Breakdown

1. **Phase 0: Infrastructure** — Project config, dependencies, build pipeline
2. **Phase 1: Frontend Skeleton** — Routing, layout, theme system
3. **Phase 2: Database Layer** — SQLite schema, migrations, models
4. **Phase 3: Core Features** — Scene management (CRUD), file scanning
5. **Phase 4: Extended Entities** — Tags, performers, studios, galleries
6. **Phase 5: Advanced Features** — Scrapers, plugin system, DLNA
7. **Phase 6: Polish** — Performance optimization, i18n, test coverage

### Stash Feature Module Mapping

| Stash Module | Description | Priority |
|-------------|-------------|----------|
| Scene Management | Scene/video CRUD, browsing, search | P0 |
| File Scanning | Filesystem scanning, ingestion | P0 |
| Performer | Performer management | P1 |
| Studio | Studio management | P1 |
| Tag | Tag management | P1 |
| Gallery | Gallery management | P1 |
| Config | Configuration management | P0 |
| Auth | Authentication system | P1 |
| Scrapers | Web scrapers | P2 |
| Plugins | Plugin system | P2 |
| DLNA | Media streaming | P2 |
| Auto-tag | Automatic tagging | P2 |
| FFmpeg | Transcoding, thumbnails | P1 |
| Identify | Scene identification | P2 |

### API Design Strategy

Stash uses GraphQL; rustash uses Tauri Commands. The mapping is:

- GraphQL Query → Tauri Command (read-only)
- GraphQL Mutation → Tauri Command (write operation)
- GraphQL Subscription → Tauri Event (event push)

Naming convention: `#[tauri::command]` functions use snake_case; frontend calls use camelCase (Tauri auto-converts).

## Frontend Architecture (Redesigned from stash)

- Routing: React Router v6 (consistent with stash)
- State Management: Zustand (lightweight, replaces Apollo Client's local cache)
- UI Components: Headless UI + TailwindCSS (replaces Bootstrap + react-bootstrap)
- Video Playback: video.js or hls.js
- Internationalization: i18next (consistent with stash)

## Notes

- The `stash/` directory is a symlink — read-only reference, do not modify
- Before each commit, ensure both `cargo clippy` and `bun run build` pass
- Tauri v2's permission system needs to be configured in capabilities/
- SQLite in Tauri requires using the bundled feature for compilation
- All documentation must be written in English
