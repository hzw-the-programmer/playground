Rust 变量遮蔽(shadowing)

Rust 允许声明相同的变量名，在后面声明的变量会遮蔽掉前面声明的。
这和 mut 变量的使用是不同的，第二个 let 生成了完全不同的新变量，两个变量只是恰好拥有同样的名称，涉及一次内存对象的再分配 ，
而 mut 声明的变量，可以修改同一个内存地址上的值，并不会发生内存对象的再分配，性能要更好。

```
struct Foo {
    id: i32,
}

impl Drop for Foo {
    fn drop(&mut self) {
        println!("Foo {} drop", self.id);
    }
}

fn test1() {
    print!("test1 begin\n\n");
    let f = Foo { id: 1 };
    let f = Foo { id: 2 };
    println!("test1 end");
}

fn test2() {
    print!("\ntest2 begin\n\n");
    let mut f = Foo { id: 1 };
    f = Foo { id: 2 };
    println!("test2 end");
}

fn main() {
    test1();
    test2();
}
```

```
test1 begin

test1 end
Foo 2 drop
Foo 1 drop

test2 begin

Foo 1 drop
test2 end
Foo 2 drop
```