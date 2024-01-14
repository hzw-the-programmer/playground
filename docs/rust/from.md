# rust 的 From, Into trait

## From trait

### 源码
$(RUSTUP_HOME)\toolchains\stable-x86_64-pc-windows-msvc\lib\rustlib\src\rust\library\core\src\convert\mod.rs:538

### 定义
```
pub trait From<T>: Sized {
    /// Converts to this type from the input type.
    fn from(value: T) -> Self;
}
```

### 示例
```
struct A {
    i: i32,
}

#[derive(Debug)]
struct B {
    i: i32,
}

impl From<A> for B {
    fn from(a: A) -> Self {
        B { i: a.i + 1 }
    }
}

fn main() {
    let a1 = A { i: 1 };
    let b1 = B::from(a1);
    println!("{:?}", b1);
}
```

## Into trait

### 源码
$(RUSTUP_HOME)\toolchains\stable-x86_64-pc-windows-msvc\lib\rustlib\src\rust\library\core\src\convert\mod.rs:444

### 定义
```
pub trait Into<T>: Sized {
    /// Converts this type into the (usually inferred) input type.
    fn into(self) -> T;
}
```

### 示例
```
struct A {
    i: i32,
}

#[derive(Debug)]
struct B {
    i: i32,
}

impl From<A> for B {
    fn from(a: A) -> Self {
        B { i: a.i + 1 }
    }
}

fn main() {
    let a2 = A { i: 2 };
    let b2: B = a2.into();
    println!("{:?}", b2);
}
```

### 疑问？
A没有实现Into trait，为什么可以调用into方法呢？
因为有个blanket实现
$(RUSTUP_HOME)\toolchains\stable-x86_64-pc-windows-msvc\lib\rustlib\src\rust\library\core\src\convert\mod.rs:706
```
impl<T, U> Into<U> for T
where
    U: From<T>,
{
    /// Calls `U::from(self)`.
    #[inline]
    fn into(self) -> U {
        U::from(self)
    }
}
```
