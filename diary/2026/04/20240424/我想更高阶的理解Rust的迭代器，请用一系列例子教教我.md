要真正理解 Rust 迭代器的高阶用法，关键在于掌握惰性求值、组合子（adapters）以及所有权的交互。下面通过一系列精心设计的例子，从概念到实际应用，帮你建立起对迭代器的深层认知。

---

# 1. 三种基础迭代器：iter()、iter_mut()、into_iter()

```rust
fn main() {
    let mut v = vec![10, 20, 30];

    // iter() -> 不可变引用
    for &x in v.iter() {
        println!("{}", x);
    }

    // iter_mut() -> 可变引用
    for x in v.iter_mut() {
        *x *= 2;
    }
    println!("After mut: {:?}", v);

    // into_iter() -> 转移所有权，消耗原集合
    let sum: i32 = v.into_iter().sum();
    println!("Sum = {}", sum);
    // v 已经不能再使用
}
```

**高阶洞察**：选择正确的迭代器类型决定了后续操作能否修改元素或复用原容器。

---

# 2. 组合子链与零成本抽象

```rust
fn main() {
    let numbers = 1..=10;

    // 一条链：偶数 -> 平方 -> 求和（惰性执行）
    let sum_of_even_squares: i32 = numbers
        .filter(|&x| x % 2 == 0)   // 2,4,6,8,10
        .map(|x| x * x)            // 4,16,36,64,100
        .sum();                    // 最终求值

    println!("{}", sum_of_even_squares); // 220
}
```

**高阶洞察**：每个适配器（filter, map）都返回一个新的迭代器，直到 sum() 这种消费者才真正执行。Rust 会通过内联和展开将这条链编译为与手写循环同等高效的代码。

---

# 3. flat_map 与 flatten：摊平嵌套结构

```rust
fn main() {
    let words = vec!["hello world", "rust iterator", "flat map"];

    // 分割字符串，得到一个迭代器的迭代器
    let chars: Vec<char> = words
        .iter()
        .flat_map(|&s| s.chars())   // 每个字符作为独立元素
        .collect();

    println!("{:?}", chars);

    // 另一种写法：先用 map 得到 Vec<Vec<_>> 再 flatten
    let nested = vec![vec![1,2], vec![3,4], vec![5]];
    let flat: Vec<_> = nested.into_iter().flatten().collect();
    println!("{:?}", flat); // [1,2,3,4,5]
}
```

**高阶洞察**：flat_map 相当于 map + flatten，常用于处理 Option、Result 或需要展开多层容器的场景。

---

# 4. filter_map：过滤并同时转换

```rust
fn main() {
    let strings = vec!["42", "foo", "7", "bar", "99"];

    // 解析数字，非数字自动过滤掉
    let numbers: Vec<i32> = strings
        .into_iter()
        .filter_map(|s| s.parse().ok())
        .collect();

    println!("{:?}", numbers); // [42, 7, 99]
}
```

**高阶洞察**：filter_map 结合了 filter 和 map，避免了使用 if let 和临时 Option，尤其适合 Result/Option 的转换。

---

# 5. fold：通用聚合器

```rust
fn main() {
    let numbers = 1..=5;

    // 手动实现乘积
    let product = numbers.fold(1, |acc, x| acc * x);
    println!("{}", product); // 120

    // 求最大值（带初始值）
    let max = [2, 8, 3, 1].iter().fold(i32::MIN, |acc, &x| acc.max(x));
    println!("{}", max); // 8

    // 甚至可以用来构建字符串
    let chars = ['a', 'b', 'c'];
    let s = chars.iter().fold(String::new(), |mut acc, &c| {
        acc.push(c);
        acc
    });
    println!("{}", s); // abc
}
```

**高阶洞察**：fold 是函数式编程中的基本归约算子，几乎所有集合聚合操作都可以用它实现。Rust 的 sum、product、max 等方法就是它的特化形式。

---

# 6. try_fold 与 try_for_each：支持提前短路

