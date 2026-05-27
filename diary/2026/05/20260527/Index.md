`Index` 和 `IndexMut` 是 Rust 标准库中 `std::ops` 模块下的两个 trait，它们为类型提供了使用 索引操作符 `[]` 的能力。简单来说，只要为一个类型实现了 `Index<T>`，就可以用 `value[index]` 的语法来读取数据；实现了 `IndexMut<T>`，就可以用 `value[index] = new_value` 的语法来写入数据。

---

# 1. Trait 定义

```rust
pub trait Index<Idx: ?Sized> {
    type Output: ?Sized;
    fn index(&self, index: Idx) -> &Self::Output;
}

pub trait IndexMut<Idx: ?Sized>: Index<Idx> {
    fn index_mut(&mut self, index: Idx) -> &mut Self::Output;
}
```

- `Idx`：索引的类型。可以是 `usize`、`Range<usize>`、`&str`，甚至是自定义类型。`?Sized` 约束表示可以接受动态大小类型（如 `str`），这使字符串切片 `&str` 也能作为索引。

- `Output`：索引操作返回的值的类型。它决定了 `container[index]` 的表达式的类型。

- `index` 方法：接受一个不可变引用 `&self`，返回一个不可变引用 `&Self::Output`。

- `index_mut` 方法：接受一个可变引用 `&mut self`，返回一个可变引用 `&mut Self::Output`。

注意 `IndexMut<Idx>` 继承自 `Index<Idx>`：这意味着实现了 `IndexMut` 的类型必须同时实现 `Index`。编译器会强制要求，如果你实现了 `IndexMut`，就必须提供对应的 `Index` 实现，不能只写可变索引而不提供只读索引。

---

# 2. 索引操作符的解糖

当你写下这样的代码：

```rust
let x = container[index];          // 读取
container[index] = value;          // 写入
```

编译器会将其展开为：

```rust
// 读取
let x = *container.index(index);
// 或者 *std::ops::Index::index(&container, index);

// 写入（实际会先取出可变引用，再赋值）
*container.index_mut(index) = value;
```

也就是说，`[]` 操作的背后是一个方法调用，然后立即解引用（`*`）以获得最终值。也正是因为需要立即解引用，`index` 方法必须返回一个引用，否则所有权会被移出。

---

# 3. 关联类型 `Output` 的设计

`Output` 并不强制是 `Idx` 对应的某个字段类型，它可以是任何与容器和索引相关的类型。这带来了极大的灵活性：

- `Vec<T>` 实现 `Index<usize>`，`Output = T`，所以 `vec[0]` 返回 `&T`。

- `HashMap<K, V>` 实现 `Index<&K>`，`Output = V`，所以 `map[&key]` 返回 `&V`。

- `[T]` 切片实现 `Index<Range<usize>>`，`Output = [T]`，所以 `&slice[0..2]` 返回 `&[T]`。

- 字符串 `str` 实现 `Index<Range<usize>>`，`Output = str`，所以 `&"hello"[0..2]` 返回 `&str`。

通过 `Output`，你可以让 `container[index]` 返回复合类型、切片，甚至其他自定义类型。

---

# 4. 为自定义类型实现 `Index` / `IndexMut`

假设我们要实现一个简单的二维数组 `Matrix`，可以用 `(usize, usize)` 作为索引。

```rust
use std::ops::{Index, IndexMut};

struct Matrix {
    data: Vec<f64>,
    rows: usize,
    cols: usize,
}

impl Matrix {
    fn new(rows: usize, cols: usize) -> Self {
        Matrix {
            data: vec![0.0; rows * cols],
            rows,
            cols,
        }
    }
}

// 实现 Index<(usize, usize)>
impl Index<(usize, usize)> for Matrix {
    type Output = f64;

    fn index(&self, (row, col): (usize, usize)) -> &f64 {
        assert!(row < self.rows && col < self.cols, "索引越界");
        &self.data[row * self.cols + col]
    }
}

// 实现 IndexMut<(usize, usize)> （必须同时已有 Index 实现）
impl IndexMut<(usize, usize)> for Matrix {
    fn index_mut(&mut self, (row, col): (usize, usize)) -> &mut f64 {
        assert!(row < self.rows && col < self.cols, "索引越界");
        &mut self.data[row * self.cols + col]
    }
}
```

使用：

