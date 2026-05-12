# 一、完整 Commit 结构

```text
tree <tree_hash>
parent <parent_hash>
parent <parent_hash>  ; 合并提交会有 0、1 或多个 parent
author <name> <email> <timestamp> <timezone>
committer <name> <email> <timestamp> <timezone>
<extra_headers...>    ; gpgsig、mergetag、Change-Id、Signed-off-by 等

<commit_message>
```

关键规则

1. 头部行：key value 格式，每行一个
2. 空行：头部和消息之间必须有一个空行分隔
3. 消息：可以是任意文本，没有格式限制

# 二、逐字段详解

1. tree（必须 1 个）
   ```text
   tree 8b75d330159566a9d15b2b39d2c9a4c27cc5f2b
   ```
   - 目录树哈希
   - 有且只有 1 个

2. parent（0、1 或多个）
   
   ```text
   parent a1b2c3d...
   parent x9y8z7...
   ```

   - 父提交
   - 普通提交：1 个
   - 合并提交：2 个
   - 根提交：0 个

3. author（必须 1 个）

   ```text
   author Bob <bob@x.com> 1710000000 +0800
   ```

   - 作者信息
   - 格式：名字 <邮箱> 时间戳 时区

4. committer（必须 1 个）

   - 提交者信息（格式同上）

5. extra_headers（扩展头部，0 或多个）

   常见 key：
   - gpgsig
   - mergetag
   - encoding
   - Change-Id
   - Signed-off-by
   - Co-authored-by

6. 空行
   
   头部结束的标志

7. commit message（提交消息）

# 三、真实完整示例（可直接解析）

```text
tree 8a7b6c5d4e3f2a1b908776554433221100998877
parent 11223344556677889900aabbccddeeff00112233
author Alice <alice@test.com> 1710123456 +0800
committer Alice <alice@test.com> 1710123456 +0800
gpgsig -----BEGIN PGP SIGNATURE-----
 iQEzBAABCAAdfkkdjskjdkjs...
 -----END PGP SIGNATURE-----
Change-Id I837fecd8923dc98d32d8923d98e23d

feat: add user login

- support password login
- support oauth login
```

# 四、message trailers

```text
tree <hash>
parent <hash>
author ...
committer ...
[extra headers]

[message 正文]

[空行]
trailer-key-1: value1
trailer-key-2: value2
...
```

- extra headers：在 “空行”之前，属于 commit 对象头的一部分
- message + trailers：在 “空行”之后，属于 commit 消息体
- trailers 必须在消息体末尾，前面要有至少一个空行

示例（含正文 + trailers）

```text
feat: add payment module

- integrate Alipay
- add unit tests

Signed-off-by: Alice <alice@example.com>
Co-authored-by: Bob <bob@example.com>
Fixes: #123
Change-Id: Iabc123...
```

每行格式：

```text
Key-Name: value with spaces
```

- key：无空格、大小写不敏感、推荐驼峰（如 Signed-off-by）
- 分隔符：冒号 + 空格（: ）是标准；也可用 =，但最终会归一化为 : Git
- value：可含空格、特殊字符、邮箱、URL 等
- 分组：trailers 必须连续成组，前面有空行，且位于消息最末尾

常见标准 Trailer Key（内核 / 开源项目通用）

Key	用途	示例
Signed-off-by	提交者签名（DCO 合规）	Signed-off-by: Alice <a@x.com>
Co-authored-by	共同作者	Co-authored-by: Bob <b@x.com>
Acked-by	审核通过	Acked-by: Charlie <c@x.com>
Reviewed-by	代码评审通过	Reviewed-by: Dave <d@x.com>
Tested-by	测试通过	Tested-by: Eve <e@x.com>
Fixes	修复的 issue	Fixes: #123 / Fixes: PROJ-456
Closes	自动关闭 issue	Closes: #789
Refs	关联引用	Refs: commit abc123, #456
Change-Id	Gerrit 变更 ID	Change-Id: I837fecd...

extra headers vs message trailers（极易混淆，重点区分）

1. extra headers（CommitRef::extra_headers）
位置：commit 对象头，空行之前
属于：commit 元数据（和 tree/parent/author 同级）
常见 key：gpgsig、mergetag、encoding

2. message trailers
位置：commit 消息体，空行之后、正文末尾
属于：commit message 的一部分
常见 key：Signed-off-by、Co-authored-by、Fixes
