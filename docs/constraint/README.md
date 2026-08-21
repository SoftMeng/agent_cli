# 约束文档索引

本目录存放 `agent_cli` 项目的工程级约束。CLAUDE.md 引用本目录的文档以保持简洁。

| 文档 | 范围 | 优先级 |
|------|------|--------|
| [naming.md](./naming.md) | 标识符命名、文件命名、配置键 | ⚠️ 软约束 |
| [structure.md](./structure.md) | 目录结构、模块边界、trait 端口 | 🔒 强制 |
| [comments.md](./comments.md) | 注释使用规范 | 🔒 强制 |
| [no-test.md](./no-test.md) | No TestCase 硬规则 | 🔒 强制 |
| [quality.md](./quality.md) | 提交前自检与质量门控 | ✅ 工具 |

## 核心原则

1. **单一职责**：一个模块/组件只做一件事。
2. **模块化**：跨层只通过显式端口（trait / 函数）通信。
3. **组件化**：同层组件可独立替换，状态由构造期注入。
4. **依赖最新 Rig**：所有 Rig 相关 API 以 `rig = "0.40.0"` 为准；通过 `rig-core` 的公开 API 集成。
