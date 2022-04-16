struct Item {
    i: i32,
}

impl Drop for Item {
    fn drop(&mut self) {
        println!("Item {} droped", self.i);
    }
}

impl Item {
    fn bref(&self) {}
    fn bval(self) {}
}

impl AsRef<Item> for Item {
    fn as_ref(&self) -> &Item {
        self
    }
}

fn f1<T>(d: T) {
}

fn f2<T: AsRef<Item>>(d: T) {
}

trait Show {
    fn showr(&self);
    fn showv(self) where Self: Sized;
}

impl Show for Item {
    fn showr(&self) {
        println!("showr");
    }
    fn showv(self) {
        println!("showv");
    }
}

fn f3<T: Show>(d: T) {
    d.showr();
    d.showv();
    println!("f3 end");
}

impl<T> Show for &T
where
    T: Show {
    fn showr(&self) {
        println!("&T showr");
    }
    fn showv(self) {
        println!("&T showv");
    }
}

impl<T> Show for &mut T
where
    T: Show {
    fn showr(&self) {
        println!("&mut T showr");
    }
    fn showv(self) {
        println!("&mut T showv");
    }
}

fn main() {
    let d0 = Item{i: 0};
    let d1 = Item{i: 1};
    let d2 = Item{i: 2};
    let d3 = Item{i: 3};
    let d4 = &Item{i: 4};
    let d5 = Item{i: 5};
    let d6 = &mut Item{i: 6};
    let d7 = Item{i: 7};
    let d8 = &Item{i: 8};
    let d9 = &mut Item{i: 9};
    f1(d0);
    f1(&d1);
    d2.bref();
    d3.bref();
    d3.bval();
    d4.bref();
    //d4.bval();
    let d = d4.as_ref();
    let d = d5.as_ref();
    f2(d4);
    f2(d5);
    f2(d6);
    f3(d7);
    f3(d8);
    f3(d9);
    println!("end");
}
