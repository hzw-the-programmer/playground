use std::ops::Deref;

fn main() {
    let s = S {
        id: 0,
        obj: Obj { id: 0 },
    };
    s.objf1();
    //func(s);
    func(&s);
    println!("{:?}", s);
}

trait I {
    fn f1(&self) {}
}

#[derive(Debug)]
struct Obj {
    id: i32,
}

impl Obj {
    fn objf1(&self) {
        println!("{:?}", self);
    }
}

impl I for Obj {
    fn f1(&self) {}
}

#[derive(Debug)]
struct S {
    id: i32,
    obj: Obj,
}

impl I for S {
    fn f1(&self) {}
}

impl Deref for S {
    type Target = Obj;
    fn deref(&self) -> &Self::Target {
        &self.obj
    }
}

impl<T> I for &T
where
    T: I + std::fmt::Debug,
{
    fn f1(&self) {
        println!("&T: {:?}", self);
    }
}

fn func<T: I>(t: T) {
    t.f1();
}