```rust
use std::num::ParseIntError;
fn main() -> Result<(), ParseIntError> {
    let strings = vec!["10", "20", "xx", "30"];

    // 尝试逐个求和，遇到非法数字就提前返回错误
    let sum: i32 = strings
        .iter()
        .try_fold(0, |acc, s| -> Result<i32, ParseIntError> {
            let n = s.parse::<i32>()?;
            Ok(acc + n)
        })?;   // 若遇到 "xx" 会立即返回 Err

    println!("Sum = {}", sum);
    Ok(())
}
```

**高阶洞察**：try_fold 允许在迭代过程中使用 ? 短路，比 fold + collect::<Result<_,_>> 更高效，因为不会先收集全部结果。

---

# 7. 有状态迭代：scan

```rust
fn main() {
    let numbers = 1..=6;

    // 累积和，但每一项都输出当前累计值（斐波那契式累积）
    let running_sum: Vec<i32> = numbers
        .scan(0, |state, x| {
            *state += x;
            Some(*state)   // 返回新状态
        })
        .collect();

    println!("{:?}", running_sum); // [1, 3, 6, 10, 15, 21]

    // 用 scan 实现一个简单的斐波那契数列生成器
    let fib = std::iter::repeat(())
        .scan((0, 1), |(a, b), _| {
            let next = *a + *b;
            let old_a = *a;
            *a = *b;
            *b = next;
            Some(old_a)
        })
        .take(10)
        .collect::<Vec<_>>();
    println!("{:?}", fib); // [0, 1, 1, 2, 3, 5, 8, 13, 21, 34]
}
```

**高阶洞察**：scan 允许你携带任意状态并在每次迭代中产生一个元素，是实现复杂状态机迭代（如滑动窗口、累加器变体）的绝佳工具。

---

# 8. 自定义迭代器：实现 Iterator

```rust
/// 一个产生斐波那契数列的迭代器
struct Fibonacci {
    curr: u64,
    next: u64,
}

impl Fibonacci {
    fn new() -> Self {
        Fibonacci { curr: 0, next: 1 }
    }
}

impl Iterator for Fibonacci {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.curr;
        self.curr = self.next;
        self.next = current + self.next;
        Some(current)   // 永远不返回 None -> 无限迭代器
    }
}

fn main() {
    let fib = Fibonacci::new();
    let first_ten: Vec<u64> = fib.take(10).collect();
    println!("{:?}", first_ten);
}
```

**高阶洞察**：实现 Iterator 只需提供 next 方法，Rust 会免费提供数十种适配器方法。你也可以实现 DoubleEndedIterator（如果支持反向）、ExactSizeIterator（如果长度已知）等 trait 来优化特定操作。

---

# 9. 无限流与惰性求值

```rust
fn main() {
    // 产生无限随机数流（使用 rand crate，但这里用简单举例）
    let infinite_odds = (0..)
        .filter(|x| x % 2 == 1)
        .map(|x| x * 3 + 1)
        .take(5)
        .collect::<Vec<_>>();

    println!("{:?}", infinite_odds); // [1, 7, 13, 19, 25]
}
```

**高阶洞察**：由于 Rust 迭代器是惰性的，无限迭代器可以安全使用，只要最后被 take 等限制终止。这让你能用声明式风格表达无穷序列。

---

# 10. 并行迭代器（Rayon）

```rust
use rayon::prelude::*;

fn main() {
    let data: Vec<i32> = (1..=100_000).collect();

    // 并行的 map-reduce
    let sum_of_squares: i32 = data
        .par_iter()          // 并行迭代
        .map(|&x| x * x)
        .sum();

    println!("{}", sum_of_squares);
}
```

**高阶洞察**：只需将 iter() 改为 par_iter()，Rayon 会自动将计算分配到线程池。但要注意：par_iter 下的 map/filter 等操作必须是线程安全的（即 Send + Sync）。

---

# 11. 错误收集：collect<Result<Vec<_>, _>>

