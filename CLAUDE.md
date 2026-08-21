# CLAUDE.md

This file provides guidance to Claude Code when working with the `agent_cli` project.

## 项目概述

`agent_cli` 是基于 [Rig](https://github.com/0xPlaygrounds/rig) 0.40 构建的交互式 Agent CLI。
目标是用单一职责、模块化、组件化的方式生产一份**可读、可改造、可演进**的 Agent 工程骨架，覆盖：

- **大模型对话**：OpenAI / Anthropic 双 Provider，统一 `Agent<Provider>` 装配
- **工具调用**：基于 `rig::tool::Tool` trait + `ToolServer` 的多工具注册
- **技能系统**：文件系统 Skill（`skills/<name>/SKILL.md` + Frontmatter）
- **记忆系统**：Rig 内置 `InMemoryConversationMemory`（按 `conversation_id` 隔离会话上下文）

运行形态为基于 `ratatui` 的多卡片 TUI：会话卡片、思考片段、工具调用与工具结果均以独立卡片呈现。

## 约束层级

| 标记 | 含义 | 处置 |
|------|------|------|
| 🔒 强制 | 不可违反 | `cargo check` / `cargo build` 失败即返工 |
| ⚠️ 软约束 | 强烈建议 | 偏离需在 PR 描述中说明理由 |
| ✅ 工具 | 由工具自动校验 | 编辑器/CI 拦截，无需人工判断 |

硬规则（不可议）：

1. **No TestCase**：禁止编写 `#[cfg(test)]` 模块、单测、集成测试代码。
2. **Low code comments**：注释仅以"标签式"为主（章节标题、模块分组标记），禁止解释性段落。
3. **单一职责**：一个模块/组件只负责一个清晰的内聚职责；超过 200 行的 `impl` 块需拆分。
4. **模块化 + 组件化**：目录按职责分层，跨层通过 trait 或显式端口（port）注入。
5. **依赖 Rig 最新 API**：以 `rig = "0.40.0"` 为准；通过 `rig-core` 的 `AgentBuilder` / `Tool` / `ToolServer` / `ConversationMemory` 装配。

## 快速命令

```bash
# 构建与运行
cargo check
cargo build --release
cargo run --release

# 格式化与 Lint（建议在 CI 前本地跑通）
cargo fmt --all
cargo clippy --workspace --all-targets -- -D warnings
```

> 禁止执行 `cargo test` —— 项目无测试代码。

## 技术栈

| 维度 | 选择 | 版本 |
|------|------|------|
| 语言 | Rust | edition = "2024" |
| LLM 框架 | rig | 0.40.0 |
| 异步运行时 | tokio | 1.x（features: macros, rt-multi-thread, full） |
| TUI | ratatui | 最新稳定 |
| 终端事件 | crossterm | 最新稳定 |
| 序列化 | serde / serde_json / serde_yaml | 最新稳定 |
| 错误处理 | anyhow + thiserror | 最新稳定 |
| 时间 | chrono | 最新稳定 |
| Markdown 渲染 | tui-markdown | 最新稳定 |

## 架构概览

```
┌─────────────────────────────────────────────────────────┐
│                       TUI 层                            │
│   terminal::{cards, render, guard, events}              │
└─────────────────────────────────────────────────────────┘
                          │
                          ▼
┌─────────────────────────────────────────────────────────┐
│                     REPL / 事件循环                       │
│                app::repl / app::session                  │
└─────────────────────────────────────────────────────────┘
                          │
        ┌─────────────────┼─────────────────┐
        ▼                 ▼                 ▼
┌──────────────┐  ┌──────────────┐  ┌──────────────┐
│  Agent 装配   │  │  Tool Server │  │   Memory     │
│ agent::build │  │ tools::*     │  │ memory::*    │
│ + Provider   │  │  (Tool trait)│  │ (Rig 内置)   │
└──────────────┘  └──────────────┘  └──────────────┘
        │
        ▼
┌─────────────────────────────────────────────────────────┐
│   Skill Registry（文件系统 SKILL.md 扫描 + 解析）         │
│                       skill::*                           │
└─────────────────────────────────────────────────────────┘
                          │
                          ▼
┌─────────────────────────────────────────────────────────┐
│                  Config（YAML + 路径解析）                │
│                       config::*                          │
└─────────────────────────────────────────────────────────┘
```

每一层的核心契约：

- **TUI** 只消费来自 REPL 的事件；不直接调用 Agent。
- **REPL** 拥有 Agent 句柄、Memory 句柄、Tool Server Handle；不直接读写文件。
- **Agent 装配** 接收 `Provider` + `ToolServerHandle` + `ConversationMemory`，返回 `Agent<M, P>`。
- **Tools** 实现 `rig::tool::Tool`；每个工具一个组件，状态由构造期注入（不持有全局可变状态）。
- **Memory** 仅暴露 `load / append` 两个动词；后端切换由 `Arc<dyn ConversationMemory>` 决定。
- **Skill Registry** 仅暴露 `list / get(name)`；不向 Agent 注册，由 Prompt 拼接注入 preamble。

## 项目结构

```
agent_cli/
├── Cargo.toml
├── CLAUDE.md
├── README.md
├── docs/
│   └── constraint/
│       ├── README.md
│       ├── naming.md
│       ├── structure.md
│       ├── comments.md
│       ├── no-test.md
│       └── quality.md
├── src/
│   ├── main.rs                    # 程序入口
│   ├── lib.rs                     # crate 根 + 模块声明
│   ├── config/                    # 配置加载与校验
│   ├── agent/                     # Provider + Agent 装配
│   │   ├── mod.rs
│   │   ├── provider.rs            # OpenAI / Anthropic 工厂
│   │   └── builder.rs             # AgentBuilder 装配
│   ├── tools/                     # 工具组件（每文件一个 Tool）
│   │   ├── mod.rs                 # ToolServer 启动
│   │   ├── file.rs                # read / write / create / append
│   │   ├── command.rs             # run_command
│   │   ├── memory.rs              # add / query / read 记忆工具
│   │   └── skill.rs               # list / get / download skill 工具
│   ├── memory/                    # ConversationMemory 适配层
│   ├── skill/                     # Skill Registry（文件系统）
│   ├── terminal/                  # ratatui TUI
│   │   ├── mod.rs
│   │   ├── cards.rs               # ChatCard / CardType
│   │   ├── render.rs              # draw_ui / sanitize
│   │   ├── guard.rs               # TerminalGuard（raw mode 生命周期）
│   │   └── events.rs              # 按键事件 -> REPL 命令
│   ├── app/                       # REPL / 会话循环
│   │   ├── mod.rs
│   │   ├── repl.rs                # 主循环
│   │   └── session.rs             # Stream 处理 + history 收集
│   ├── preamble.rs                # 系统提示词模板加载 + 技能目录拼接
│   └── error.rs                   # 顶层错误类型
├── skills/                        # 本地技能目录（每个技能一个子目录 + SKILL.md）
├── memory/                        # （可选）运行时落盘的辅助记忆文件
└── AGENT_PROMPT.txt               # 系统提示词模板
```

## 代码风格

### 命名规范（详见 `docs/constraint/naming.md`）

- 模块：`snake_case`；文件名与模块名一致。
- 类型：`PascalCase`；trait `PascalCase`。
- 常量：`SCREAMING_SNAKE_CASE`。
- 工具名称（在 `Tool::NAME`）：`snake_case`（如 `add_memory`）。
- 配置文件：YAML 用 `snake_case`；字段名与 Rust 结构体字段保持一致。

### 注释规范（详见 `docs/constraint/comments.md`）

- 默认不写注释。
- 仅允许以下三类注释：
  1. **模块/分区标签**：`// ============ Section ============`
  2. **TODO/警示**：`// TODO: <事项>` / `// SAFETY: ...`
  3. **trait/类型简述**：单行 `///` doc，仅说明"做什么"而非"为什么"。
- 禁止：解释意图、列举历史、引用 issue/PR。

### 错误处理

- 顶层 `anyhow::Result<T>`，边界处给出 `with_context`。
- 组件内自定义错误用 `thiserror` 派生，**不要**泛用 `String` 兜底。
- 工具错误用 `Tool::Error`，由 Rig 统一归一化到 `ToolExecutionError`。

## Skill 映射

| 任务 | Skill | 说明 |
|------|-------|------|
| 项目脚手架 | `/harness-init` | 已用于初始化本项目 |
| 设计决策 | `/harness-design` | 模块拆分、trait 设计 |
| Rust 编码 | `/harness-rust-development` | 代码生成 |
| 代码审查 | `/harness-code-review` | 变更审查 |
| 文档结构 | `/harness-doc-design` | CLAUDE.md / constraint 维护 |

## 约束引用

- [docs/constraint/README.md](./docs/constraint/README.md) — 约束索引
- [docs/constraint/naming.md](./docs/constraint/naming.md) — 命名规范
- [docs/constraint/structure.md](./docs/constraint/structure.md) — 目录与模块
- [docs/constraint/comments.md](./docs/constraint/comments.md) — 注释规范
- [docs/constraint/no-test.md](./docs/constraint/no-test.md) — No TestCase 硬规则
- [docs/constraint/quality.md](./docs/constraint/quality.md) — 质量门控

## 开发流程

1. **理解**：读 CLAUDE.md 与 `docs/constraint/`；如涉及 Rig API 不熟，先读 `rig/crates/rig-core/src/` 与 `rig/examples/`。
2. **设计**：跨模块改动先在 PR 描述或简短设计备忘里说清楚 trait 边界。
3. **编码**：单一职责 + 组件化；每文件 < 200 行，超过则拆。
4. **自检**：`cargo check` + `cargo clippy --all-targets -- -D warnings` + `cargo fmt` 全绿。
5. **文档**：触及模块结构变更时同步更新 CLAUDE.md 与 constraint 文档。

## 重要注意事项

- **No TestCase 优先于一切**：永远不要新增 `#[test]` 或测试文件。
- **依赖 Rig 公开 API**：不修改 `rig` 自身代码；所有扩展通过 trait / 适配器完成。
- **Provider 切换**：OpenAI 与 Anthropic 的 Agent 类型不同（`Agent<OpenAICompletionModel, _>` 与 `Agent<AnthropicCompletionModel, _>`），用枚举 + trait object 时务必确认 trait 约束（参见 `rig::completion::CompletionModel`）。
- **Tool 注册路径**：使用 `ToolServer::new().tool(ToolImpl).run()` 返回 `ToolServerHandle`，再 `.tool_server_handle(handle)` 喂给 `AgentBuilder`（不要同时用 `.tool()` + `.tool_server_handle()`，会被 typestate 拒绝）。
- **ConversationMemory**：`InMemoryConversationMemory::new()` 进程内存储；通过 `.memory(memory).default_conversation_id(id)` 接入；每次 `.prompt(...).conversation(id)` 按 id 拉取/追加历史。
- **Skill Frontmatter**：仅识别 `name` + `description` 两个键，其余视为正文。失败需在 preamble 构造阶段显式 `bail!`。

## 初衷
按照单一职责、模块化、组件化的编程思想和设计思想，基于Rig框架编写一个Agent CLI，涵盖了「大模型对话 + 工具调用 + 技能系统 + 记忆系统」功能。
- Rig框架源码：/Users/xiangyuanmeng/Documents/Qoder/rig  （里面有examples）

遵循要求：
1. 功能开发过程中，遵循“No TestCase”原则，即无单元测试代码和测试用例代码。
2. 功能开发过程中，遵循“Low code comments”原则，即极简的代码注释，多数情况会使用标签的形式的注释。
3. 时刻要求AI按照单一职责、模块化、组件化的编程思想和设计思想，高专注的实现生产级别的开发设计和代码编写。
4. 遵循最新版本的Rig的特性，可通过检索Rig框架源码和源码中的examples来学习如何编写。
5. Demo工程源码已经过时了，但是设计思路是可以借鉴的。