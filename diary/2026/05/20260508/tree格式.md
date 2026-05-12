# 一、Tree 对象是什么？

- 代表一个**目录**
- 里面存：**文件 + 子目录** 的列表
- 每个条目叫 **tree entry**
- 二进制格式（不是文本！）

# 二、Tree 格式 终极简化版

```text
[mode] [name]\0[20-byte-object-id]
[mode] [name]\0[20-byte-object-id]
[mode] [name]\0[20-byte-object-id]
...
```

**三个固定部分（每个 entry）**
- **mode**：八进制文件类型 + 权限（字符串形式，如 100644）
- **空格**：分隔 mode 和 name
- **name**：文件名 / 目录名
- **\0**：NULL 字节（结束 name）
- **20 字节 SHA-1**：指向 blob 或子 tree 的 ObjectId
- 没有换行符

# 三、mode 含义（必须记住）

Git 的 mode 是 **6 位八进制**，前两位表示类型：

```text
1 0  0 6 4 4
│ │  │ │ │ └── 其他用户：r-- (4)
│ │  │ │ └──── 组：r-- (4)
│ │  │ └────── 所有者：rw- (6)
│ │  └──────── 固定 0
│ └─────────── 文件 = 普通文件
└───────────── 文件类型高位
```

mode	类型	说明
100644	普通文件	blob
100755	可执行文件	blob
040000	子目录	tree
120000	符号链接	blob
160000	submodule	commit

# 四、真实 Tree 二进制示例（可看懂）

假设一个目录：

```test
file.txt (100644)
src/ (040000)
```

Tree 二进制长这样：

```text
100644 file.txt\0\x12\x34...(20字节hash)
040000 src\0\xab\xcd...(20字节hash)
```

**关键点**

- **没有换行！**所有 entry 直接连在一起
- **按 name 字典序排序**（Git 强制要求）
- **二进制紧凑存储**，非常小

# 五、Tree 格式 vs Commit 格式

对象	格式	存储
commit	纯文本（头部 + 消息）	可读
tree	紧凑二进制	不可直接读