```rust
fn main() {
    let strings = vec!["42", "7", "not a number"];

    // 尝试将所有字符串转为数字，一旦有错误就得到 Err
    let result: Result<Vec<i32>, _> = strings
        .iter()
        .map(|s| s.parse::<i32>())
        .collect();

    match result {
        Ok(nums) => println!("Parsed: {:?}", nums),
        Err(e) => println!("Failed: {}", e),
    }
}
```

**高阶洞察**：collect 非常智能 – 当从 Iterator<Item = Result<T, E>> 收集到 Result<Vec<T>, E> 时，它会短路在第一个错误处。这是 try_fold 的一个便捷替代。

---

# 12. partition：一次性分割成两个集合

```rust
fn main() {
    let numbers = vec![1, 2, 3, 4, 5, 6];

    let (even, odd): (Vec<_>, Vec<_>) = numbers
        .into_iter()
        .partition(|&x| x % 2 == 0);

    println!("Even: {:?}, Odd: {:?}", even, odd);
}
```

**高阶洞察**：partition 内部使用两次收集，比手动 filter 两次更高效（只遍历一次）。

---

# 13. 比较两个迭代器：cmp、eq、partial_cmp

```rust
fn main() {
    let a = [1, 2, 3];
    let b = [1, 2, 3];
    let c = [1, 3, 2];

    println!("a == b: {}", a.iter().eq(b.iter()));    // true
    println!("a == c: {}", a.iter().eq(c.iter()));    // false
    println!("a < c: {:?}", a.iter().cmp(c.iter()));  // Less
}
```

**高阶洞察**：这些方法会逐个元素比较，遇到差异立即停止。当你需要判定两个序列之间的词典序关系时非常有用。

---

# 14. 使用 cycle 轮转生成重复模式

```rust
fn main() {
    let colors = ["red", "green", "blue"];
    let infinite_color_cycle = colors.iter().cycle();

    let first_10: Vec<_> = infinite_color_cycle.take(10).collect();
    println!("{:?}", first_10);
    // ["red", "green", "blue", "red", "green", "blue", ...]
}
```

**高阶洞察**：cycle 可以无限重复一个序列，配合 zip 可用于给另一个序列附上周期标签。

---

# 15. zip 与最短板对齐

```rust
fn main() {
    let names = ["Alice", "Bob", "Charlie"];
    let scores = [100, 80];

    // 自动截断到较短的那个
    let combined: Vec<_> = names.iter().zip(scores.iter()).collect();
    println!("{:?}", combined); // [("Alice", 100), ("Bob", 80)]
}
```

---

# 16. 高级适配器：inspect 用于调试

```rust
fn main() {
    let _: Vec<_> = (1..=3)
        .inspect(|x| println!("before: {}", x))
        .map(|x| x * 2)
        .inspect(|x| println!("after: {}", x))
        .collect();
}
```

**高阶洞察**：inspect 让你窥视迭代链中间状态而不改变元素，是调试长链的利器。

---

# 总结 – 高阶理解的要点

1. **惰性是默认的** – 适配器只是构建蓝图，消费者才执行。
2. **零成本抽象** – 组合子链的性能与手写最优循环相同。
3. **所有权及可变性** – 通过 iter/iter_mut/into_iter 精确控制。
4. **错误处理** – try_fold、collect<Result<..>> 和 ? 在迭代中优雅工作。
5. **自定义迭代器** – 通过实现 Iterator，可以无缝使用所有内置适配器。
6. **并行化** – 借助 Rayon 将 iter 替换为 par_iter 即可获得多核加速。
7. **无限与有限** – 无限迭代器必须搭配 take 等限制器，否则无穷循环。
8. **收集器的智能** – collect 不仅能生成 Vec，还能生成 Result、HashMap 等，取决于目标类型。

掌握这些例子和概念后，你就能像工匠一样组合 Rust 迭代器，写出简洁、高效、可读性强的代码。
