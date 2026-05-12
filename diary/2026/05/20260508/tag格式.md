# 一、先分清两种 Tag

1. 轻量标签（lightweight tag）
   
   只是一个分支引用：
   .git/refs/tags/v1.0 里面直接存 commit OID
   不是独立 Git 对象，没有 tag 格式。

2. 注解标签（annotated tag）

   是独立 Git 对象，有完整固定格式，存放在 objects 里，有自己的 SHA1，这就是你要的 tag 格式。

# 二、Annotated Tag 标准底层格式（固定顺序）

```text
object <oid>
type <obj-type>
tag <tag-name>
tagger <name> <email> <timestamp> <tz>

tag message
可选多行备注
```

字段逐条解释

1. object

   必须有，指向被打的对象 OID
   可以是：commit / tree / blob / 另一个 tag

2. type

   对象类型：
   commit / tree / blob / tag

3. tag

   标签名，如 v1.0.0

4. tagger

   格式和 author/committer 完全一样：
   ```text
   名字 <邮箱> 时间戳 时区
   ```

5. 空行

   分隔头部 和 tag 消息体

6. tag message

   备注说明，可多行，可空

# 三、真实完整示例

```text
object 67a1f234567890abcdef1234567890abcdef12
type commit
tag v2.1.0
tagger Li <li@example.com> 1740000000 +0800

release version 2.1.0
fix bug and optimize performance
```

```text
object 8b75d330159566a9d15b2b39d2c9a4c27cc5f2b
type commit
tag v1.0.0
tagger Someone <a@b.com> 1712345678 +0800

Release v1.0.0
Fix bugs and improve performance

-----BEGIN PGP SIGNATURE-----
wsBcBAABCAAQBQJn1234CRxxxxxx...
xxxxxx...
-----END PGP SIGNATURE-----
```

# 四、Loose 对象内部结构（存到磁盘）

和 commit/tree/blob 一模一样：

1. 头部：tag 123\0
   - tag 对象类型
   - 空格 + 内容字节长度
   - \0 空字节
2. 后面紧跟上面标准 tag 文本
3. 整体 zlib 压缩 存入 loose 文件

# 五、和 Commit / Tree 格式对比

对象	结构特点
commit	tree + parent + author + committer + 扩展头 + 消息
tree	二进制 mode+name\0+oid 连续条目
tag	object+type+tag+tagger + 空行 + 备注（纯文本）
