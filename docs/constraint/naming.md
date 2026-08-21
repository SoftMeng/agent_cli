# 命名规范

## Rust 标识符

| 类别 | 风格 | 示例 |
|------|------|------|
| 模块 | `snake_case` | `agent::provider` |
| 文件名 | 与模块名一致 | `provider.rs` |
| 结构体 / 枚举 | `PascalCase` | `AppConfig`, `LLMProvider` |
| Trait | `PascalCase`，无 `I` 前缀 | `Tool`, `ConversationMemory` |
| 公开方法 | `snake_case` | `load_skill_configs` |
| 私有方法 | `snake_case` | `parse_skill_metadata` |
| 变量 / 函数参数 | `snake_case` | `memory_dir`, `chat_model` |
| 常量 | `SCREAMING_SNAKE_CASE` | `DEFAULT_MAX_TURNS` |
| 静态项 | `SCREAMING_SNAKE_CASE` | `static NAME: &str` |
| 错误类型后缀 | `Error` | `MemoryToolError` |
| 异步函数 | 不加 `async_` 前缀 | `stream_prompt` |

## 工具名称（`Tool::NAME`）

- 全小写下划线：`add_memory`, `run_command`, `read_file`。
- 动词在前：`get_*` / `list_*` / `query_*` / `add_*` / `delete_*`。
- 同一文件内 Tool 实现可重复，但 `NAME` 必须全局唯一。

## 文件 / 目录

- 目录：`snake_case`，按职责命名（不复用单词拼接 `agentbuilder`）。
- 多文件模块：使用目录 + `mod.rs` 形式（如 `tools/file.rs` + `tools/mod.rs`）。
- 配置文件：`config.yaml` / `AGENT_PROMPT.txt` 保持大写作为约定。

## YAML 配置键

- 全 `snake_case`：`skill_dir`, `memory_dir`, `chat_model`。
- 与对应 Rust 结构体字段名一致，便于 `serde` 直接反序列化。

## 命名禁忌

- 单字母变量（除循环计数 `i, j`）。
- 拼音 / 中文拼音混合。
- 缩写过度：`cfg`、`ctx`、`mgr`、`util`、`helper`、`tmp` 仅在 Rust 习惯用法下保留（如 `ctx` 在 trait 形参）。
- 与标准库同名导致遮蔽：`Result`、`Option`、`String`。
