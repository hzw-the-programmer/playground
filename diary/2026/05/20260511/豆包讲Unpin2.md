1. Unpin 官方本质（一句话）

Unpin 是一个自动派生的标记 trait：
只要结构体内部没有自引用，编译器自动给你实现 Unpin；
只要有自引用，编译器自动不给实现 Unpin（就是 !Unpin）。

2. 判定铁律（不用记原理，直接套）

满足下面全部 → 自动 impl Unpin
所有字段都是 Unpin
结构体不存在字段指针指向自身内存

只要有自引用 → !Unpin
比如：结构体里一个字段是指针 / 引用，指向自己另一个字段，这种不能随便移动，一移动指针就失效，所以是 !Unpin。

3. 哪些 Stream 天生 Unpin

- stream::iter / once / empty
- map/filter/take/skip/fuse 这些组合子
- 普通 Vec、数组迭代转的流
- 没有自引用的自定义结构体 Stream

原因：都是普通数据组合，没有自己引用自己，编译器自动实现 Unpin。

4. 哪些 Stream 是！Unpin

只有一类：带自引用的异步生成器 / 流
典型：
用 async-stream 宏生成的流
手写的、内部字段互相引用的 Stream
裸的 GenStream 这类自引用结构

5. 为什么 next() 要 Unpin

StreamExt::next() 签名：
```rust
fn next(&mut self) -> Next<'_, Self>
where
    Self: Stream + Unpin
```

它要按普通 &mut 借用反复 poll，允许随意移动内存，所以要求 Unpin。

6. 遇到！Unpin 怎么搞定

两种标准写法：

```rust
// 1. 栈上钉住
futures_util::pin_mut!(stream);
// 然后就能 next().await
stream.next().await;
```

```rust
// 2. 堆上 Pin<Box>
let stream = Box::pin(stream);
stream.next().await;
```

7. 极简口诀

无自引用 → 自动 Unpin
有自引用 → 一定！Unpin
不确定就 pin_mut! / Box::pin。
