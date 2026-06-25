# 一、什么是 `sqlx-cli`

`sqlx-cli` 是 SQLx 官方配套命令行工具，独立于你的 Rust 项目，核心三大能力：

1. 数据库版本迁移管理（migrate），自动记录已执行脚本，支持回滚
2. 离线模式缓存生成 `sqlx-data.json`，让 Rust 编译时不用连数据库校验 SQL
3. 辅助工具：创建 / 删除库、交互式数据库终端、执行 SQL 文件、查看表结构

支持驱动：`postgres` / `mysql` / `sqlite` / `mssql`

# 二、安装 & 卸载

## 1. 安装（按需选择驱动）

```bash
# 只装 postgres（推荐本模板使用）
cargo install sqlx-cli --no-default-features --features postgres

# 全驱动（pg/mysql/sqlite/mssql）
cargo install sqlx-cli --no-default-features --features postgres,mysql,sqlite,mssql

# 仅 sqlite
cargo install sqlx-cli --no-default-features --features sqlite
```

## 2. 验证 & 卸载

```bash
# 查看版本
sqlx --version
# 查看全局帮助
sqlx -h

# 卸载
cargo uninstall sqlx-cli
```

# 三、核心前置：DATABASE_URL 连接串

所有命令都会自动读取项目根目录 .env 文件，无需手动导出环境变量。

PostgreSQL 格式

```env
# 有密码
DATABASE_URL=postgres://用户名:密码@127.0.0.1:5432/库名
# 无密码
DATABASE_URL=postgres://postgres@127.0.0.1:5432/pg_demo
# 带ssl（远程云数据库）
DATABASE_URL=postgres://user:pass@xxx.rds.aliyuncs.com:5432/db?sslmode=require
```

其他数据库格式参考

```env
# MySQL
DATABASE_URL=mysql://root:123456@127.0.0.1:3306/mydb
# SQLite（文件）
DATABASE_URL=sqlite://./data.db
```

手动临时指定 URL（不读 .env）

```bash
DATABASE_URL="postgres://postgres:123@127.0.0.1:5432/test" sqlx migrate run
```

# 四、命令分组详解

## 1. migrate 迁移模块（最常用）

迁移文件存放目录固定为项目根 migrations/，文件命名规则：
0001_xxx.up.sql（升级脚本）、0001_xxx.down.sql（回滚脚本）
数字版本号全局递增，不可手动修改。

### 1.1 创建迁移文件

```bash
sqlx migrate add init_users
```

自动生成：

```plaintext
migrations/
  0001_init_users.up.sql
  0001_init_users.down.sql
```

### 1.2 执行所有未运行迁移（升级）

```bash
sqlx migrate run
```

底层逻辑：

1. 数据库自动创建 _sqlx_migrations 系统表，记录版本、哈希、执行时间
2. 只执行从未运行过的脚本，重复执行安全
3. 脚本按数字从小到大顺序执行

### 1.3 回滚最近一次迁移

```bash
sqlx migrate revert
```

执行对应版本的 .down.sql，并删除迁移记录。

注意：只能单步回滚，不支持一键回滚多个版本。

### 1.4 查看迁移状态

```bash
sqlx migrate info
```

输出每一条迁移：版本号、文件名、是否已执行、执行时间。

### 1.5 指定迁移目录（非默认 migrations）

```bash
sqlx migrate run --migrations-dir ./db/mig
```

## 2. database 数据库操作

### 2.1 创建数据库（库不存在则新建）

```bash
sqlx database create
```

根据 .env 中 DATABASE_URL 里的库名自动创建，无需手动进 psql。

### 2.2 删除数据库（高危操作）

```bash
sqlx database drop
```

直接清空整个库，生产环境慎用。

### 2.3 交互式数据库终端（替代 psql）

```bash
sqlx database
```

进入 SQL 交互环境，可直接执行 `SELECT * FROM users;`

### 2.4 导出完整库 schema

```bash
sqlx database schema
```

打印所有建表、索引、约束 SQL。

## 3. prepare：离线模式核心命令（配合 Rust sqlx offline）

### 作用

Rust 中 `sqlx::query!` 编译期需要连接数据库校验 SQL；
`sqlx prepare` 扫描项目所有 SQL，生成 `sqlx-data.json` 缓存文件，开启 `offline` 后编译不再依赖数据库。

### 使用流程

1. 本地数据库必须执行完所有迁移（表结构完整）

2. 执行生成缓存：
```bash
sqlx prepare
```

3. 项目 Cargo.toml 开启离线特性
```toml
sqlx = { version = "0.7", features = ["postgres", "runtime-tokio", "macros", "offline"] }
```

4. 提交 `sqlx-data.json` 到 Git，CI / 打包环境无需启动数据库即可编译。

### 更新缓存

只要代码里修改了 SQL、新增 SQL，必须重新执行：

```bash
sqlx prepare
```

不更新会报 `sqlx-data.json is outdated` 编译错误。

### 指定源码目录（多目录项目）

```bash
sqlx prepare --src ./crates/server/src
```

## 4. run：直接执行 SQL 文件

```bash
# 执行单文件
sqlx run --file seed.sql

# 多行SQL直接传参
sqlx run --sql "SELECT * FROM users LIMIT 10;"
```

# 五、全局通用参数

```bash
# 自定义 .env 文件路径
sqlx migrate run --env .env.prod

# 不读取 .env，纯靠环境变量
sqlx migrate run --no-env-file

# 输出详细日志
sqlx migrate run --verbose
```

# 六、完整标准工作流（PostgreSQL 项目）

1. 安装工具
```bash
cargo install sqlx-cli --no-default-features --features postgres
```

2. 项目新建 .env 配置连接串

3. 创建第一条迁移
```bash
sqlx migrate add init_users
```

4. 编写 up/down SQL

5. 创建数据库并执行迁移
```bash
sqlx database create
sqlx migrate run
```

6. 开发写 Rust SQL 代码，生成离线缓存
```bash
sqlx prepare
```

7. 日常迭代新增表 / 字段
```bash
sqlx migrate add add_user_phone
sqlx migrate run
sqlx prepare
```

8. 需要撤销上次变更
```bash
sqlx migrate revert
```

# 七、常见问题与坑

1. sqlx prepare 报错找不到表
本地数据库没执行完 sqlx migrate run，结构不完整。

2. 修改表结构后编译报错 outdated
SQL 或表结构变更后，忘记重新 sqlx prepare。

3. 迁移文件手动修改版本号
会导致 _sqlx_migrations 版本错乱，只能删除库重新执行迁移。

4. CI 环境编译失败
未提交 sqlx-data.json，或 Cargo.toml 没开启 offline。

5. .env 不生效
命令执行目录不是 .env 所在根目录。

# 八、全命令速查表

```bash
# 迁移管理
sqlx migrate add NAME        创建迁移文件
sqlx migrate run             执行全部未运行迁移
sqlx migrate revert          回滚最近一次迁移
sqlx migrate info            查看迁移执行状态

# 数据库操作
sqlx database create         创建数据库
sqlx database drop           删除数据库
sqlx database drop -y
sqlx database                交互式SQL终端
sqlx database schema         导出库结构

# 离线缓存
sqlx prepare                 生成sqlx-data.json离线缓存

# 执行SQL
sqlx run --file xxx.sql      执行SQL文件
sqlx run --sql "SELECT ..."  执行单行SQL

# 全局辅助
sqlx --version               查看版本
sqlx -h                      帮助文档
```
