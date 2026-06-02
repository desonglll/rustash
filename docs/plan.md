# Rustash Detailed Task Plan

> Each PR does exactly one thing. Every change must compile and run without bugs. PR numbers are assigned during implementation.

## Phase 0: Infrastructure Setup

| PR | Task | Description | Files | Depends on |
|----|------|-------------|-------|------------|
| P0-1 | Add TailwindCSS | Install tailwindcss v4 + @tailwindcss/vite, configure vite.config.ts, create base CSS | vite.config.ts, src/index.css | None |
| P0-2 | Install Frontend Dependencies | Install react-router-dom, zustand, @headlessui/react, @heroicons/react, i18next, react-i18next | package.json | None |
| P0-3 | Install Rust Core Dependencies | Add rusqlite(bundled), serde, serde_json, directories, tokio to Cargo.toml | src-tauri/Cargo.toml | None |
| P0-4 | Configure Tauri Permissions | Configure capabilities/default.json with necessary API permissions | src-tauri/capabilities/ | P0-3 |
| P0-5 | Create Project Directory Structure | Create frontend src/ subdirectories (components, hooks, stores, pages, utils, types, locales) and backend src-tauri/src/ subdirectories (commands, db, models, services, config) | Directory creation | P0-2, P0-3 |
| P0-6 | Optimize TypeScript Config | Configure path aliases (@/ etc.), strict mode | tsconfig.json, vite.config.ts | P0-2 |

## Phase 1: Frontend Skeleton

| PR | Task | Description | Files | Depends on |
|----|------|-------------|-------|------------|
| P1-1 | Layout Components | MainLayout (Sidebar + Content + Header) using TailwindCSS | src/components/layout/ | P0-1 |
| P1-2 | Sidebar Navigation | Sidebar component with nav links (Scenes, Performers, Studios, Tags, Galleries, Settings) | src/components/layout/Sidebar.tsx | P1-1 |
| P1-3 | Route Configuration | React Router setup, base route table (placeholder page components) | src/App.tsx, src/pages/ | P1-2 |
| P1-4 | Theme System | Dark/light theme toggle, CSS variables + TailwindCSS dark mode | src/hooks/useTheme.ts, src/components/layout/ | P1-1 |
| P1-5 | i18n Foundation | i18next configuration, Chinese/English language pack skeleton | src/locales/ | P0-2 |
| P1-6 | State Management | Zustand store foundations: settingsStore, uiStore | src/stores/ | P0-2 |

## Phase 2: Database Layer

| PR | Task | Description | Files | Depends on |
|----|------|-------------|-------|------------|
| P2-1 | Database Initialization | SQLite connection management, migration framework, schema v1 | src-tauri/src/db/ | P0-3 |
| P2-2 | Schema: files Table | files/folders tables, filesystem metadata | src-tauri/src/db/migrations/ | P2-1 |
| P2-3 | Schema: scenes Table | scenes table + scene_markers table | src-tauri/src/db/migrations/ | P2-2 |
| P2-4 | Schema: performers Table | performers table | src-tauri/src/db/migrations/ | P2-2 |
| P2-5 | Schema: studios Table | studios table (self-referencing parent_studio) | src-tauri/src/db/migrations/ | P2-2 |
| P2-6 | Schema: tags Table | tags table + scenes_tags join table | src-tauri/src/db/migrations/ | P2-2 |
| P2-7 | Schema: galleries Table | galleries + gallery_chapters + images tables | src-tauri/src/db/migrations/ | P2-2 |
| P2-8 | Schema: groups Table | groups (formerly movies) table + scenes_groups join table | src-tauri/src/db/migrations/ | P2-2 |
| P2-9 | Schema: saved_filters + config | saved_filters table + config KV table | src-tauri/src/db/migrations/ | P2-2 |
| P2-10 | Rust Models: Core | File, Folder, Scene Rust structs + serde | src-tauri/src/models/ | P2-2, P2-3 |
| P2-11 | Rust Models: Extended | Performer, Studio, Tag, Gallery, Image, Group structs | src-tauri/src/models/ | P2-4~P2-8, P2-10 |
| P2-12 | DB CRUD Traits | Define generic Repository trait + per-entity Read/Write traits | src-tauri/src/db/ | P2-10, P2-11 |
| P2-13 | DB CRUD Implementation: Scene | Scene CRUD implementation + Tauri Command exposure | src-tauri/src/commands/ | P2-12 |

