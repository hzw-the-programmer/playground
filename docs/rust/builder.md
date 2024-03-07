builder 模式：
```
struct Multiply {
    num: u16,
    factor: u16,
}

impl Multiply {
    /// Construct a new instance of `Multiply`.
    pub fn new(num: u16, factor: u16) -> Self {
        Self { num, factor }
    }

    /// Set the number to multiply by the factor.
    pub fn number(mut self, num: u16) -> Self {
        self.num = num;
        self
    }

    /// Set the factor to multiply the number with.
    pub fn factor(mut self, factor: u16) -> Self {
        self.factor = factor;
        self
    }
}
```

调用方法一：
```
fn main() {
    let a = Multiply::new(2, 3);
    let b = a.number(3);
    let c = b.factor(4);
    // println!("{} {}", a.num, a.factor);
    // println!("{} {}", b.num, b.factor);
    println!("{} {}", c.num, c.factor);
}
```
a 不为 mut，也可以 mut 传到 number 里面

调用方法二：
```
fn main() {
    let d = Multiply::new(0, 0).number(3).factor(4);
    println!("{} {}", d.num, d.factor);
}
```