use anyhow::Result;
use sqlx::{PgPool, postgres::PgPoolOptions};

// 用户实体结构体，与数据库字段一一映射
#[derive(Debug, sqlx::FromRow)]
struct User {
    id: i64,
    username: String,
    email: String,
    age: Option<i32>,
    create_at: time::OffsetDateTime,
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
