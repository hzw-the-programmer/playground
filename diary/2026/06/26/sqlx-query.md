# 一、基础定义

`sqlx::query!` 是 SQLx 提供的过程宏，编译期校验 SQL 语句，生成静态类型查询代码。

```rust
sqlx::query!(sql_str, param1, param2...)
```

区别于动态 `sqlx::query()`（运行时校验、无类型）：

- `query!`：编译期校验 SQL、字段、参数类型，返回匿名结构体，无运行时 SQL 错误
- `query_as!`：和 `query!` 逻辑一致，但映射到你自己定义的 `#[derive(FromRow)]` 结构体
- `query`：纯动态，仅运行时校验，返回 `Row`，无编译检查

# 二、核心工作机制

1. 编译阶段
   - 有数据库连接：直接连接库执行 SQL 解析，获取表、字段、类型、约束
   - 开启 offline 特性：读取 `sqlx-data.json` 缓存文件离线校验，无需数据库
2. 宏自动生成匿名结构体，每个 SELECT 字段对应结构体字段
3. 占位符 $1 $2 $3 自动匹配传入参数的类型，类型不匹配直接编译报错

# 三、基础使用示例

## 1. 单条查询 fetch_one

```rust
// 执行查询，返回单行匿名结构体
let user = sqlx::query!(
    "SELECT id, username, email FROM users WHERE id = $1",
    1i64 // $1 对应参数
)
.fetch_one(&pool)
.await?;

// 直接访问字段
println!("{} {}", user.id, user.username);
```

## 2. 多行查询 fetch_all

```rust
let user_list = sqlx::query!(
    "SELECT id, username FROM users WHERE age >= $1",
    18i32
)
.fetch_all(&pool)
.await?;
```

## 3. DML 插入 / 更新 / 删除（返回受影响行数 / RETURNING）

### 插入并返回自增 ID

```rust
let res = sqlx::query!(
    "INSERT INTO users(username, email) VALUES ($1, $2) RETURNING id",
    "zhangsan",
    "zhangsan@xxx.com"
)
.fetch_one(&pool)
.await?;
println!("new user id: {}", res.id);
```

### 更新，只获取影响行数

```rust
let row_affected = sqlx::query!(
    "UPDATE users SET age = $1 WHERE id = $2",
    20,
    1
)
.execute(&pool)
.await?
.rows_affected();
```

# 四、返回值类型规则（重点）

## 1. SELECT 有返回字段

`fetch_one` / `fetch_all` 返回自动生成匿名结构体，字段名和 SQL 查询列完全一致。

- 数据库字段带下划线 `create_at` → 结构体字段 `create_at`
- 数据库字段关键字需别名：`SELECT "user" AS user_name FROM users`

## 2. 无返回列（UPDATE/DELETE 无 RETURNING）

`.execute()` 返回 `sqlx::PgQueryResult`，只能用 `.rows_affected()` 获取修改行数，不能用 `fetch_one`/`fetch_all`。

## 3. 可为 NULL 的字段

数据库允许 NULL → 宏自动生成 Option<T> 类型；
非空字段 → 直接 T。

```sql
-- age 允许 null，username NOT NULL
SELECT username, age FROM users
```

```rust
// user.username: String
// user.age: Option<i32>
let user = sqlx::query!("SELECT username, age FROM users LIMIT 1")
    .fetch_one(&pool)
    .await?;
```

# 五、占位符参数规则（PostgreSQL）

PostgreSQL 使用 `$1 $2 $3` 数字占位符，禁止 ?（MySQL/SQLite 才用？）

1. 参数顺序严格对应数字，不可打乱
2. 自动类型校验：数据库字段类型和传入 Rust 类型不匹配直接编译失败
3. 支持字符串、数字、`Option`、`DateTime`、`Vec` 等类型

示例：可选参数传 None

```rust
let age: Option<i32> = None;
sqlx::query!(
    "INSERT INTO users(username, age) VALUES ($1, $2)",
    "test",
    age
)
.execute(&pool)
.await?;
```

