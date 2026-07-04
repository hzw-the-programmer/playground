# 一、作用

`RETURNING` 是 `INSERT` / `UPDATE` / `DELETE` 专属子句，执行修改语句后直接返回本次操作的行数据，不用额外再 `SELECT` 查询，大幅减少数据库交互。
类比：SQLx 中 `query!` 配合 `RETURNING` 可以拿到新增 / 修改后的完整字段、自增 ID、变更前后数据。

# 二、基础语法

```sql
-- 通用格式
INSERT INTO 表(字段) VALUES(...) RETURNING 输出列;
UPDATE 表 SET ... WHERE ... RETURNING 输出列;
DELETE FROM 表 WHERE ... RETURNING 输出列;
```

**三种输出写法**

1. 返回指定字段：`RETURNING id, username, create_at`
2. 返回整行所有字段：`RETURNING *`
3. 常量 / 表达式：`RETURNING id, now() as update_time`

# 三、分场景示例（配合 sqlx 使用）

## 1. INSERT + RETURNING（最常用，获取自增主键）

表结构：

```sql
CREATE TABLE users (
    id BIGSERIAL PRIMARY KEY,
    username TEXT NOT NULL,
    email TEXT,
    age INT,
    create_at TIMESTAMPTZ DEFAULT NOW()
);
```

### 只返回自增 ID

```sql
INSERT INTO users (username, email)
VALUES ('rust', 'rust@demo.com')
RETURNING id;
```

sqlx 使用：

```rust
let row = sqlx::query!(
    r#"
        INSERT INTO users (username, email)
        VALUES ($1, $2)
        RETURNING id
    "#,
    "rust",
    "rust@demo.com"
)
.fetch_one(&pool)
.await?;
println!("新增用户ID: {}", row.id);
```

### 返回全部字段 RETURNING *

```sql
INSERT INTO users (username, email)
VALUES ('test', 'test@demo.com')
RETURNING *;
```

```rust
let user = sqlx::query!(
    r#"
        INSERT INTO users (username, email)
        VALUES ($1, $2)
        RETURNING *
    "#,
    "test",
    "test@demo.com"
)
.fetch_one(&pool)
.await?;
// 直接访问所有字段
println!("{} {} {}", user.id, user.username, user.create_at);
```

## 2. UPDATE + RETURNING：返回修改后的数据

更新后拿到最新字段值，不用二次查询：

```sql
UPDATE users
SET age = 30
WHERE id = 1
RETURNING id, username, age;
```

```rust
let updated = sqlx::query!(
    r#"
        UPDATE users
        SET age = $1
        WHERE id = $2
        RETURNING id, username, age
    "#,
    30,
    1i64
)
.fetch_optional(&pool)
.await?;
if let Some(u) = updated {
    println!("更新后年龄: {}", u.age);
}
```

## 3. DELETE + RETURNING：获取被删除的数据

删除同时拿到被删掉的行，用于日志、软删除替代方案：

```sql
DELETE FROM users
WHERE id = 1
RETURNING *;
```

```rust
let deleted_user = sqlx::query!(
    "DELETE FROM users WHERE id = $1 RETURNING *",
    1i64
)
.fetch_optional(&pool)
.await?;
```

# 四、高级特性

## 1. 批量插入多行，全部返回

```sql
INSERT INTO users (username)
VALUES ('a'), ('b'), ('c')
RETURNING id, username;
```

sqlx `fetch_all` 会拿到全部插入记录：

```rust
let list = sqlx::query!(
    r#"
        INSERT INTO users (username)
        VALUES ('a'), ('b'), ('c')
        RETURNING id, username
    "#
)
.fetch_all(&pool)
.await?;
```

## 2. 配合 CTE 复杂操作返回

```sql
WITH new_row AS (
    INSERT INTO users(username) VALUES ('cte') RETURNING id
)
SELECT id FROM new_row;
```

## 3. 返回表达式、别名

```sql
INSERT INTO users(username) VALUES ('alias')
RETURNING id, concat(username, '-user') AS full_name, now();
```

# 五、与 `execute().rows_affected()` 区别

1. 不加 `RETURNING`

```rust
// 仅返回受影响行数，拿不到字段数据
let affect = sqlx::query!("UPDATE users SET age=20")
    .execute(&pool)
    .await?
    .rows_affected();
```

2. 加 `RETURNING`

使用 `fetch_one` / `fetch_all` / `fetch_optional` 获取行数据，也能通过返回列表长度判断影响行数：

```rust
let rows = sqlx::query!("DELETE FROM users RETURNING id")
    .fetch_all(&pool)
    .await?;
let affect_count = rows.len() as u64;
```

# 六、sqlx 关键注意点

1. `query!` / `query_as!` 会根据 `RETURNING` 后的字段自动生成结构体，字段名、类型必须完全匹配；
2. 如果 `RETURNING *`，表结构变更后会触发编译报错，适合强类型约束；
3. 离线模式修改 `RETURNING` 字段后，必须执行 `sqlx prepare` 更新缓存；
4. `timestamptz` 类型返回：
   - sqlx 默认：`time::OffsetDateTime`
   - 开启 chrono feature：`chrono::DateTime<FixedOffset>`，不能直接用 `DateTime<Utc>`

# 七、常见使用场景

1. 新增数据，立刻获取自增主键 ID；
2. 更新记录后无需额外 SELECT，直接读取更新后的字段；
3. 删除数据同时获取被删除内容，用于日志备份；
4. 批量插入后一次性获取所有新增 ID，批量关联其他表；
5. 减少 SQL 往返次数，提升并发性能。

# 八、补充限制

1. `SELECT` / `TRUNCATE` 不支持 `RETURNING`；
2. `TRUNCATE ... CASCADE` 无法返回删除行；
3. DDL（CREATE TABLE/ALTER）无 `RETURNING`；
4. 视图更新 / 删除时，`RETURNING` 只能返回视图可见字段。
