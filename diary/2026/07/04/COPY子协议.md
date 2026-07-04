# COPY 子协议

用于高效导入导出数据。由 SQL `COPY` 触发。

**CopyInResponse ('G')** —— 服务器要求客户端发送数据：

```text
'G'
Int32   长度
Int8    整体格式 (0 文本, 1 二进制)
Int16   列数
Int16[N] 每列格式 (0 文本, 1 二进制)
```

**CopyOutResponse ('H')** —— 服务器将要向客户端发送数据，格式同上。

客户端发送：**CopyData ('d')** 发送数据块，**CopyDone ('c')** 表示结束，**CopyFail ('f')** 表示失败。

服务器可回复 **CopyDone** 或 **CommandComplete**。



