struct Item {
    i: i32,
}

impl Drop for Item {
    fn drop(&mut self) {
        println!("Item {} droped", self.i);
    }
}

impl Item {
    fn bref(&self) {
        println!("{} bref", self.i);
    }
    fn bval(self) {
        println!("{} bval", self.i);
    }
}

fn f1<T>(d: T) {
    println!("f1: {}", std::any::type_name::<T>());
}

impl AsRef<Item> for Item {
    fn as_ref(&self) -> &Item {
        println!("as_ref: {} {}", std::any::type_name::<Self>(), self.i);
        self
    }
}

fn f2<T: AsRef<Item>>(d: T) {
    println!("f2: {}", std::any::type_name::<T>());
    d.as_ref();
}

trait Show {
    fn showr(&self);
    fn showv(self)
    where
        Self: Sized;
}

impl Show for Item {
    fn showr(&self) {
        println!("showr: {}, T", std::any::type_name::<Self>());
    }
    fn showv(self) {
        println!("showv: {}, T", std::any::type_name::<Self>());
    }
}

fn f3<T: Show>(d: T) {
    println!("f3: {}", std::any::type_name::<T>());
    d.showr();
    d.showv();
}

impl<T> Show for &T
where
    T: Show,
{
    fn showr(&self) {
        println!("showr: {}, &T", std::any::type_name::<T>());
    }
    fn showv(self) {
        println!("showv: {}, &T", std::any::type_name::<T>());
    }
}

impl<T> Show for &mut T
where
    T: Show,
{
    fn showr(&self) {
        println!("showr: {}, &mut T", std::any::type_name::<T>());
    }
    fn showv(self) {
        println!("showv: {}, &mut T", std::any::type_name::<T>());
    }
}

fn f4<T: Show>(d: &T) {
    println!("f4: {}", std::any::type_name::<T>());
    d.showr();
    d.showv();
}

fn test_bref_bval() {
    let d0 = Item { i: 0 };
    println!("d0.bref/bval begin");
    d0.bref();
    d0.bval();
    println!("d0.bref/bval end");

    let d1 = &Item { i: 1 };
    println!("d1.bref/bval begin");
    d1.bref();
    //d1.bval();
    println!("d1.bref/bval end");
}

fn test_f1() {
    let d0 = Item { i: 0 };
    println!("f1(d0) begin");
    f1(d0);
    println!("f1(d0) end");

    let mut d1 = Item { i: 1 };
    println!("f1(&d1) begin");
    f1(&d1);
    println!("f1(&d1) end");

    println!("f1(&mut d1) begin");
    f1(&mut d1);
    println!("f1(&mut d1) end");
}

fn test_f2() {
    let d0 = Item { i: 0 };
    println!("f2(d0) begin");
    f2(d0);
    println!("f2(d0) end");

    let mut d1 = Item { i: 1 };
    println!("f2(&d1) begin");
    f2(&d1);
    println!("f2(&d1) end");

    println!("f2(&mut d1) begin");
    f2(&mut d1);
    println!("f2(&mut d1) end");
}

fn test_f3() {
    let d0 = Item { i: 0 };
    println!("f3(d0) begin");
    f3(d0);
    println!("f3(d0) end");

    let mut d1 = Item { i: 1 };
    println!("f3(&d1) begin");
    f3(&d1);
    println!("f3(&d1) end");

    println!("f3(&mut d1) begin");
    f3(&mut d1);
    println!("f3(&mut d1) end");
}

fn test_f4() {
    let mut d1 = Item { i: 1 };
    println!("f4(&d1) begin");
    f4(&d1);
    println!("f4(&d1) end");

    println!("f4(&mut d1) begin");
    f4(&mut d1);
    println!("f4(&mut d1) end");

    println!("f4(&&d1) begin");
    f4(&&d1);
    println!("f4(&&d1) end");
}

fn main() {
    test_bref_bval();
    println!("");

    test_f1();
    println!("");

    test_f2();
    println!("");

    test_f3();
    println!("");

    test_f4();
    println!("");
}
