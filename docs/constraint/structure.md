# 目录与模块结构

## 分层

```
src/
├── main.rs            # 仅做装配 + 启动
├── lib.rs             # crate 根，pub mod 声明
├── config/            # 配置加载
├── agent/             # Agent 装配（Provider 选择 + AgentBuilder）
├── tools/             # 工具组件集合
├── memory/            # 记忆系统（基于 Rig ConversationMemory）
├── skill/             # Skill 注册中心（文件系统）
├── terminal/          # TUI 渲染
├── app/               # REPL 主循环
├── preamble.rs        # 系统提示词模板
└── error.rs           # 顶层错误类型
```

## 跨层契约

| 上层 | 只能依赖 | 不得依赖 |
|------|----------|----------|
| `main.rs` | `lib.rs` 内所有公开项 | — |
| `terminal/*` | `app::session` 事件 + `ChatCard` | `rig::agent::*`（不允许直接驱动 Agent） |
| `app::*` | `agent`, `tools`, `memory`, `skill`, `terminal` | 任何具体 Provider 类型 |
| `agent/*` | `rig::client`, `rig::completion`, `rig::tool::server` | `terminal/*` |
| `tools/*` | `rig::tool::Tool`, `config::*`, `memory::*`, `skill::*` | `terminal/*`, `app::*` |
| `memory/*` | `rig::memory::ConversationMemory` | 任何 UI / 终端依赖 |
| `skill/*` | `config::*`, 文件系统 | 任何 LLM / 工具依赖 |

## 模块拆分规则

- 单文件 ≤ 200 行；超过则按职责拆为子模块。
- 单 `impl` 块 ≤ 80 行；超过则按方法分组。
- 一个 `mod.rs` 仅做"声明 + 重导出"，不写业务逻辑。

## 组件注入

- 组件通过构造函数注入依赖（如 `AddMemoryTool { memory_dir }`）。
- 不允许在组件内部 `static` 全局可变状态。
- 共享状态（如 `ConversationMemory`）通过 `Arc<dyn ConversationMemory>` 注入。

## Provider 抽象

- 定义 `enum LLMProvider { OpenAI, Anthropic }`（在 `config` 层）。
- `agent::provider` 暴露 `pub enum AnyCompletionModel` 或工厂方法 `build_agent(provider, ...) -> AnyAgent`。
- 调用方不感知具体 Provider；切换 Provider 仅改 `config.yaml`。