```rust
let mut mat = Matrix::new(3, 3);
mat[(1, 1)] = 5.0;          // 调用 index_mut
println!("{}", mat[(1, 1)]); // 调用 index
```

**注意**：

- 我们在 `index_mut` 中使用了和 `index` 完全相同的索引计算逻辑。因为两者无法互相复用，通常需要手动保持同步。

- 索引类型 `(usize, usize)` 是一个元组，你可以为其实现任意复杂的索引逻辑，比如支持 `(Range<usize>, usize)` 来获取一整列的部分行等。

---

# 5. `IndexMut` 与 `Index` 的强制绑定

为什么 `IndexMut` 必须依赖 `Index`？考虑以下场景：如果允许只实现 `IndexMut` 而不实现 `Index`，那么 `container[index]` 用于读取时就会编译失败，但用于写入时却可以。这会造成 API 的不一致性，并且在使用泛型时可能产生混乱。因此，Rust 通过 trait 继承确保了“可变索引”一定伴随着“只读索引”。

另外，当你对同一个索引同时需要读和写时，语言会强制你遵守借用规则——你不能在同一作用域内同时持有对同一个 `Matrix` 的不可变引用和可变引用。

---

# 6. 生命周期与借用

- `index(&self, index: Idx) -> &Self::Output` 返回的引用的生命周期与 `&self` 绑定，因此只要你持有索引返回的引用，就不能对容器进行可变操作。

- `index_mut(&mut self, index: Idx) -> &mut Self::Output` 返回的引用的生命周期与 `&mut self` 绑定，意味着在可变引用存在期间，容器不能被其他任何方式访问。

这保证了索引操作不会造成悬垂指针或数据竞争。

---

# 7. 与 `Deref` 的区别

- `Deref` 是隐式的：只要类型实现了 `Deref`，在方法调用、解引用等位置编译器会自动插入 `*`。

- `Index` 是显式的：只有当你写出 `[]` 语法时才会触发，编译器不会自动插入索引操作。

因此，`Index` 更像是为你的类型提供一种自定义的“下标访问”语法，而不会干扰正常的类型行为。

---

# 8. 错误处理：越界应该怎么做？

标准库中的 `Vec`、切片等会在索引越界时 **panic**。这是出于性能考虑：返回 `Option<&T>` 会引入分支，而 Rust 的原则是让调用者保证索引有效。如果你希望提供不 panic 的索引，需要提供额外的方法（例如 `get(index) -> Option<&T>`），而不是改变 `Index` 的实现。

在自定义实现中，你可以选择 panic、返回一个默认引用，甚至执行某种回绕逻辑，但为了符合用户预期，建议要么 panic，要么确保逻辑上不会越界（例如使用 wrapping 操作），并在文档中明确说明。

---

# 9. 常见实现模板

很多标准库和第三方类型都实现了这两个 trait，例如：

-`Vec<T>` 对 `usize`、`Range<usize>`、`RangeFrom<usize>` 等各种范围类型实现了索引。

- `[T; N]` 数组同样实现了多种索引类型。

- `HashMap<K, V>` 实现了 `Index<&K>`，注意索引是借用 `K` 而不是 `K` 本身（因为哈希查找不需要所有权）。

- `BTreeMap<K, V>` 同理。

- `str` 实现了对 `Range<usize>` 等范围的索引，返回 `&str`，但不能用 `usize` 直接索引到单个字符（因为 UTF-8 编码下字符不定长）。

---

# 10. 总结与最佳实践

- 何时实现：当你的类型是一个“集合”或“容器”，并且希望通过类似数组的语法访问内部元素时。

- `Output` 的选择：返回元素本身、元素的引用、或某个切片。避免返回临时值（因为方法签名要求返回引用，临时值无法被引用）。

- 保持 `index` 和 `index_mut` 逻辑一致：通常可变版本只是将不可变版本中的 `&` 换成 `&mut`，并增加写入权限。

- 考虑性能：索引操作通常应该非常廉价（O(1) 或接近），如果代价高昂，考虑提供命名方法而不是 `Index`，以免用户误以为它是轻量操作。

- 文档说明 panic 条件：如果索引可能 panic，必须在文档中清晰指出。

通过实现 `Index` 和 `IndexMut`，你可以让自定义类型无缝融入 Rust 的语言生态，享受和内置类型同样简洁直观的访问语法。
