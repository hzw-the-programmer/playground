`ReadyForQuery` 是 PostgreSQL 前端/后端协议中一个非常重要的服务端消息，它标志着后端已处理完当前请求，重新回到了“空闲”状态，前端现在可以安全地发送下一个查询。

---

# 1. 消息类型标识

- **消息码**：`'Z'`（ASCII 90）

- **含义**：后端已准备好接受新的查询。

---

# 2. 消息格式


字节范围	内容	说明
第1字节	`'Z'`	消息类型标识
第2-5字节	`Length` (Int32)	消息体长度，值为 5（包含自身4字节 + 事务状态1字节）
第6字节	`Transaction Status`	当前事务状态，1字节

因此，一条完整的 `ReadyForQuery` 消息总是固定 6 字节：

```text
5A 00 00 00 05 [Transaction Status]
```

---

# 3. 事务状态（Transaction Status）

该字段指示在发送此消息后，连接处于何种事务状态。

状态码	含义	说明
`'I'`	**Idle**	空闲，不在事务块中。例如刚连接或执行了 `COMMIT`/`ROLLBACK` 后。
`'T'`	**In Transaction**	处在事务块中。通常在 `BEGIN` 之后，未提交前，每个简单查询后都会得到此状态。
`'E'`	**Failed Transaction**	处在失败的事务块中。因某个错误导致事务已不可提交，必须执行 `ROLLBACK` 才能回到 `'I'` 状态。此后直到回滚前，除 `ROLLBACK` 外的任何命令都将被忽略并返回错误。

---

# 4. 在简单查询协议中的行为

在**简单查询**（Simple Query，通过 `Query` 消息发送字符串 SQL）中：

- 客户端发送一个包含若干 SQL 语句的字符串。

- 服务端逐条解析并执行，为每条正常结束的命令发送 `CommandComplete`（或 `EmptyQueryResponse`）。

- 若某条命令出错，会立即发送 `ErrorResponse`，并跳过后续语句。

- **当整个查询字符串处理完毕（或遇到错误后）**，服务端发送一条 `ReadyForQuery`，附上当前的事务状态。

## 流程示意（成功执行两条 SELECT）：

```text
客户端 -> Query("SELECT 1; SELECT 2;")
服务端 -> RowDescription... DataRow... CommandComplete("SELECT 1")
        -> RowDescription... DataRow... CommandComplete("SELECT 2")
        -> ReadyForQuery('I')    // 不在事务中
```

---

# 5. 在扩展查询协议中的行为

在**扩展查询**（Extended Query，Parse/Bind/Execute/Describe/Sync 等）中：

- 服务端处理 `Parse`、`Bind`、`Describe` 时不会立即发送完毕通知。

- `Execute` 完成后发送 `CommandComplete`。

- 只有接收到 `Sync` 消息后，服务端才会完成所有挂起的操作，清空未使用的门户，并最终发送 `ReadyForQuery`。

## 流程示意：

```text
客户端 -> Parse, Bind, Execute, Sync
服务端 -> ParseComplete, BindComplete, CommandComplete, ReadyForQuery('T')
```

如果过程中出错，服务端会跳过剩余消息直到 `Sync`，然后发送 `ErrorResponse` 和 `ReadyForQuery('E')`。

---

# 6. 与其他消息的关系及注意事项

- **流水线处理**：PostgreSQL 允许流水线，前端可以连续发送多个查询而不等待 `ReadyForQuery`。服务端必须按照发送顺序依次返回每个查询的完整结果集，每条 `ReadyForQuery` 对应一个请求。状态码表示发送该消息时的事务状态。

- **COPY 模式**：在 `COPY` 操作期间，协议进入特殊的子协议。只有当 `COPY` 完成后（收到 `CopyDone` 或 `CopyFail`），才会发送 `ReadyForQuery`。

- **通知与警告**：`NoticeResponse` 和 `NotificationResponse` 可能随时到达，但不会改变 `ReadyForQuery` 到来的时机。

- **连接初始**：认证成功后，服务端会发送 `ReadyForQuery`，此时状态为 `'I'`。这是连接建立完成、可以接受命令的标志。

- **事务状态的精确性**：状态码反映的是该消息发送后立即的状态。例如，`COMMIT` 命令会先返回 `CommandComplete("COMMIT")`，然后 `ReadyForQuery('I')`；`ROLLBACK` 类似。

- **错误状态转换**：如果状态是 `'E'`，大部分 SQL（除 `ROLLBACK` 或特定事务控制外）会被直接拒绝并返回错误 `“current transaction is aborted...”`，直到执行 `ROLLBACK` 使事务终止，状态才变回 `'I'`。

---

# 7. 实例演示

假设执行 `BEGIN; SELECT 1/0; SELECT 2; COMMIT;`

```text
Query("BEGIN")
→ CommandComplete("BEGIN")
→ ReadyForQuery('T')      // 进入事务

Query("SELECT 1/0; SELECT 2;")
→ ErrorResponse("division by zero")
→ ReadyForQuery('E')      // 事务失败，后续 SELECT 2 未执行

Query("COMMIT")
→ ErrorResponse("current transaction is aborted...")
→ ReadyForQuery('E')      // 状态仍为 E，COMMIT 被拒绝

Query("ROLLBACK")
→ CommandComplete("ROLLBACK")
→ ReadyForQuery('I')      // 回到空闲
```

---

# 8. 驱动实现要点

如果你正在编写驱动或客户端库，需要注意：

- 在收到 `ReadyForQuery` 之前，不能将该连接用于新的查询（除非使用流水线，但响应顺序必须严格对应）。

- 根据返回的事务状态码，更新本地记录的连接状态，以避免向 `'E'` 状态的连接发送无效 SQL。

- 简单查询模式下，一个 `Query` 消息一定对应一个 `ReadyForQuery`；扩展查询模式下，一个 `Sync` 对应一个 `ReadyForQuery`。

- 状态码 `'E'` 不会自动恢复，必须显式执行 `ROLLBACK`。