## Phase 3: Core Features — Scene Management

| PR | Task | Description | Files | Depends on |
|----|------|-------------|-------|------------|
| P3-1 | Scene List Page | SceneList page: grid/list view toggle, pagination | src/pages/Scenes/ | P1-3, P2-13 |
| P3-2 | Scene Card Component | SceneCard: thumbnail, title, tag preview | src/components/Scenes/ | P3-1 |
| P3-3 | Scene Detail Page | SceneDetail: video player placeholder, metadata display | src/pages/Scenes/ | P3-1 |
| P3-4 | Scene Edit Form | SceneEditForm: metadata editing, tag selection | src/components/Scenes/ | P3-3 |
| P3-5 | File Scan Command | scan_paths Tauri Command, scan directories and ingest | src-tauri/src/commands/ | P2-12 |
| P3-6 | Frontend Scan UI | Settings page scan config + scan button + progress bar | src/pages/Settings/ | P3-5 |
| P3-7 | Search/Filter | Scene filter (Tauri Command + frontend filter bar) | src/components/List/ | P3-1 |

## Phase 4: Extended Entities

| PR | Task | Description | Files | Depends on |
|----|------|-------------|-------|------------|
| P4-1 | Performer CRUD | Backend CRUD + list/detail/edit pages | Full stack | P2-12 |
| P4-2 | Studio CRUD | Backend CRUD + list/detail/edit pages | Full stack | P2-12 |
| P4-3 | Tag CRUD | Backend CRUD + list/detail/edit pages + color picker | Full stack | P2-12 |
| P4-4 | Gallery CRUD | Backend CRUD + list/detail pages + image grid | Full stack | P2-12 |
| P4-5 | Group CRUD | Backend CRUD + list/detail/edit pages | Full stack | P2-12 |
| P4-6 | Image CRUD | Backend CRUD + image browsing | Full stack | P2-12 |
| P4-7 | Entity Associations | Scene↔Performer, Scene↔Tag, Scene↔Studio associations | Full stack | P4-1~P4-3 |

## Phase 5: Advanced Features

| PR | Task | Description | Files | Depends on |
|----|------|-------------|-------|------------|
| P5-1 | Authentication | Login page, JWT auth, session management | Full stack | Phase 3 |
| P5-2 | Config Management | YAML config read/write (replaces stash's koanf) | Rust + Settings page | Phase 3 |
| P5-3 | Thumbnail Generation | FFmpeg integration, video thumbnails, preview generation | Rust | P3-5 |
| P5-4 | HLS Video Streaming | Video streaming playback, hls.js integration | Full stack | P3-3, P5-3 |
| P5-5 | Scraper Framework | XPath/JSON/GraphQL scraper engine | Rust | Phase 4 |
| P5-6 | Scraper UI | Scraper configuration + execution UI | Frontend | P5-5 |
| P5-7 | Plugin System | JS/WASM plugin runtime | Rust | Phase 4 |
| P5-8 | Auto-tag | Filename-based automatic tag matching | Rust | P4-3 |
| P5-9 | DLNA Service | DLNA media streaming service | Rust | P5-4 |
| P5-10 | Import/Export | JSON format data import/export | Full stack | Phase 4 |

## Phase 6: Polish

| PR | Task | Description | Files | Depends on |
|----|------|-------------|-------|------------|
| P6-1 | Performance Optimization | Virtual scrolling, image lazy loading, SQLite query optimization | Full stack | Phase 5 |
| P6-2 | Keyboard Shortcuts | Shortcut key system | Frontend | Phase 5 |
| P6-3 | i18n Completion | Full Chinese/English translations | src/locales/ | P1-5 |
| P6-4 | Test Coverage | Rust unit tests + frontend component tests | Full stack | Phase 5 |
| P6-5 | Packaging & Release | Tauri build config, auto-update, installers | src-tauri/ | Phase 6 |

## Implementation Standards

1. Every PR must compile and run
2. Each PR's code changes must be ≤ 500 lines (exceptions allowed with justification)
3. PR title format: `[P{phase}-{num}] brief description`
4. Before committing, run: `cargo clippy && bun run build`
5. Complex logic must have Rust unit tests
6. Reference stash code to understand business logic, but don't copy the implementation
7. All documentation must be written in English
