TT muncher 是 Rust 声明宏（`macro_rules!`）中一种经典**递归解析模式**，全称 **Token Tree muncher**（token 树咀嚼器）。它的核心思想是：**宏每次“吃掉”输入中的一个 token，然后递归调用自身处理剩余的 token 流，直到输入为空。**

---

# 1. 为什么叫 TT muncher？

- **TT**：Token Tree，是宏匹配的最小单位，可以是标识符、字面量、标点、括号包裹的子树等。
- **muncher**：因为宏像咀嚼一样，一步一步地消耗（munch）输入的 token 序列。

它让宏能够**对输入的 token 进行逐一的、带状态的处理**，这是普通重复模式 $(...)* 难以做到的。

---

# 2. 基本模式

一个典型的 TT muncher 定义包含两个分支：

1. **递归情况**：匹配一个头 token（`$first:tt`）和剩余尾 token（`$($rest:tt)*`），处理头 token，然后递归调用宏消化尾 token。
2. **终止情况**：匹配空的 token 流（`()` 或空规则），停止递归。

```rust
macro_rules! tt_muncher {
    // 终止条件：没有 token 了
    () => {};

    // 递归：吃掉第一个 token，再处理剩下的
    ($first:tt $($rest:tt)*) => {
        // 在这里处理 $first，例如打印它
        println!("token: {}", stringify!($first));
        // 递归消化剩余 token
        tt_muncher!($($rest)*);
    };
}
```

调用 `tt_muncher!(a b + c [d] { e } )`，它会依次处理 `a`, `b`, `+`, `c`, `[d]`, `{ e }`。

---

# 3. 为什么需要 TT muncher？重复模式不够吗？

声明宏的重复模式 `$(...)*` 很强大，但它**无法在遍历时携带可变状态，也不能根据已处理的 token 动态改变后续行为**。TT muncher 则允许你在每一步执行任意 Rust 代码（通过生成表达式/语句），并能通过内部标记（如 `@state`）传递状态。

经典场景：

- 解析一小段**自定义 DSL**（如 HTML 模板、路由定义、简单命令序列）。

- 实现 `vec!` **等标准宏无法做到的累加计算**（例如在编译期计算列表长度并生成不同代码）。

- 处理**嵌套结构**，如解析 key-value 对生成 `HashMap`。

---

# 4. 实例：手工实现一个 count! 宏

计算 token 个数，这在编译期用重复模式做不到（无法简单地将计数返回为表达式）。TT muncher 可以：

```rust
macro_rules! count {
    // 终止：返回 0
    () => { 0usize };
    // 递归：吃掉一个 token，计数加 1
    ($first:tt $($rest:tt)*) => {
        1usize + count!($($rest)*)
    };
}

fn main() {
    let n = count!(a b c d e); // 生成 1 + 1 + 1 + 1 + 1
    assert_eq!(n, 5);
}
```

注意：生成的表达式是 `1 + 1 + ...`，编译器会常量折叠，零运行时开销。

---

# 5. 复杂实例：用 TT muncher 解析 map! 宏

需求：`map! { a: 1, b: 2 }` → 生成 `HashMap` 初始化的代码。

```rust
use std::collections::HashMap;

macro_rules! map {
    // 公共入口：尾随逗号处理
    ($($key:tt : $val:expr),* $(,)?) => {
        {
            let mut _map = HashMap::new();
            map!(@inner _map; $($key : $val,)*);
            _map
        }
    };

    // 内部 TT muncher：循环处理每个键值对
    // @inner 标记；_map 是生成的临时变量名，也可通过其他方式传递
    (@inner $map:ident; $key:tt : $val:expr, $($rest:tt)*) => {
        _map.insert(stringify!($key), $val);
        map!(@inner $map; $($rest)*);
    };

    // 终止：没有剩余键值对
    (@inner $map:ident;) => {};
}
```

这里用 `@inner` 标记作为内部状态的“函数名”，`$map` 作为累积器（实际上是生成的变量名，代码中可以直接引用）。每次递归处理一个键值对，生成 `_map.insert(...);`，最后剩下空时终止。

---

# 6. TT muncher vs. 重复模式 $(...)*

特性	TT muncher	$(...)* 重复模式
逐 token 处理	✅	❌（只能整体遍历一个固定模式）
携带编译期状态	✅（通过额外的累加器或内部标记）	❌
条件分支	✅（可在递归中根据 token 类型切换）	❌
复杂度	高，容易出错	低
性能（编译时间）	较慢（多次宏展开）	快（单次展开）
递归深度限制	需要注意，可能需要 #![recursion_limit = "..."]	无此问题

**原则**：能用重复模式解决的，就别用 TT muncher。

---

7. 注意事项与陷阱

- **左递归绝对禁止**：递归调用必须出现在匹配规则的右边，即必须先消耗掉某个 token。

- **卫生性（Hygiene）**：递归中直接引入的标识符（如临时变量）在不同展开层次中可能被视为相同，需用作用域块 `{}` 或 `paste` 创建唯一名称。

- **递归深度限制**：默认 128，长输入需提高。

- **编译时间膨胀**：每一步都生成新的 AST 并展开，过长输入会导致编译明显变慢。

---

# 8. 更现代的替代方案

如果 TT muncher 开始失控（比如解析复杂的 DSL），可以考虑：

- **过程宏**（proc macro）：可以编写普通的 Rust 代码操作 token 流，几乎无深度、复杂度限制。

- **`macro_rules!` + `$()*` + 巧妙利用重复展开**：有时可以通过将数据先转化为数组再迭代来避免递归。

但掌握 TT muncher 仍然非常实用，因为它是声明宏能力的极限体现，且无需引入过程宏的额外 crate。

---

# 总结

TT muncher 是 Rust 声明宏中的“手工递归解析器”，通过递归消耗 token 流来逐 token 处理，实现编译期的复杂逻辑。它灵活强大，但需警惕性能与卫生性问题，适合那些必须逐步解析、携带状态的宏场景。
