psql 是 PostgreSQL 自带交互式终端，用于登录数据库、执行 SQL、管理库 / 表、导出导入数据、运维，区分元命令（反斜杠 \ 开头） 和标准 SQL。

# 一、登录与连接

## 1. 基础登录语法

```bash
psql [参数] [库名] [用户名]
```

常用登录方式

```bash
# 1. 本地默认用户 postgres，登录 postgres 库
psql -U postgres

# 2. 指定地址、端口、库、账号密码
psql -h 127.0.0.1 -p 5432 -U postgres -d pg_demo

# 3. 带密码参数（不安全，仅测试）
psql -h 127.0.0.1 -p 5432 -U postgres -d pg_demo -W
# -W 强制弹出密码输入框

# 4. 直接通过连接串登录（和sqlx DATABASE_URL格式兼容）
psql postgres://postgres:123456@127.0.0.1:5432/pg_demo

# 5. 执行单条SQL后退出
psql -U postgres -d pg_demo -c "SELECT * FROM users;"

# 6. 执行外部sql文件（导入脚本）
psql -U postgres -d pg_demo -f init.sql
```

登录参数速查

参数	作用
-h	数据库主机地址
-p	端口号，默认 5432
-U	登录用户名
-d	指定数据库名
-W	手动输入密码
-c	执行单行 SQL 并退出
-f	执行外部 SQL 文件
-q	静默模式，不打印多余输出
-X	不加载～/.psqlrc 配置文件

# 二、内置元命令（\ 开头，psql 专属，不属 SQL）

## 1. 数据库切换、基础信息

```psql
# 切换数据库
\c pg_demo
\connect pg_demo postgres  # 切换库+指定用户

# 查看当前连接信息（用户、库、端口）
\conninfo

# 退出psql
\q
quit
```

## 2. 库、表、视图、索引查询（最常用）

```psql
# 列出所有数据库
\l
\list

# 列出当前库所有表、视图、序列
\dt

# 只看用户表，排除系统表
\dt public.*

# 查看单张表结构（字段、类型、约束、注释）
\d users

# 详细表信息（含索引、外键、触发器）
\d+ users

# 列出所有视图
\dv

# 列出所有索引
\di

# 列出序列自增主键
\ds

# 列出函数/存储过程
\df

# 列出所有schema
\dn

# 查看schema下所有对象
\dt public.*
```

## 3. 用户、权限、角色管理

```psql
# 查询所有用户/角色
\du

# 创建用户
CREATE USER test WITH PASSWORD '123456';
# 赋予建库权限
ALTER ROLE test CREATEDB;
```

## 4. 导入导出数据（csv/text）

```psql
# 导出表数据到本地csv
\copy users TO '/tmp/users.csv' WITH (FORMAT csv, HEADER);

# 本地csv导入表
\copy users FROM '/tmp/users.csv' WITH (FORMAT csv, HEADER);
```

区别：COPY 是 SQL 服务端文件操作；\copy 是 psql 客户端本地文件，普通用户可用。

## 5. 文件操作、执行脚本

```psql
# 加载执行外部sql文件
\i ./mig.sql

# 编辑SQL（调用系统编辑器 vim/nano）
\e
# 编辑单条SQL
\e SELECT * FROM users;

# 将查询结果写入本地文件
\o output.txt
SELECT * FROM users;
\o  # 关闭输出重定向
```

## 6. 格式化输出

```psql
# 切换格式化模式
\x          # 横向/纵向展示（宽表必备）
\x auto     # 自动判断是否纵向展示

# 输出格式：csv/html/aligned
\pset format csv
\pset format html
\pset aligned  # 默认表格

# 关闭分页输出
\pset pager off
```

## 7. 事务、数据库维护

```psql
# 查看当前执行中的SQL、进程
\pg_stat_activity

# 重新加载配置（无需重启数据库）
SELECT pg_reload_conf();
```

## 8. 注释、帮助

```psql
# 查看SQL命令帮助
\h CREATE TABLE
\h SELECT

# 查看元命令帮助
\?
```

# 三、日常运维常用 SQL（配合 psql 使用）

## 1. 库管理

```psql
-- 创建数据库
CREATE DATABASE pg_demo;

-- 删除数据库
DROP DATABASE IF EXISTS pg_demo;

-- 查看数据库大小
SELECT pg_size_pretty(pg_database_size('pg_demo'));
```

## 2. 表维护

```psql
-- 查看表占用空间
SELECT pg_size_pretty(pg_table_size('users'));

-- 清空表（不记录日志，速度快）
TRUNCATE TABLE users;

-- 重建索引、回收空间（碎片整理）
VACUUM ANALYZE users;
```

## 3. 连接与进程管理

```psql
-- 查看所有连接
SELECT pid, usename, datname, query, state FROM pg_stat_activity;

-- 杀死卡死连接
SELECT pg_terminate_backend(pid);
```

# 四、psql 配置文件～/.psqlrc

每次打开 psql 自动执行，可预设格式、别名：

```ini
~/.psqlrc
\x auto
\pset pager off
\pset format aligned
```

# 五、高频实战组合命令

## 1. 一键初始化库（shell 直接执行）

```psql
# 创建库 + 执行迁移脚本
psql -U postgres -c "CREATE DATABASE pg_demo;"
psql -U postgres -d pg_demo -f migrations/0001_init_users.up.sql
```

## 2. 导出整库备份

```psql
# 备份整个库（pg_dump，配套psql工具）
pg_dump -U postgres -d pg_demo > backup.sql
# 恢复备份
psql -U postgres -d pg_demo -f backup.sql
```

## 3. 快速查询表结构

```psql
psql -U postgres -d pg_demo -c "\d users"
```

# 六、psql vs sqlx-cli 对比

工具	定位	优势
psql	PostgreSQL 原生客户端	完整运维、权限、导出、调优、查看执行计划
sqlx-cli	Rust SQLx 配套工具	专门管理迁移、离线预编译，贴合 Rust 项目开发

# 七、常见报错解决

1. 连接报错：Peer authentication failed
本地登录不加 -h 127.0.0.1 默认走 socket，修改 pg_hba.conf 认证方式或使用密码 TCP 登录。

2. \copy 找不到文件
路径是本机客户端路径，不是数据库服务器路径；权限不足切换文件读写权限。

3. 中文乱码
登录前设置编码：export PGCLIENTENCODING=utf8

4. 端口 5432 拒绝连接
PostgreSQL 未启动、防火墙拦截、postgresql.conf listen_address 未开放。

# 八、元命令速查表

```psql
# 连接
\c dbname       切换库
\conninfo       查看连接信息
\q              退出

# 对象查询
\l              所有数据库
\dt             列表
\d table        表结构
\d+ table       详细表信息
\dv \di \df     视图/索引/函数
\du             用户角色

# 文件IO
\i file.sql     执行脚本
\copy           本地导入导出
\o file         输出到文件
\e              打开编辑器写SQL

# 格式化
\x              纵向展示
\pset format    修改输出格式

# 帮助
\h sql命令      SQL语法帮助
\?              psql元命令帮助
```
