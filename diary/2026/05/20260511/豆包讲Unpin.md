什么样的stream实现Unpin？

---

我用**最简单、最直白、不绕弯**的方式告诉你：
# 一句话结论
**绝大多数你日常用的 Stream 都自动实现了 Unpin，不需要你管。**

---

# 一、哪些 Stream 是 Unpin？

满足下面任意一种，就是 `Unpin`：

1. **普通集合、普通结构体 的 Stream**
```rust
stream::iter(vec![1,2,3]) // Unpin
stream::once(async { 1 }) // Unpin
stream::empty()           // Unpin
```
它们**没有自引用**，内存固定，随便移动。

2. **Box 包裹的 Stream**
```rust
Box::pin(stream) // 是 !Unpin → Pin<Box<_>>
Box::new(stream) // 是 Unpin
```
Pin<Box<dyn Stream>> = !Unpin
Box<dyn Stream> = Unpin

3. **大部分 futures-util 组合子**
```rust
stream.map(|x| x)    // Unpin
stream.filter(...)   // Unpin
stream.fuse()        // Unpin
stream.then(...)     // Unpin
```

只要**内部流是 Unpin**，组合子也自动 Unpin。

---

# 二、只有这些是！Unpin（不能随便移动）

只有 内部有自引用 的流才是 !Unpin：

- stream::select(…) 某些情况
- 自己写的、存在 &mut 指向自己 的异步流
- 用 pin_mut! 绑定到栈上的流（变成 Pin<&mut St>）

99% 的业务代码不会遇到！Unpin 的 Stream。

---

# 三、最简单判断规则（背这个）

只要不是你手动写的自引用异步流，都是 Unpin。
只要你用 stream::iter /once/empty / 组合子，都是 Unpin。

---

# 四、代码里怎么保证安全？

如果你不确定，直接用：
```rust
use futures_util::StreamExt;

stream.next()           // 自动要求 St: Unpin
```

或者如果你不确定是不是 Unpin：
```rust
pin_mut!(stream);       // 强制钉在栈上，变成 Unpin 可用
stream.next().await;
```

---

# 五、超级精简总结
✅ 日常用的流 99% 都是 Unpin
✅ iter、once、empty、map、filter、fuse → 全是 Unpin
✅ 只有自引用流才是！Unpin
✅ 不确定就 pin_mut!(stream)，永远安全
