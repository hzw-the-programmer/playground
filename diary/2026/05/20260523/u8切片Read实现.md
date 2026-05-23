我们通过一个“阅读卷轴”的比喻，来看看 `&[u8]` 是如何实现 `Read` 的，以及这种实现有多巧妙。

---

# 卷轴的秘密

想象你手里有一卷纸（`&[u8]`），上面写着几个字节：`[1, 2, 3, 4, 5]`。
你每次想读一点，但不允许修改纸上的内容，只能调整你视线对准纸的位置。
这就是 `&[u8]` 实现 `Read` 的本质：不移动数据，只移动“视口”。

标准库的源码极度精简，核心就这三步：

```rust
fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
    let amt = cmp::min(buf.len(), self.len());  // 最多读完当前视口
    let (a, b) = self.split_at(amt);             // 把视口切成 [已读 | 剩余]
    buf[..amt].copy_from_slice(a);               // 复制出已读部分
    *self = b;                                   // 视口前移，指向剩余部分
    Ok(amt)
}
```

注意：这里 `self` 的类型是 `&mut &[u8]`，所以 `*self = b` 修改的是引用本身，而非底层数据。

---

# 巧妙的例子：像卷轴一样滚动

```rust
use std::io::Read;

fn main() {
    let data: &[u8] = &[10, 20, 30, 40, 50][..];
    let mut scroll = data;         // 卷轴引用，刚开始指向全部数据

    println!("初始卷轴: {:?} (长度 {})", scroll, scroll.len());

    // 第一次读 3 个字节
    let mut buf = [0u8; 3];
    scroll.read(&mut buf).unwrap();
    println!("读出: {:?}", buf);
    println!("剩余卷轴: {:?} (长度 {})\n", scroll, scroll.len());

    // 第二次读 3 个字节（但只剩 2 个了）
    let mut buf2 = [0u8; 3];
    let n = scroll.read(&mut buf2).unwrap();
    println!("读到了 {} 个字节: {:?}", n, &buf2[..n]);
    println!("剩余卷轴: {:?} (长度 {})", scroll, scroll.len());
}
```

输出：

```text
初始卷轴: [10, 20, 30, 40, 50] (长度 5)
读出: [10, 20, 30]
剩余卷轴: [40, 50] (长度 2)

读到了 2 个字节: [40, 50]
剩余卷轴: [] (长度 0)
```

`scroll` 本身是 `&[u8]` 类型的变量，在 `read` 调用后它指向的位置发生了变化。底层数组纹丝不动，你只是把卷轴向前卷动了。

---

# 陷阱：复制引用会“分身”

因为这个机制依赖 `&mut self` 修改引用，如果你不小心复制了一个引用，对副本的读取不会影响原引用：

```rust
let data = &[1, 2, 3, 4][..];
let mut a = data;      // a 指向全部
let mut b = a;         // b 是 a 的拷贝，同样指向全部

let mut buf = [0u8; 2];
a.read(&mut buf).unwrap();   // a 视口前移，现在 a 是 [3,4]
println!("a = {:?}", a);     // [3, 4]
println!("b = {:?}", b);     // b 还是 [1, 2, 3, 4] ！
```

这恰恰说明了 `Read for &[u8]` 通过修改引用本身来记录位置，而不是维护额外的状态对象。

---

# 与 `Cursor<&[u8]>` 的对比

`Cursor` 是另一个字节流适配器，它通过内部 `pos` 字段记录读取位置，而不修改你给它的切片：

```rust
use std::io::Cursor;
let data = &[1, 2, 3];
let mut cur = Cursor::new(data);
cur.read(&mut [0u8; 2]).unwrap();
// data 仍然完整，cur.position() 返回 2
```

而 `&[u8]` 的 `Read` 实现没有额外的 `pos` 字段，它直接把位置信息编码在引用值本身中。
这正是它巧妙的地方：利用 Rust 的所有权和引用机制，把一个普通的宽指针（切片引用）变成了零开销的只读流，不需要任何堆分配，连 `Cursor` 里的那个 `usize` 都省了。
