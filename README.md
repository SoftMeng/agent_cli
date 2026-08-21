<div align="center">

# agent_cli

**基于 [Rig](https://github.com/0xPlaygrounds/rig) 0.40 的 Rust Agent CLI**
单一职责 · 模块化 · 组件化 · 多卡片 TUI

[![Rust](https://img.shields.io/badge/rust-1.80%2B-orange.svg?style=flat-square&logo=rust)](https://www.rust-lang.org)
[![Edition](https://img.shields.io/badge/edition-2024-blueviolet.svg?style=flat-square)](https://doc.rust-lang.org/edition-guide/)
[![Rig](https://img.shields.io/badge/rig--core-0.40-00ADD8.svg?style=flat-square)](https://github.com/0xPlaygrounds/rig)
[![License](https://img.shields.io/badge/license-MIT%2FApache--2.0-green.svg?style=flat-square)](#license)
[![No Test](https://img.shields.io/badge/policy-No%20TestCase-critical.svg?style=flat-square)](./docs/constraint/no-test.md)

让大模型在终端里 **聊、读、写、跑命令、调用工具**——一张卡片一事件，结构可读，组件可拆。

[快速开始](#-快速开始) · [特性](#-特性) · [架构](#-架构) · [文档](#-文档) · [路线图](./docs/路线图/v0.1.md)

</div>

---

## ✨ 特性

| | |
|---|---|
| 🤖 **多 Provider 对话** | OpenAI / Anthropic 一键切换，统一 `Agent<Provider>` 装配 |
| 🧰 **可插拔工具系统** | 基于 `rig::tool::Tool` + `ToolServer`，文件 / 命令 / 记忆 / 技能四件套 |
| 📚 **文件系统 Skill** | `skills/<name>/SKILL.md` + Frontmatter 自动扫描，按需拼入系统提示词 |
| 🧠 **会话级记忆** | 复用 Rig `InMemoryConversationMemory`，按 `conversation_id` 隔离上下文 |
| 🖥️ **多卡片 TUI** | 基于 `ratatui` 的卡片式会话：对话 / 思考 / 工具调用 / 工具结果独立呈现 |

## 🖼️ TUI 预览

```
┌─ session ─────────────────────────────────────────────────────────────┐
│ 👤 帮我把 src/agent/builder.rs 的 Anthropic 分支也支持 extra_params    │
│                                                                    │
│ 💭 thinking ────────────────────────────────────────────────────────┐│
│ │ 用户希望 Anthropic 路径与 OpenAI 路径具备同等能力                ││
│ └──────────────────────────────────────────────────────────────────┘│
│                                                                    │
│ 🛠 tool_call: read_file                                            │
│    path = src/agent/builder.rs                                      │
│                                                  │ output 1.2 KB    │
│ 🛠 tool_call: edit_file                                            │
│    path = src/agent/builder.rs                                      │
│    diff = +6 / -3                                                   │
│                                                  │ output ok        │
│                                                                    │
│ 🤖 已对齐：在 `AgentKind::Anthropic` 分支补上 `additional_params` 调用 │
└────────────────────────────────────────────────────────────────────┘
 ▌  Enter 提交    Ctrl+C 退出    Ctrl+L 清屏    ↑/↓ 滚动卡片历史
```

## 🏛️ 架构

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

**契约**：
- TUI 只消费 REPL 事件；不直接调用 Agent。
- REPL 持有 Agent / Memory / ToolServer 句柄；不直接读写文件。
- Tools 实现 `rig::tool::Tool`；状态由构造期注入，不持有全局可变状态。
- Skill Registry 仅暴露 `list / get(name)`，由 Prompt 拼接注入 preamble。

## 🚀 快速开始

### 1. 克隆与构建

```bash
git clone https://github.com/SoftMeng/agent_cli.git
cd agent_cli
cargo build --release
```

### 2. 准备配置

```bash
cp config.yaml.example config.yaml
```

编辑 `config.yaml`，填入你的 API Key：

```yaml
skill_dir: ./skills
memory_dir: ./memory
sys_prompt: ./AGENT_PROMPT.txt
llm_provider: openai          # or: anthropic
llm_config:
  api_key: sk-replace-me      # ← 填入真实 key（或用 OPENAI_API_KEY 环境变量）
  base_url: https://dashscope.aliyuncs.com/compatible-mode/v1
  chat_model: qwen3.5-flash
```

> 🔒 **安全提示**：`config.yaml` 已在 `.gitignore` 中被排除。**不要把真实 Key 提交到仓库**。推荐通过环境变量 `OPENAI_API_KEY` / `ANTHROPIC_API_KEY` 注入。

### 3. 运行

```bash
cargo run --release
```

## 🧩 模块一览

```
src/
├── main.rs                    # 程序入口
├── lib.rs                     # crate 根 + 模块声明
├── config/                    # 配置加载与校验
├── agent/                     # Provider + Agent 装配
│   ├── mod.rs
│   ├── provider.rs            # OpenAI / Anthropic 工厂
│   └── builder.rs             # AgentBuilder 装配
├── tools/                     # 工具组件（每文件一个 Tool）
│   ├── mod.rs                 # ToolServer 启动
│   ├── file.rs                # read / write / create / append
│   ├── command.rs             # run_command
│   ├── memory.rs              # add / query / read 记忆工具
│   └── skill.rs               # list / get / download skill 工具
├── memory/                    # ConversationMemory 适配层
├── skill/                     # Skill Registry（文件系统）
├── terminal/                  # ratatui TUI
│   ├── mod.rs
│   ├── cards.rs               # ChatCard / CardType
│   ├── render.rs              # draw_ui / sanitize
│   ├── guard.rs               # TerminalGuard（raw mode 生命周期）
│   └── events.rs              # 按键事件 -> REPL 命令
├── app/                       # REPL / 会话循环
│   ├── mod.rs
│   ├── repl.rs                # 主循环
│   └── session.rs             # Stream 处理 + history 收集
├── preamble.rs                # 系统提示词模板加载 + 技能目录拼接
└── error.rs                   # 顶层错误类型
```

## 📜 自定义 Skill

把任意目录放到 `skills/` 下，每个 Skill 一个子目录：

```
skills/
└── my-skill/
    └── SKILL.md
```

`SKILL.md` 顶部必须是 YAML Frontmatter（仅识别 `name` + `description`）：

```markdown
---
name: my-skill
description: 在回答中追加公司术语表与回复风格约束
---

# My Skill
当用户询问产品问题时，先调取术语表……（正文会作为系统提示词的一部分被注入）
```

启动后 `agent_cli` 会自动扫描并拼接到系统提示词。

## 🛠 技术栈

| 维度 | 选择 | 版本 |
|------|------|------|
| 语言 | Rust | edition = "2024" |
| LLM 框架 | [rig](https://github.com/0xPlaygrounds/rig) | 0.40 |
| 异步运行时 | tokio | 1.x（full） |
| TUI | ratatui | 0.29 |
| 终端事件 | crossterm | 0.28 |
| 序列化 | serde / serde_json / serde_yaml | latest |
| 错误处理 | anyhow + thiserror | latest |
| 时间 | chrono | latest |

## 📐 设计原则

`agent_cli` 不是"功能堆叠"，而是按以下约束逐层生长的工程骨架：

- 🔒 **No TestCase** — 项目不写 `#[test]`，不引入测试框架（详见 [`docs/constraint/no-test.md`](./docs/constraint/no-test.md)）。
- 🔒 **Low code comments** — 注释仅以"标签式"分区存在，禁止解释性段落。
- 🔒 **单一职责** — 一个模块只做一件事；`impl` 块不超过 200 行。
- 🔒 **模块化 + 组件化** — 跨层通过 trait / 显式端口注入。
- ⚠️ **依赖 Rig 公开 API** — 不修改 `rig` 自身代码，所有扩展通过 trait / 适配器完成。

## 📚 文档

| 主题 | 入口 |
|------|------|
| 约束索引 | [docs/constraint/README.md](./docs/constraint/README.md) |
| 命名规范 | [docs/constraint/naming.md](./docs/constraint/naming.md) |
| 模块结构 | [docs/constraint/structure.md](./docs/constraint/structure.md) |
| 注释规范 | [docs/constraint/comments.md](./docs/constraint/comments.md) |
| 质量门控 | [docs/constraint/quality.md](./docs/constraint/quality.md) |
| v0.1 路线图 | [docs/路线图/v0.1.md](./docs/路线图/v0.1.md) |

## 🗺️ 路线图

- [x] 双 Provider 装配（OpenAI / Anthropic）
- [x] 文件系统 Skill Registry
- [x] 多卡片 ratatui TUI
- [ ] 流式响应卡片化
- [ ] Memory 落盘（持久化）
- [ ] 多 conversation_id 切换
- [ ] Plugin 形式的第三方 Tool 接入

完整计划见 [`docs/路线图/v0.1.md`](./docs/路线图/v0.1.md)。

## 🤝 贡献

欢迎 PR。在动手前请先读：

1. [CLAUDE.md](./CLAUDE.md) — 项目规约总览
2. [docs/constraint/](./docs/constraint/README.md) — 硬规则与软约束

本地自检：

```bash
cargo fmt --all
cargo clippy --workspace --all-targets -- -D warnings
cargo check
```

> ⛔ **禁止执行 `cargo test`** —— 项目无测试代码，这是设计选择而非疏忽。

## 📄 License

本项目采用 **MIT OR Apache-2.0** 双许可 —— 详见仓库根目录的 `LICENSE` 文件。

## 🙏 致谢

- [0xPlaygrounds/rig](https://github.com/0xPlaygrounds/rig) — 提供核心 Agent / Tool / Memory 抽象
- [ratatui](https://github.com/ratatui/ratatui) — TUI 渲染基石
- [crossterm](https://github.com/crossterm-rs/crossterm) — 跨平台终端事件

---

<div align="center">

如果这个项目对你有帮助，欢迎 ⭐ Star 让更多人看到。

<sub>Made with ❤️ in Rust</sub>

</div>