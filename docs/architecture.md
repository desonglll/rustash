# Rustash 架构设计

## 整体架构

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

## 分层职责

### Frontend Layer

| 模块 | 职责 |
|------|------|
| Pages | 路由页面组件 |
| Components | 可复用 UI 组件 |
| Stores | Zustand 全局状态 |
| Hooks | 自定义 React Hooks |
| Utils | 工具函数 |
| Types | TypeScript 类型定义 |
| Locales | 国际化资源 |

### Rust Backend Layer

| 模块 | 职责 |
|------|------|
| commands/ | Tauri Command 处理器（薄层，参数校验 + 调用 service） |
| services/ | 业务逻辑层（事务编排、权限检查） |
| db/ | 数据库访问层（SQLite 连接池、migration、CRUD） |
| models/ | 领域模型（纯数据结构 + serde） |
| config/ | 配置管理（YAML 读写） |

## 数据流

### 读取数据
```
Frontend → invoke("scene_list", filters) → Command → Service → Repo → SQLite → 返回 Vec<Scene>
```

### 写入数据
```
Frontend → invoke("scene_update", data) → Command → Service → Repo → SQLite → 返回 Scene
```

### 事件推送（替代 GraphQL Subscription）
```
Rust Backend → app.emit("scan_progress", payload) → Frontend listen("scan_progress", callback)
```

## 数据库设计

### 核心表（参考 stash schema v85，简化）

```sql
-- 文件系统
files (id, basename, parent_folder_id, size, mod_time, created_at, updated_at)
folders (id, path, parent_folder_id, mod_time, created_at, updated_at)

-- 核心实体
scenes (id, title, details, url, date, rating, organized, studio_id,
        created_at, updated_at)
scene_files (scene_id, file_id)  -- 多对多
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

-- 关联表
scenes_performers (scene_id, performer_id)
scenes_tags (scene_id, tag_id)
scenes_groups (scene_id, group_id)
performers_tags (performer_id, tag_id)
galleries_tags (gallery_id, tag_id)
galleries_performers (gallery_id, performer_id)
galleries_scenes (gallery_id, scene_id)
images_tags (image_id, tag_id)
images_performers (image_id, performer_id)

-- 其他
saved_filters (id, mode, name, filter_json, created_at, updated_at)
config (key TEXT PRIMARY KEY, value TEXT)
```

### Migration 策略

- 使用嵌入式 SQL 文件，按序号命名：`001_initial.sql`, `002_add_xxx.sql`
- 应用启动时检查当前版本并自动执行未应用的 migration
- 版本记录在 `schema_migrations` 表中

## Tauri Commands 设计

### 通用模式

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

### 命名规范

| 操作 | 命名 | 示例 |
|------|------|------|
| 列表 | `{entity}_list` | `scene_list` |
| 单个查询 | `{entity}_find` | `scene_find` |
| 创建 | `{entity}_create` | `scene_create` |
| 更新 | `{entity}_update` | `scene_update` |
| 删除 | `{entity}_destroy` | `scene_destroy` |
| 统计 | `{entity}_count` | `scene_count` |

## 前端组件架构

### 页面路由

```
/                    → FrontPage (仪表盘)
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
// 设置 store
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

// 实体 store (按需)
interface SceneStore {
  scenes: Scene[];
  totalCount: number;
  loading: boolean;
  filter: SceneFilter;
  setFilter: (filter: SceneFilter) => void;
  fetchScenes: () => Promise<void>;
}
```

## 与 stash 的关键差异

| 方面 | stash | rustash |
|------|-------|---------|
| 通信方式 | GraphQL over HTTP | Tauri IPC |
| 状态管理 | Apollo Client cache | Zustand |
| UI 框架 | Bootstrap | TailwindCSS + Headless UI |
| 桌面集成 | Systray + 浏览器 | Tauri 窗口 |
| 插件运行时 | goja (JS) | WASM (未来) |
| 数据库访问 | go-sqlite3 (CGO) | rusqlite (bundled) |
| 流媒体 | HLS via FFmpeg | 同左 (逐步实现) |
