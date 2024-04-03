trait Buf {
    fn copy_to_bytes(&mut self) {
        println!("copy_to_bytes");
        self.take();
    }

    fn take(self)
    where
        Self: Sized,
    {
        println!("take");
    }
}

struct Foo {
    id: i32,
}

impl Drop for Foo {
    fn drop(&mut self) {
        println!("drop {}", self.id);
    }
}

impl Buf for Foo {}

impl<T: Buf + ?Sized> Buf for &mut T {}

pub fn test() {
    let foo = Foo { id: 1 };
    foo.take();
    let mut foo = Foo { id: 2 };
    foo.copy_to_bytes();
    println!("finish");
}
