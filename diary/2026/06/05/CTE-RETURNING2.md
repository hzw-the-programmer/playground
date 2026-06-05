# 一、整体结构拆分

```sql
with inserted_post as (
    -- 子句1：执行插入，并把插入结果存入临时CTE表 inserted_post
    insert into post(user_id, content)
    values ($1, $2)
    returning post_id, user_id, content, created_at
)
-- 子句2：拿刚插入的数据联查user表，拼接username返回最终结果
select post_id, username, content, created_at
from inserted_post
inner join "user" using (user_id)
```

> `$1`、`$2`：预处理参数（JDBC/Node-Pg/ORM 占位符），`$1=user_id`，`$2=content`，防 SQL 注入。

# 二、逐段拆解

## 1. `WITH inserted_post AS (...)`

`WITH` 在 PG 里叫 CTE (公用表表达式)，这里特殊用法：CTE 内部包裹 INSERT 语句

- CTE 别名：`inserted_post`，是仅本条 SQL 内临时虚拟表，不会落地数据库；
- 只有 PG、MySQL8.0+、SQLServer 支持 CTE 内嵌 DML (INSERT/UPDATE/DELETE)，MySQL 低版本不支持。

## 2. CTE 内部：`INSERT` + `RETURNING`

```sql
insert into post(user_id, content)
values ($1, $2)
returning post_id, user_id, content, created_at
```

## 1. `insert into post(...) values($1,$2)`：向帖子表 `post` 插入一行记录：用户 ID、帖子内容；

## 2. `RETURNING` 是 PG 核心特性：
   + 执行 INSERT 成功后，把当前刚新增的数据字段当做结果集返回；
   + 返回字段：`post_id`(帖子主键自增ID)、`user_id`、`content`、`created_at`(创建时间)；
   + 这一行返回的数据，全部存入临时虚拟表 `inserted_post`。

> 等价逻辑：插入后立刻 `select 新增行 from post where 主键=刚生成id`，但不用二次查库。

## 3. 外层 `SELECT` + `INNER JOIN`

```sql
select post_id, username, content, created_at
from inserted_post
inner join "user" using (user_id)
```

1. `inserted_post`：拿到上一步刚插入的帖子数据；
2. `inner join "user" using(user_id)`：
   + `using(user_id)` = `on inserted_post.user_id = "user".user_id`；
   + `"user"` 加双引号：`user` 是 SQL 保留关键字，PG 中双引号标识表名，避免语法报错；
3. 关联用户表，从 `user` 表取出 `username` (用户名)，和帖子字段拼接；
4. 最终返回：帖子ID、用户名、帖子内容、创建时间。

# 三、业务逻辑一句话总结

根据传入的用户 ID ($1) 和帖子内容 ($2) 新增一条帖子，插入完成立刻关联用户表查出发帖人姓名，一条 SQL 直接返回完整帖子详情。

# 四、执行流程时序

1. 数据库接收参数 `$1=用户ID`，`$2=帖子正文`；
2. 执行 `post` 表 `INSERT` 写入磁盘；
3. `RETURNING` 捕获新增行→存入临时 CTE `inserted_post`；
4. CTE 数据 `JOIN user` 表匹配用户；
5. 拼接字段，输出最终结果。

# 五、优点 & 注意事项

✅ 优点

1. 原子性：整段 SQL 是单事务，插入和查询在同一个事务，要么全成要么全失败；
2. 性能高：仅 1 次 SQL 往返，无需「insert→再 select」两次 IO；
3. 安全：参数占位符 $1/$2 预编译，杜绝拼接 SQL 注入。

⚠️ 注意

1. `INNER JOIN` 坑：如果 `user` 表中不存在该 `user_id`，插入成功但最终返回空数据
2. 需求要强制用户必须存在：数据库给 `post.user_id` 加外键约束，不存在 `user_id` 时 `INSERT` 直接报错；
3. 需要帖子一定能查出（用户可能被删仍保留帖子）：改用 `LEFT JOIN "user"`，`username` 为 `null`；
4. 仅 PostgreSQL 原生完美支持，MySQL 不能在 `WITH` 中写 `INSERT`。

# 六、等价拆分写法（不用 CTE，两次 SQL，不推荐）

```sql
-- 第一步插入，拿到自增ID
insert into post(user_id,content) values($1,$2) returning post_id;
-- 第二步根据ID联查用户
select p.post_id,u.username,p.content,p.created_at
from post p join "user" u using(user_id) where post_id=上面拿到的id;
```

需要两次数据库请求，并发场景容易出现中间数据变更问题。

# 七、拓展变种

1. 左连接（用户删除后帖子仍展示）

```sql
with inserted_post as (
    insert into post(user_id, content) values ($1, $2)
    returning post_id, user_id, content, created_at
)
select post_id, username, content, created_at
from inserted_post
left join "user" using (user_id);
```

2. 批量插入多条 post 再联查

```sql
with inserted_post as (
    insert into post(user_id, content)
    values ($1,$2),($3,$4) -- 多行插入
    returning post_id, user_id, content, created_at
)
select ... from inserted_post join "user" using(user_id);
```
