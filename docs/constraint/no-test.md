# No TestCase 硬规则

## 铁律

> **禁止编写任何单元测试、集成测试、E2E 测试代码与测试用例。**

## 不允许出现的形态

- `#[cfg(test)]` 模块
- `#[test]` 标注的函数
- `tests/` 目录及其下任何文件
- `examples/` 作为测试用途（仅允许作为 `cargo run --example` 的可运行示例）
- 任何 `assert_eq!` / `assert!` 出现在业务模块
- `mock`、`stub`、`fake` 命名空间或模块
- 任何 `#[tokio::test]` / `#[async_std::test]`

## 替代做法

| 原测试目标 | 替代方案 |
|-----------|----------|
| 验证 LLM 调用链路 | 启动本地 REPL 手工对话；通过 TUI 观察 |
| 验证 Tool 行为 | 复用 Agent REPL，让模型调用 Tool，观察结果卡片 |
| 验证 Skill 解析 | 在 `preamble` 中让模型列技能；TUI 中显示 |
| 验证 Config 加载 | 启动时报错即修；不再做反序列化单元测试 |
| 验证 Memory | 在 REPL 中多轮对话 + 切换 `conversation_id` |

## Cargo.toml 配置

- 不声明 `[dev-dependencies]`。
- 不声明 `[[test]]` / `[[bench]]` 段。
- `Cargo.lock` 仍正常生成（仅用于二进制构建的可重现性）。

## 例外

无。"快速 smoke 验证" 应在 REPL 启动后由人完成。

## 自检命令

```bash
grep -rn "#\[test\]\|#\[cfg(test)\]\|mod tests" src/ && exit 1
ls tests/ 2>/dev/null && exit 1
echo "OK: no test code"
```

CI 必须执行上述脚本并以非零退出失败。
