//https://doc.rust-lang.org/book/ch07-02-defining-modules-to-control-scope-and-privacy.html

pub struct Foo {
    pub id: usize,
}

impl Drop for Foo {
    fn drop(&mut self) {
        println!("Foo {} drop", self.id);
    }
}

mod num2str;
mod nums_sum;
