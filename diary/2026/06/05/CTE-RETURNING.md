```sql
with inserted_post as (
    insert into post(user_id, content)
    values ($1, $2)
    returning post_id, user_id, content, created_at
)
select post_id, username, content, created_at
from inserted_post
inner join "user" using (user_id)
```

# 整体功能

往 `post` 表插入一条新帖子，然后立即返回这条帖子的基本信息，同时带上发帖用户的用户名。

相当于把 `INSERT` 和 `SELECT ... JOIN` 合并成一次数据库请求，常用于后端 API 创建资源后直接返回完整数据。

# 逐步详解

## 1. 公共表表达式 (CTE) 部分

```sql
with inserted_post as (
    insert into post(user_id, content)
    values ($1, $2)
    returning post_id, user_id, content, created_at
)
```

- `WITH inserted_post AS (...)`

  定义了一个名为 `inserted_post` 的公共表表达式（CTE），可以把它理解为本次查询内的一个临时结果集（类似临时视图），只在这条 SQL 语句中有效。

- `INSERT INTO post(user_id, content) VALUES ($1, $2)`

  向 `post` 表插入一行数据。`$1`, `$2` 是参数占位符，分别代表用户 ID 和帖子内容（具体值由应用层传入）。

- `RETURNING post_id, user_id, content, created_at`

  `RETURNING` 子句让数据库在插入完成后 直接返回 刚刚插入的行的指定列，而不需要再执行一次 `SELECT`。
  这里返回了 `post_id`（主键）、`user_id`、`content` 以及数据库自动生成的 `created_at` 时间戳。
  返回的这行数据就构成了 `inserted_post` 这个 CTE 的全部内容（只有一行）。

> 注意：`RETURNING` 是 PostgreSQL 的特性，其他数据库（如 SQL Server 的 `OUTPUT`，Oracle 的 `RETURNING INTO`）也有类似功能，但语法可能不同。

## 2. 主查询部分

```sql
select post_id, username, content, created_at
from inserted_post
inner join "user" using (user_id)
```

- `FROM inserted_post`

  主查询的起点就是上面 CTE 刚插入的那一行数据。

- `INNER JOIN "user" USING (user_id)`

  将 `inserted_post` 这一行与 `"user"` 表进行内连接，连接条件是两边的 `user_id` 相等。

  + `"user"` 用双引号括起来，通常是因为 `user` 是 SQL 保留字，或者表名是大小写敏感的形式（如建表时用了引号），需要用双引号来正确引用。

  + `USING (user_id)` 等价于 `ON inserted_post.user_id = "user".user_id`，但有一个区别：结果集中只会保留一个 `user_id` 列，且 `SELECT *` 时会把该列放在最前面。这里由于我们手动指定了列，所以不影响。

- `SELECT post_id, username, content, created_at`

  从连接后的结果中选取需要返回的字段：

  + `post_id`：来自 `inserted_post`（即刚插入的帖子 ID）

  + `username`：来自 `"user"` 表，对应的用户名

  + `content`：帖子内容

  + `created_at`：帖子创建时间

## 3. 整体执行流程

1. 执行 `INSERT INTO post ... RETURNING ...`，插入新帖子并得到带有 `post_id`, `user_id`, `content`, `created_at` 的一行记录，作为 `inserted_post`。

2. 用 `inserted_post` 中的 `user_id` 去 `"user"` 表查找对应的用户行（内连接）。

3. 返回一条结果：帖子 ID、用户名、内容、创建时间。

最终返回结果示例（假设插入成功）：

post_id	username	content	created_at
42	alice	Hello, world!	2026-06-05 12:34:56.789012

# 为什么这样写？

## 优点

- 一次往返：插入 + 关联查询合并为一条语句，减少客户端与数据库的交互次数。

- 原子性：在同一个语句中完成，不会出现插入成功但查询失败而需要重试的情况。

- 代码简洁：不需要先用一条语句插入，获取 `post_id`，再用另一条语句去联表查询。

- 利用 `RETURNING` 获取生成列：如果 `post_id` 是自增主键，`created_at` 有默认值，插入后能直接拿到这些值，非常方便。

## 必须注意的点

- 内连接的风险：这里用的是 `INNER JOIN`，如果 `$1` 传入的 `user_id` 在 `"user"` 表中不存在，连接就会失败，整个查询返回 0 行（但插入操作已经成功并提交了！）。

- 一般来说，`user_id` 应该有外键约束，确保插入时 `user_id` 已经存在于 `"user"` 表，否则插入就会报错，所以内连接失败的情况基本不会发生。但理论上，如果外键缺失且用户被并发删除，就可能出现插入成功但返回空结果的问题。可用 `LEFT JOIN` 并根据业务需求处理。

- 表名引号：`"user"` 的双引号是 PostgreSQL 的定界标识符，表明表名就是 `user`（可能建表时用了 `CREATE TABLE "user"`）。在大多数数据库中，不加引号的 `user` 是保留关键字，直接使用会报错，所以必须用引号。

# 等价替代写法（若不使用 CTE）

如果数据库不支持 `RETURNING` 或 `CTE`，通常需要分两步：

```sql
-- 第一步：插入
INSERT INTO post(user_id, content) VALUES ($1, $2);
-- 第二步：查询（假设 post_id 是自增主键，需要获取最后插入的 id）
SELECT p.post_id, u.username, p.content, p.created_at
FROM post p
JOIN "user" u ON p.user_id = u.user_id
WHERE p.post_id = last_insert_rowid();  -- 或 SCOPE_IDENTITY() 等
```

相比起来，CTE + `RETURNING` 的方案更干净、更安全（无需依赖会话级的上次插入 ID 函数）。

# 总结

这段 SQL 的作用就是：插入一条帖子，并立即返回这条帖子的 id、作者用户名、内容和创建时间。它利用了 CTE 和 RETURNING 将插入与联表查询完美组合成一条原子 SQL，非常实用。
