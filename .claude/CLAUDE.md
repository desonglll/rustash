# Rustash 项目指南

## 项目概述

Rustash 是 [stashapp/stash](https://github.com/stashapp/stash) 的重构版本，使用 Tauri v2 + Rust + React + TailwindCSS 技术栈替代原有的 Go + React + Bootstrap 方案。

原始 stash 代码位于 `stash/` 目录（通过符号链接），当前 rustash 是一个全新的 Tauri v2 脚手架项目。

## 技术栈

| 层 | 技术 |
|---|------|
| 桌面框架 | Tauri v2 |
| 后端 | Rust (src-tauri/) |
| 前端 | React 18 + TypeScript |
| 样式 | TailwindCSS v4 |
| 构建 | Vite 6 + bun |
| 数据库 | SQLite (via rusqlite/diesel) |
| API | Tauri Commands (非 GraphQL) |

## 核心原则

1. **最小变更原则**：每个 PR 只做一件事，代码量尽可能少
2. **正确性优先**：每步都确保编译通过、无 bug
3. **渐进式重构**：先搭骨架再填充功能，按层次逐步实现
4. **参考不照搬**：理解 stash 的架构意图，用 Rust 惯用方式重新实现

## 项目结构

```
rustash/
├── stash/                  # 原始 stash 项目（符号链接，仅参考）
├── src/                    # React 前端源码
├── src-tauri/              # Rust 后端源码
│   └── src/
│       ├── main.rs
│       ├── lib.rs
│       ├── commands/       # Tauri commands
│       ├── db/             # 数据库层
│       ├── models/         # 数据模型
│       ├── services/       # 业务逻辑
│       └── config/         # 配置管理
├── docs/                   # 项目文档
│   ├── plan.md             # 详细任务规划
│   └── architecture.md     # 架构设计
└── .claude/
    ├── CLAUDE.md           # 本文件
    └── settings.json       # Claude Code 设置
```

## 开发命令

```bash
bun install              # 安装前端依赖
bun run dev              # 启动开发服务器
bun run tauri dev        # 启动 Tauri 开发模式
bun run build            # 构建前端
bun run tauri build      # 构建生产包
cd src-tauri && cargo test  # 运行 Rust 测试
cd src-tauri && cargo clippy # Rust lint
```

## 重构策略

### 阶段划分

1. **Phase 0: 基础设施** - 项目配置、依赖、构建流程
2. **Phase 1: 前端骨架** - 路由、布局、主题系统
3. **Phase 2: 数据库层** - SQLite schema、migrations、models
4. **Phase 3: 核心功能** - 场景管理（CRUD）、文件扫描
5. **Phase 4: 扩展功能** - 标签、演员、工作室、图库
6. **Phase 5: 高级功能** - 抓取器、插件系统、DLNA
7. **Phase 6: 打磨** - 性能优化、国际化、测试覆盖

### stash 功能模块对照

| stash 模块 | 说明 | 优先级 |
|-----------|------|--------|
| Scene 管理 | 场景/视频 CRUD、浏览、搜索 | P0 |
| File 扫描 | 文件系统扫描、入库 | P0 |
| Performer | 演员管理 | P1 |
| Studio | 工作室管理 | P1 |
| Tag | 标签管理 | P1 |
| Gallery | 图库管理 | P1 |
| Config | 配置管理 | P0 |
| Auth | 认证系统 | P1 |
| Scrapers | 抓取器 | P2 |
| Plugins | 插件系统 | P2 |
| DLNA | 流媒体 | P2 |
| Auto-tag | 自动标签 | P2 |
| FFmpeg | 转码、缩略图 | P1 |
| Identify | 场景识别 | P2 |

### API 设计策略

stash 使用 GraphQL，rustash 使用 Tauri Commands。对照关系：

- GraphQL Query → Tauri Command (只读)
- GraphQL Mutation → Tauri Command (写操作)
- GraphQL Subscription → Tauri Event (事件推送)

命名规则：`#[tauri::command]` 函数使用 snake_case，前端调用使用 camelCase（Tauri 自动转换）。

## 前端架构（参考 stash 重设计）

- 路由：React Router v6（与 stash 一致）
- 状态管理：Zustand（轻量，替代 Apollo Client 的本地缓存）
- UI 组件：Headless UI + TailwindCSS（替代 Bootstrap + react-bootstrap）
- 视频播放：video.js 或 hls.js
- 国际化：i18next（与 stash 一致）

## 注意事项

- `stash/` 目录是符号链接，只读参考，不要修改
- 每次提交前确保 `cargo clippy` 和 `bun run build` 都通过
- Tauri v2 的权限系统需要在 capabilities/ 中配置
- SQLite 在 Tauri 中需要使用 bundled feature 编译
