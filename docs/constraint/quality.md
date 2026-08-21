# 质量门控

## 提交前自检（人工）

按顺序执行，全部必须成功：

```bash
cargo fmt --all
cargo check --all-targets
cargo clippy --all-targets -- -D warnings
```

> ⚠️ 跳过 `cargo test` —— 本项目不包含测试。

## 质量标准

| 维度 | 标准 |
|------|------|
| 模块大小 | 单文件 ≤ 200 行；超则拆 |
| 函数大小 | 单函数 ≤ 60 行；超则拆 |
| 圈复杂度 | 单函数分支数 ≤ 8 |
| 注释 | 仅标签 / TODO / SAFETY / 单行 doc |
| 错误处理 | 顶层 `anyhow::Result`；组件内 `thiserror` |
| 公共 API | 一行 `///` doc 说明"做什么" |
| 未使用代码 | 当次提交即时清理（无 backward-compat 兜底） |
| 依赖 | 仅新增 `rig` 已用到的 feature；其他依赖需在 PR 描述中说明 |

## 改动边界

- 不重构与本次任务无关的代码。
- 不为了"统一风格"顺手调整其他文件。
- 不引入"未来可能用得上"的抽象。
- 不修改 `rig` 源码；所有扩展通过 trait / 适配器完成。

## 提交规范

- 一次提交只做一件事。
- 提交信息格式：

  ```
  <scope>: <imperative summary>

  - bullet 1
  - bullet 2
  ```

  示例：

  ```
  tools: add AddMemoryTool with ConversationMemory backend

  - inject memory_dir via constructor
  - expose tool result as ToolOutput::text
  ```

## Rig API 变更监控

- 每次 `cargo update` 后检查 `rig` 的 CHANGELOG。
- 若 Rig 升级涉及 `Tool` / `ToolServer` / `AgentBuilder` 破坏性变更，更新本目录文档后再合入。
