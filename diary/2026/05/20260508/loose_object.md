# 一、是什么

Loose Object（松散对象）：

Git 仓库里单个单独存放、未打包的 Git 对象（commit/tree/blob/tag），以独立文件形式存在。
对应对立面：Pack Object（打包进 .pack/.idx 的对象）

# 二、存放路径规则

```text
.git/objects/xx/xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx
```

- 取 SHA1 哈希前 2 字节做文件夹名
- 剩余 38 字节做文件名

示例
哈希：a1b2c3...
- 目录：.git/objects/a1/
- 文件：b2c3...

# 三、文件内部格式（固定二进制结构）

Loose 对象文件内容原样固定格式：

```text
对象类型 空格 长度 \0 原始内容
```

拆解：
1. type：commit / tree / blob / tag
2. 空格
3. 字节长度（十进制字符串）
4. \0 空字节
5. 对象原始二进制内容
6. 整体 zlib 压缩 存到文件

举例 blob 松散对象

原始明文：
```text
blob 11\0hello world
```
然后做 zlib 压缩 写入 loose 文件。

# 四、特点

1. 一个对象 = 一个独立文件
2. 新创建的对象默认先存成 loose
3. 数量多了 Git 会自动 git gc 打包进 pack
4. 可直接用底层命令查看：
```bash
git cat-file -p <oid>
git cat-file -t <oid>
```

# 五、Loose vs Pack 对比

特性	Loose Object	Pack Object
存储形式	单个独立文件	多个对象打包进 .pack
位置	objects/xx/xxx	objects/pack/
压缩	单个 zlib 压缩	整体增量压缩、省空间
场景	刚提交、新对象	git gc 之后、仓库精简
访问	直接按文件读	靠 idx 索引定位偏移
