// cargo run
// no error

// cd src/
// rustc main.rs
// borrow of moved value: `f`

// Cargo.toml
// [package]
// edition = "2021"

// https://doc.rust-lang.org/edition-guide/rust-2021/disjoint-capture-in-closures.html

struct Foo {
    id: i32,
}

impl Foo {
    fn work(&self) {}
}

fn main() {
    let f = Foo { id: 1 };
    {
        let c = move || {
            println!("{}", f.id)
        };
        c();
    }
    f.work();
}
