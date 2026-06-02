# Rustash 详细任务规划

> 每个 PR 只做一件事，确保编译通过、无 bug。PR 编号在实施时分配。

## Phase 0: 基础设施搭建

| PR | 任务 | 描述 | 涉及文件 | 依赖 |
|----|------|------|----------|------|
| P0-1 | 添加 TailwindCSS | 安装 tailwindcss v4 + @tailwindcss/vite，配置 vite.config.ts，创建基础 CSS | vite.config.ts, src/index.css | 无 |
| P0-2 | 前端依赖安装 | 安装 react-router-dom, zustand, @headlessui/react, @heroicons/react, i18next, react-i18next | package.json | 无 |
| P0-3 | Rust 核心依赖 | 添加 rusqlite(bundled), serde, serde_json, directories, tokio 到 Cargo.toml | src-tauri/Cargo.toml | 无 |
| P0-4 | Tauri 权限配置 | 配置 capabilities/default.json 允许必要的 API 权限 | src-tauri/capabilities/ | P0-3 |
| P0-5 | 项目目录结构 | 创建前端 src/ 子目录(components, hooks, stores, pages, utils, types, locales) 和后端 src-tauri/src/ 子目录(commands, db, models, services, config) | 目录创建 | P0-2, P0-3 |
| P0-6 | TypeScript 配置优化 | 配置路径别名(@/ 等)，严格模式 | tsconfig.json, vite.config.ts | P0-2 |

## Phase 1: 前端骨架

| PR | 任务 | 描述 | 涉及文件 | 依赖 |
|----|------|------|----------|------|
| P1-1 | 布局组件 | MainLayout(Sidebar + Content + Header)，使用 TailwindCSS | src/components/layout/ | P0-1 |
| P1-2 | 侧边栏导航 | Sidebar 组件，含导航链接(Scenes, Performers, Studios, Tags, Galleries, Settings) | src/components/layout/Sidebar.tsx | P1-1 |
| P1-3 | 路由配置 | React Router 配置，基础路由表(各页面占位组件) | src/App.tsx, src/pages/ | P1-2 |
| P1-4 | 主题系统 | 深色/浅色主题切换，CSS 变量 + TailwindCSS dark mode | src/hooks/useTheme.ts, src/components/layout/ | P1-1 |
| P1-5 | 国际化基础 | i18next 配置，中英文语言包骨架 | src/locales/ | P0-2 |
| P1-6 | 状态管理 | Zustand store 基础：settingsStore, uiStore | src/stores/ | P0-2 |

## Phase 2: 数据库层

| PR | 任务 | 描述 | 涉及文件 | 依赖 |
|----|------|------|----------|------|
| P2-1 | 数据库初始化 | SQLite 连接管理、migration 框架、schema v1 | src-tauri/src/db/ | P0-3 |
| P2-2 | Schema: files 表 | files/folders 表，文件系统元数据 | src-tauri/src/db/migrations/ | P2-1 |
| P2-3 | Schema: scenes 表 | scenes 表 + scene_markers 表 | src-tauri/src/db/migrations/ | P2-2 |
| P2-4 | Schema: performers 表 | performers 表 | src-tauri/src/db/migrations/ | P2-2 |
| P2-5 | Schema: studios 表 | studios 表(自引用 parent_studio) | src-tauri/src/db/migrations/ | P2-2 |
| P2-6 | Schema: tags 表 | tags 表 + scenes_tags 关联表 | src-tauri/src/db/migrations/ | P2-2 |
| P2-7 | Schema: galleries 表 | galleries + gallery_chapters + images 表 | src-tauri/src/db/migrations/ | P2-2 |
| P2-8 | Schema: groups 表 | groups(原 movies)表 + scenes_groups 关联 | src-tauri/src/db/migrations/ | P2-2 |
| P2-9 | Schema: saved_filters + config | saved_filters 表 + config KV 表 | src-tauri/src/db/migrations/ | P2-2 |
| P2-10 | Rust Models: 基础 | File, Folder, Scene Rust 结构体 + serde | src-tauri/src/models/ | P2-2, P2-3 |
| P2-11 | Rust Models: 扩展 | Performer, Studio, Tag, Gallery, Image, Group 结构体 | src-tauri/src/models/ | P2-4~P2-8, P2-10 |
| P2-12 | DB CRUD traits | 定义通用的 Repository trait + 各实体的 Read/Write trait | src-tauri/src/db/ | P2-10, P2-11 |
| P2-13 | DB CRUD 实现: Scene | Scene 的 CRUD 实现 + Tauri Command 暴露 | src-tauri/src/commands/ | P2-12 |

## Phase 3: 核心功能 - 场景管理

