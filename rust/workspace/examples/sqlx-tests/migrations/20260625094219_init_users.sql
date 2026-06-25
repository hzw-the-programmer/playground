-- Add migration script here
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