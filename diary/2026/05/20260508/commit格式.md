# Git Commit 底层格式（终极版）

```text
tree <40位哈希>
parent <40位哈希>      ; 0、1 或多个
parent <40位哈希>
author <名字> <邮箱> 时间戳 时区
committer <名字> <邮箱> 时间戳 时区
[额外头部…]            ; gpgsig、encoding、mergetag 等

提交消息正文
可多行
可空行
末尾可带 trailers
```

# 一、强制固定结构（必须按顺序）

- tree（必须 1 行）
- parent（0、1 或多行）
- author（必须 1 行）
- committer（必须 1 行）
- extra headers（0 或多行）
- 空行（必须 1 行，分隔头部和消息）
- message（剩余所有内容）

# 二、逐行格式详解

1. tree

```text
tree 8fb9960449b932f40e77d35c11725fa27a25f439
```

- 指向目录树 hash
- 只能有 1 个

2. parent

```text
parent 11223344556677889900aabbccddeeff00112233
parent 99887766554433221100aabbccddeeff00998877
```

- 父提交
- 普通提交：1 个
- 合并提交：2 个
- 第一次提交：0 个

3. author /committer 格式

```text
author 小明 <test@example.com> 1735678901 +0800
```

结构：

```text
名字 <邮箱> 时间戳 时区
```

4. extra headers（扩展头）

除前 4 个关键字外，所有 key: value 都在这里：

```text
gpgsig -----BEGIN PGP SIGNATURE-----
 ...
 -----END PGP SIGNATURE-----
encoding utf-8
mergetag ...
```

5. 空行

头部结束标志

6. message（提交消息）

```text
feat: add login page

- support password
- support oauth

Signed-off-by: xiaoming <test@example.com>
```

可以任意内容
末尾可以带 trailers（结构化键值对）

# 三、真实完整 commit 示例

```text
tree 8fb9960449b932f40e77d35c11725fa27a25f439
parent 7766554433221100aabbccddeeff009988776655
author Alice <alice@test.com> 1735678901 +0800
committer Alice <alice@test.com> 1735678901 +0800
gpgsig -----BEGIN PGP SIGNATURE-----
 iQEzBAABCAAdfkkdjskjdkjs...
 -----END PGP SIGNATURE-----

feat: add user authentication

- add login form
- add password encryption

Co-authored-by: Bob <bob@test.com>
Signed-off-by: Alice <alice@test.com>
```
