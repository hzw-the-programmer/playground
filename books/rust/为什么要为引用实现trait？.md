# trait 方法的第一个参数是 &self

只需为 Foo 实现 ByRef，&Foo 以及 &&Foo 就可以调用 by_ref 函数。

```rust
trait ByRef {
    fn by_ref(&self);
}

struct Foo;

impl ByRef for Foo {
    fn by_ref(&self) {
        println!("Foo.by_ref");
    }
}

/*
  输出：
  Foo.by_ref
  Foo.by_ref
  Foo.by_ref
*/
fn main() {
    let foo = Foo;
    foo.by_ref();
    
    let rfoo = &Foo;
    rfoo.by_ref();
    
    let rrfoo = &&Foo;
    rrfoo.by_ref();
}
```

再看为 Foo 和 &Foo 同时实现 ByRef 后，会是什么输出

```rust
trait ByRef {
    fn by_ref(&self);
}

struct Foo;

impl ByRef for Foo {
    fn by_ref(&self) {
        println!("Foo.by_ref");
    }
}

impl ByRef for &Foo {
    fn by_ref(&self) {
        println!("&Foo.by_ref");
    }
}

/*
  输出：
  Foo.by_ref
  Foo.by_ref
  &Foo.by_ref
*/
fn main() {
    let foo = Foo;
    foo.by_ref();
    
    let rfoo = &Foo;
    rfoo.by_ref();
    
    let rrfoo = &&Bar;
    rrfoo.by_ref();
}
```

输出表明，即使为 &Foo 实现了 ByRef，rfoo.by_ref() 任然调用的是 Foo.by_ref。
rrfoo.by_ref() 有变化，调用的是 &Bar.by_ref。

那为什么要为 &Foo 实现 ByRef 呢？不是多次一举吗？

# trait 方法的第一个参数是 self

下面这个trait，就必须为 &Foo 实现 ByVal 了，否则 rfoo 调用 by_val 会编译报错。

```rust
trait ByVal {
    fn by_val(self);
}

struct Foo;

impl ByVal for Foo {
    fn by_val(self) {
        println!("Foo.by_val");
    }
}

fn main() {
    let foo = Foo;
    foo.by_val();

    let rfoo = &Foo;
    // rfoo.by_val(); // cannot move out of `*rfoo` which is behind a shared reference
    // ByVal::by_val(rfoo); // the trait bound `&by_val_1::Foo: ByVal` is not satisfied
    // ByVal::by_val(*rfoo); // cannot move out of `*rfoo` which is behind a shared reference
}
```

为 &Foo 实现 ByVal

```rust
trait ByVal {
    fn by_val(self);
}

struct Foo;

impl ByVal for Foo {
    fn by_val(self) {
        println!("Foo.by_val");
    }
}

impl ByVal for &Foo {
    fn by_val(self) {
        println!("&Foo.by_val");
    }
}

fn main() {
    let foo = Foo;
    foo.by_val();

    let rfoo = &Foo;
    rfoo.by_val();
    ByVal::by_val(rfoo);
    // ByVal::by_val(*rfoo); // cannot move out of `*rfoo` which is behind a shared reference
}
```

# 这解释了迭代器为什么可以有三种实现了

先看 IntoIterator into_iter 方法第一个参数是 self

```rust
pub trait IntoIterator {
    type Item;
    type IntoIter: Iterator<Item = Self::Item>;
    fn into_iter(self) -> Self::IntoIter;
}
```

再看 Vec 是怎么实现这个 trait 的

消耗实现

```rust
impl<T, A: Allocator> IntoIterator for Vec<T, A> {
    type Item = T;
    type IntoIter = IntoIter<T, A>;
    fn into_iter(self) -> Self::IntoIter {
    }
}
```

引用实现

```rust
impl<'a, T, A: Allocator> IntoIterator for &'a Vec<T, A> {
    type Item = &'a T;
    type IntoIter = slice::Iter<'a, T>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
```

可变引用实现

```rust
impl<'a, T, A: Allocator> IntoIterator for &'a mut Vec<T, A> {
    type Item = &'a mut T;
    type IntoIter = slice::IterMut<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}
```

# 也解释了 Bytes 的 Buf trait 的 take 方法

```rust
pub trait Buf {
    fn copy_to_bytes(&mut self, len: usize) -> crate::Bytes {
        ...
        // 如果引用没实现 Buf，这里调用 take 会编译不过
        ret.put(self.take(len));
        ...
    }

    fn take(self, limit: usize) -> Take<Self>
    where
        Self: Sized,
    {
        take::new(self, limit)
    }
}

macro_rules! deref_forward_buf {
    () => {
        #[inline]
        fn remaining(&self) -> usize {
            (**self).remaining()
        }
        
        ...
    };
}

impl<T: Buf + ?Sized> Buf for &mut T {
    deref_forward_buf!();
}
```
