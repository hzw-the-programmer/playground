std::rc::Rc 为啥是只读的？
因为没有实现 DerefMut

```
use std::rc::Rc;

struct Foo {
    id: i32,
}

impl Foo {
    fn f1(&self) {
        println!("Foo.f1 called");
    }

    fn f2(&mut self) {
        println!("Foo.f2 called");
    }
}

fn main() {
    let mut rc = Rc::new(Foo { id: 1 });
    rc.f1();
    rc.f2();
}
```

编译报错：
```
error[E0596]: cannot borrow data in an `Rc` as mutable
  --> examples\rc.rs:20:5
   |
20 |     rc.f2();
   |     ^^ cannot borrow as mutable
   |
   = help: trait `DerefMut` is required to modify through a dereference, but it is not implemented for `Rc<Foo>`
```