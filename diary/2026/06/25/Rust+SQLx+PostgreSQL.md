# 目录结构

```plaintext
pg-demo/
├── .env
├── Cargo.toml
├── migrations/
│   ├── 0001_init_users.up.sql
│   └── 0001_init_users.down.sql
└── src/
    └── main.rs
```

# 1. Cargo.toml

```toml
[package]
name = "pg-demo"
version = "0.1.0"
edition = "2021"

[dependencies]
# sqlx 核心，postgres驱动 + tokio异步 + 离线预编译
sqlx = { version = "0.7", features = ["postgres", "runtime-tokio", "macros", "offline"] }
tokio = { version = "1.0", features = ["full"] }
dotenv = "0.15" # 读取.env文件
anyhow = "1.0"  # 简易错误处理

[dev-dependencies]
# 开发工具，sqlx-cli全局安装即可，无需写在这里
```

# 2. .env 数据库配置

```env
# PostgreSQL 连接地址，自行修改账号、密码、库名
DATABASE_URL=postgres://postgres:123456@127.0.0.1:5432/pg_demo

# 可选：连接池最大连接数
DATABASE_POOL_MAX=5
```

# 3. 迁移文件

migrations/0001_init_users.up.sql
```sql
-- 用户表初始化
CREATE TABLE IF NOT EXISTS users (
    id BIGSERIAL PRIMARY KEY,
    username VARCHAR(64) NOT NULL UNIQUE,
    email VARCHAR(128) NOT NULL UNIQUE,
    age INT,
    create_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- 插入测试数据
INSERT INTO users (username, email, age)
VALUES ('test_user', 'test@example.com', 22);
```

migrations/0001_init_users.down.sql
```sql
DROP TABLE IF EXISTS users;
```

# 4. src/main.rs 业务示例（增删改查）

```rust
use anyhow::Result;
use sqlx::{postgres::PgPoolOptions, PgPool};

// 用户实体结构体，与数据库字段一一映射
#[derive(Debug, sqlx::FromRow)]
struct User {
    id: i64,
    username: String,
    email: String,
    age: Option<i32>,
    create_at: chrono::DateTime<chrono::Utc>,
}

// 创建数据库连接池
async fn create_pool() -> Result<PgPool> {
    // 加载.env环境变量
    dotenv::dotenv()?;
    let database_url = std::env::var("DATABASE_URL")?;
    let max_conn = std::env::var("DATABASE_POOL_MAX")
        .unwrap_or_else(|_| "5".to_string())
        .parse()?;

    let pool = PgPoolOptions::new()
        .max_connections(max_conn)
        .connect(&database_url)
        .await?;

    println!("数据库连接成功");
    Ok(pool)
}

#[tokio::main]
async fn main() -> Result<()> {
    let pool = create_pool().await?;

    // 1. 查询所有用户
    let users: Vec<User> = sqlx::query_as!(
        User,
        r#"
            SELECT id, username, email, age, create_at
            FROM users
        "#
    )
    .fetch_all(&pool)
    .await?;
    println!("全部用户: {:#?}", users);

    // 2. 新增用户
    let new_user = sqlx::query!(
        "INSERT INTO users (username, email, age) VALUES ($1, $2, $3) RETURNING id",
        "rust_user",
        "rust@demo.com",
        Some(25)
    )
    .fetch_one(&pool)
    .await?;
    println!("新增用户ID: {}", new_user.id);

    // 3. 更新用户年龄
    sqlx::query!(
        "UPDATE users SET age = $1 WHERE username = $2",
        26,
        "rust_user"
    )
    .execute(&pool)
    .await?;

    // 4. 删除用户
    sqlx::query!("DELETE FROM users WHERE username = $1", "rust_user")
        .execute(&pool)
        .await?;

    Ok(())
}
```

# 配套完整操作流程（复制直接执行）

## 1. 全局安装 sqlx-cli

```bash
cargo install sqlx-cli --no-default-features --features postgres
```

## 2. 初始化数据库 & 执行迁移

```bash
# 根据.env里的库名自动创建库
sqlx database create

# 执行所有迁移脚本
sqlx migrate run
```

## 3. 生成离线预编译缓存（编译期校验 SQL）

```bash
sqlx prepare
```

执行后生成 sqlx-data.json，提交 git，CI 环境无需数据库即可编译。

## 4. 运行项目

```bash
cargo run
```

## 5. 常用迁移命令

```bash
# 新建迁移
sqlx migrate add add_user_phone

# 回滚上一次迁移
sqlx migrate revert

# 查看迁移执行状态
sqlx migrate info

# 打开交互式PostgreSQL终端
sqlx database
```

# 补充说明

1. 离线模式：Cargo.toml 开启 offline，CI / 打包不用启动数据库；修改代码内 SQL 必须重新 sqlx prepare。
2. sqlx::query_as!：宏会在编译期校验 SQL、字段类型，写错直接编译报错，无运行时 SQL 错误。
3. 连接池：生产环境建议调整 max_connections，根据服务器配置优化。
4. 若本地 PostgreSQL 无密码，.env 连接串改为：
   ```env
   DATABASE_URL=postgres://postgres@127.0.0.1:5432/pg_demo
   ```