| PR | 任务 | 描述 | 涉及文件 | 依赖 |
|----|------|------|----------|------|
| P3-1 | 场景列表页 | SceneList 页面：网格/列表视图切换，分页 | src/pages/Scenes/ | P1-3, P2-13 |
| P3-2 | 场景卡片组件 | SceneCard：缩略图、标题、标签预览 | src/components/Scenes/ | P3-1 |
| P3-3 | 场景详情页 | SceneDetail：视频播放器占位、元数据展示 | src/pages/Scenes/ | P3-1 |
| P3-4 | 场景编辑表单 | SceneEditForm：元数据编辑、标签选择 | src/components/Scenes/ | P3-3 |
| P3-5 | 文件扫描 Command | scan_paths Tauri Command，扫描目录并入库 | src-tauri/src/commands/ | P2-12 |
| P3-6 | 前端扫描 UI | Settings 页面中的扫描配置 + 扫描按钮 + 进度条 | src/pages/Settings/ | P3-5 |
| P3-7 | 搜索/过滤 | Scene 过滤器(Tauri Command + 前端 filter bar) | src/components/List/ | P3-1 |

## Phase 4: 扩展实体

| PR | 任务 | 描述 | 涉及文件 | 依赖 |
|----|------|------|----------|------|
| P4-1 | Performer CRUD | 后端 CRUD + 列表/详情/编辑页 | 全栈 | P2-12 |
| P4-2 | Studio CRUD | 后端 CRUD + 列表/详情/编辑页 | 全栈 | P2-12 |
| P4-3 | Tag CRUD | 后端 CRUD + 列表/详情/编辑页 + 颜色选择 | 全栈 | P2-12 |
| P4-4 | Gallery CRUD | 后端 CRUD + 列表/详情页 + 图片网格 | 全栈 | P2-12 |
| P4-5 | Group CRUD | 后端 CRUD + 列表/详情/编辑页 | 全栈 | P2-12 |
| P4-6 | Image CRUD | 后端 CRUD + 图片浏览 | 全栈 | P2-12 |
| P4-7 | 实体关联 | Scene↔Performer, Scene↔Tag, Scene↔Studio 等关联 | 全栈 | P4-1~P4-3 |

## Phase 5: 高级功能

| PR | 任务 | 描述 | 涉及文件 | 依赖 |
|----|------|------|----------|------|
| P5-1 | 认证系统 | 登录页、JWT 认证、Session 管理 | 全栈 | Phase 3 |
| P5-2 | 配置管理 | YAML 配置读写(替代 stash 的 koanf) | Rust + Settings 页 | Phase 3 |
| P5-3 | 缩略图生成 | FFmpeg 集成，视频缩略图、预览生成 | Rust | P3-5 |
| P5-4 | HLS 视频流 | 视频流式播放，hls.js 集成 | 全栈 | P3-3, P5-3 |
| P5-5 | 抓取器框架 | XPath/JSON/GraphQL 抓取器引擎 | Rust | Phase 4 |
| P5-6 | 抓取器 UI | 抓取器配置 + 执行 UI | 前端 | P5-5 |
| P5-7 | 插件系统 | JS/WASM 插件运行时 | Rust | Phase 4 |
| P5-8 | Auto-tag | 基于文件名的自动标签匹配 | Rust | P4-3 |
| P5-9 | DLNA 服务 | DLNA 流媒体服务 | Rust | P5-4 |
| P5-10 | 导入/导出 | JSON 格式数据导入导出 | 全栈 | Phase 4 |

## Phase 6: 打磨

| PR | 任务 | 描述 | 涉及文件 | 依赖 |
|----|------|------|----------|------|
| P6-1 | 性能优化 | 虚拟滚动、图片懒加载、SQLite 查询优化 | 全栈 | Phase 5 |
| P6-2 | 键盘快捷键 | 快捷键系统 | 前端 | Phase 5 |
| P6-3 | 国际化完善 | 完整中英文翻译 | src/locales/ | P1-5 |
| P6-4 | 测试覆盖 | Rust 单元测试 + 前端组件测试 | 全栈 | Phase 5 |
| P6-5 | 打包发布 | Tauri 构建配置、自动更新、安装包 | src-tauri/ | Phase 6 |

## 实施规范

1. 每个 PR 必须可编译、可运行
2. 每个 PR 的代码变更 ≤ 500 行（特殊情况除外）
3. PR 标题格式：`[P{phase}-{num}] 简短描述`
4. 提交前执行：`cargo clippy && bun run build`
5. 复杂逻辑必须有 Rust 单元测试
6. 参考 stash 代码理解业务逻辑，但不照搬实现
