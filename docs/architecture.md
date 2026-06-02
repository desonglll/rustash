# Rustash Architecture Design

## Overall Architecture

```
┌─────────────────────────────────────────────┐
│                  Frontend                    │
│  React + TypeScript + TailwindCSS + Vite    │
│  ┌──────┐ ┌──────┐ ┌──────┐ ┌──────────┐   │
│  │Pages │ │Stores│ │Hooks │ │Components│   │
│  └──┬───┘ └──┬───┘ └──┬───┘ └────┬─────┘   │
│     └────────┴────────┴──────────┘          │
│              Tauri IPC (invoke)              │
├─────────────────────────────────────────────┤
│                  Rust Backend                │
│  ┌─────────────────────────────────────┐    │
│  │           Tauri Commands             │    │
│  │  scene_list, scene_create, scan...   │    │
│  └──────────────┬──────────────────────┘    │
│  ┌──────────────┴──────────────────────┐    │
│  │           Services Layer             │    │
│  │  SceneService, ScanService, ...      │    │
│  └──────────────┬──────────────────────┘    │
│  ┌──────────────┴──────────────────────┐    │
│  │          Repository Layer            │    │
│  │  SceneRepo, PerformerRepo, ...       │    │
│  └──────────────┬──────────────────────┘    │
│  ┌──────────────┴──────────────────────┐    │
│  │           Models + DB                │    │
│  │  SQLite (rusqlite) + Migrations      │    │
│  └─────────────────────────────────────┘    │
└─────────────────────────────────────────────┘
```

## Layer Responsibilities

### Frontend Layer

| Module | Responsibility |
|--------|---------------|
| Pages | Route page components |
| Components | Reusable UI components |
| Stores | Zustand global state |
| Hooks | Custom React Hooks |
| Utils | Utility functions |
| Types | TypeScript type definitions |
| Locales | Internationalization resources |

### Rust Backend Layer

| Module | Responsibility |
|--------|---------------|
| commands/ | Tauri Command handlers (thin layer, param validation + service dispatch) |
| services/ | Business logic layer (transaction orchestration, permission checks) |
| db/ | Database access layer (SQLite connection pool, migrations, CRUD) |
| models/ | Domain models (pure data structures + serde) |
| config/ | Configuration management (YAML read/write) |

## Data Flow

### Read Data
```
Frontend → invoke("scene_list", filters) → Command → Service → Repo → SQLite → returns Vec<Scene>
```

### Write Data
```
Frontend → invoke("scene_update", data) → Command → Service → Repo → SQLite → returns Scene
```

### Event Push (replaces GraphQL Subscription)
```
Rust Backend → app.emit("scan_progress", payload) → Frontend listen("scan_progress", callback)
```

## Database Design

### Core Tables (simplified from stash schema v85)

```sql
-- Filesystem
files (id, basename, parent_folder_id, size, mod_time, created_at, updated_at)
folders (id, path, parent_folder_id, mod_time, created_at, updated_at)

-- Core entities
scenes (id, title, details, url, date, rating, organized, studio_id,
        created_at, updated_at)
scene_files (scene_id, file_id)  -- many-to-many
scene_markers (id, scene_id, title, seconds, primary_tag_id, created_at, updated_at)

performers (id, name, disambiguation, gender, url, birthdate, ethnicity,
            country, eye_color, height, measurements, fake_tits, career_length,
            tattoos, piercings, alias_list, rating, detail, death_date, hair_color,
            weight, studio_id, created_at, updated_at)

studios (id, name, url, parent_studio_id, image, created_at, updated_at)

tags (id, name, category_id, description, ignore_auto_tag, image, created_at, updated_at)

galleries (id, title, details, url, date, studio_id, rating, organized,
           created_at, updated_at)
gallery_images (gallery_id, image_id)

images (id, title, rating, organized, studio_id, created_at, updated_at)
image_files (image_id, file_id)

groups (id, name, aliases, duration, date, rating, studio_id, director,
        synopsis, created_at, updated_at)

-- Join tables
scenes_performers (scene_id, performer_id)
scenes_tags (scene_id, tag_id)
scenes_groups (scene_id, group_id)
performers_tags (performer_id, tag_id)
galleries_tags (gallery_id, tag_id)
galleries_performers (gallery_id, performer_id)
galleries_scenes (gallery_id, scene_id)
images_tags (image_id, tag_id)
images_performers (image_id, performer_id)

-- Other
saved_filters (id, mode, name, filter_json, created_at, updated_at)
config (key TEXT PRIMARY KEY, value TEXT)
```

### Migration Strategy

- Use embedded SQL files, numbered sequentially: `001_initial.sql`, `002_add_xxx.sql`
- On startup, check current version and auto-execute unapplied migrations
- Version tracking in `schema_migrations` table

## Tauri Commands Design

### General Pattern

```rust
#[tauri::command]
async fn scene_list(
    state: State<'_, AppState>,
    filter: Option<SceneFilter>,
    page: Option<i32>,
    per_page: Option<i32>,
) -> Result<PaginatedResult<Scene>, String> {
    let service = SceneService::new(&state.db);
    service.list(filter, page.unwrap_or(1), per_page.unwrap_or(25))
        .map_err(|e| e.to_string())
}
```

### Naming Convention

| Operation | Pattern | Example |
|-----------|---------|---------|
| List | `{entity}_list` | `scene_list` |
| Find by ID | `{entity}_find` | `scene_find` |
| Create | `{entity}_create` | `scene_create` |
| Update | `{entity}_update` | `scene_update` |
| Delete | `{entity}_destroy` | `scene_destroy` |
| Count | `{entity}_count` | `scene_count` |

## Frontend Component Architecture

### Page Routes

```
/                    → FrontPage (dashboard)
/scenes              → SceneList
/scenes/:id          → SceneDetail
/scenes/:id/edit     → SceneEdit
/performers          → PerformerList
/performers/:id      → PerformerDetail
/studios             → StudioList
/studios/:id         → StudioDetail
/tags                → TagList
/tags/:id            → TagDetail
/galleries           → GalleryList
/galleries/:id       → GalleryDetail
/images              → ImageList
/groups              → GroupList
/settings            → Settings
/settings/about      → About
```

### Zustand Stores

```typescript
// Settings store
interface SettingsStore {
  theme: 'dark' | 'light';
  language: string;
  setTheme: (theme: string) => void;
  setLanguage: (lang: string) => void;
}

// UI store
interface UIStore {
  sidebarCollapsed: boolean;
  setSidebarCollapsed: (collapsed: boolean) => void;
}

// Entity store (per-entity)
interface SceneStore {
  scenes: Scene[];
  totalCount: number;
  loading: boolean;
  filter: SceneFilter;
  setFilter: (filter: SceneFilter) => void;
  fetchScenes: () => Promise<void>;
}
```

## Key Differences from stash

| Aspect | stash | rustash |
|--------|-------|---------|
| Communication | GraphQL over HTTP | Tauri IPC |
| State Management | Apollo Client cache | Zustand |
| UI Framework | Bootstrap | TailwindCSS + Headless UI |
| Desktop Integration | Systray + browser | Tauri window |
| Plugin Runtime | goja (JS) | WASM (future) |
| Database Access | go-sqlite3 (CGO) | rusqlite (bundled) |
| Streaming | HLS via FFmpeg | Same (incremental) |
