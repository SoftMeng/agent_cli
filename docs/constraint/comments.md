# 注释规范（Low code comments）

## 铁律

- 默认 **不写注释**。
- 仅允许以下三类注释，超出范围一律删除。

## 允许的注释

### 1. 模块/分区标签（Tag-form）

```rust
// ============ Section ============
```

- 用于长文件中标识逻辑段落。
- 不附带任何解释文字。
- 段落标题 ≤ 4 个英文单词，PascalCase。

### 2. TODO / SAFETY 警告

```rust
// TODO: 处理 Anthropic 流式 tool_use 边界
// SAFETY: 同一线程持有 RwLock 读锁，无并发风险
```

- 必须可执行：留 TODO 时同步在 issue / PR 描述里建任务。
- `SAFETY` 必须紧跟 unsafe 块，并说明为什么 unsafe 安全。

### 3. 类型 / Trait 简述（单行 doc）

```rust
/// ConversationMemory 后端的进程内实现。
pub struct InMemoryConversationMemory { /* ... */ }
```

- 仅说明"做什么"，不解释"为什么"和"如何工作"。
- 不超过一行，不写示例代码块。
- 公共 API 一行 doc 即可，不写长篇文档。

## 禁止的注释

- 解释意图、列举实现步骤。
- 引用 issue / PR 编号。
- 注释掉的旧代码。
- 函数前的"参数说明"重复签名已表达的信息。
- 改动历史（属于 git log，不属于源码）。

## 自检

提交前 grep 以下模式，命中即改：

```bash
# 排查多行注释
grep -rn "^[[:space:]]*//" src/ | grep -v "TODO:\|SAFETY:\|// ====="

# 排查文档注释过长（> 1 行）
grep -rn "^[[:space:]]*///" src/ -A 2 | grep -v "^--$" | awk '/^$/{ln=0;next} {ln++; if(ln>3) print FILENAME":"NR": "$0}'
```

> 本项目不需要 lint 工具强制；人工 review 时执行。