# 六、关键特性与配置要求

## 1. Cargo.toml 必须开启的 feature

`macros` 是 `query!` 宏必需；PostgreSQL 需要 `postgres`；异步需要 `runtime-tokio`

```toml
sqlx = {
    version = "0.7",
    default-features = false,
    features = ["postgres", "runtime-tokio", "macros", "offline"]
}
```

## 2. 离线模式 offline

开启 offline 后编译不需要本地数据库，但必须提前执行：

```bash
sqlx prepare
```

生成 `sqlx-data.json`，SQL 修改后必须重新执行，否则编译报错缓存过期。

# 七、`query!` vs `query_as!` 核心区别

宏	返回类型	使用场景
`query!`	自动生成匿名结构体	简单查询，不想手动定义结构体
`query_as!(T)`	你自己定义的 `T: FromRow`	复用结构体、多接口共用同一张表映射

示例对比：

```rust
// query! 匿名结构体
let u_anon = sqlx::query!("SELECT id,username FROM users LIMIT 1").fetch_one(&pool).await?;

// query_as! 自定义 User 结构体
#[derive(sqlx::FromRow)]
struct User { id: i64, username: String }
let u: User = sqlx::query_as!(User, "SELECT id,username FROM users LIMIT 1").fetch_one(&pool).await?;
```

# 八、常见编译报错原因

## 1. one of the runtime features of SQLx must be enabled

缺少 `runtime-tokio` / `runtime-async-std`，sqlx 宏需要绑定异步运行时。

## 2. column "xxx" does not exist

SQL 列名写错、表不存在，编译期直接检测，运行不会炸。

## 3. mismatched types between Rust param and SQL column

传入参数类型和数据库字段不匹配，例如传 `i32` 给 `BIGSERIAL(i64)`。

## 4. sqlx-data.json is outdated

修改 SQL 后没执行 `sqlx prepare`，离线缓存失效。

## 5. trait bound DateTime<Utc>: From<OffsetDateTime>

表字段 `timestamptz` 映射 `time::OffsetDateTime`；开启 `chrono` feature 则是 `DateTime<FixedOffset>`，不能直接用 `DateTime<Utc>`。

# 九、链式调用方法大全

```rust
// 执行并获取单行，无数据则报错
.fetch_one(&pool)

// 执行获取单行，返回 Option，无数据返回 None
.fetch_optional(&pool)

// 获取全部结果 Vec<匿名结构体>
.fetch_all(&pool)

// 仅执行，不返回行，获取影响行数
.execute(&pool)

// 流式迭代（大量数据避免一次性加载内存）
.fetch(&mut pool)
```

# 十、高级用法

## 1. 原始字符串 r#""# 写多行 SQL

```rust
sqlx::query!(
    r#"
        SELECT id, username, email
        FROM users
        WHERE create_at > $1
    "#,
    now
)
```

## 2. IN 查询（数组参数）

PostgreSQL 支持 ANY($1) 传 Vec 数组：

```rust
let ids = vec![1i64,2,3];
let list = sqlx::query!(
    "SELECT id, username FROM users WHERE id = ANY($1)",
    &ids
)
.fetch_all(&pool)
.await?;
```

## 3. 事务中使用

```rust
let mut tx = pool.begin().await?;
sqlx::query!("INSERT INTO users(username) VALUES ($1)", "tx_test")
    .execute(&mut tx)
    .await?;
tx.commit().await?;
```

# 十一、优缺点总结

## 优点

1. 编译期 SQL 校验：写错表名、字段、类型直接编译失败，线上无 SQL 崩溃
2. 自动类型映射，无需手动解析 Row
3. 参数化占位符，天然防 SQL 注入
4. 支持离线编译，CI 环境无需数据库

## 缺点

1. 必须依赖数据库 / 缓存文件才能编译
2. 动态拼接 SQL 无法使用（复杂动态查询只能用 `sqlx::query()`）
3. 离线模式修改 SQL 后需要手动刷新缓存 `sqlx prepare`
