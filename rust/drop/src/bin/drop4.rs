trait Show {
    fn showr(&self);
    fn showv(self);
}

fn f1<T: Show>(t: T) {
    t.showr();
    t.showv();
    println!("f1 end");
}

struct Item {
    i: i32,
}

impl Show for Item {
    fn showr(&self) {
        println!("{} show_by_ref", self.i);
    }
    fn showv(self) {
        println!("{} show_by_value", self.i);
    }
}

impl Drop for Item {
    fn drop(&mut self) {
        println!("{} drop", self.i);
    }
}

fn main() {
    let i0 = Item { i: 0 };
    f1(i0);
    let i1 = &Item { i: 1 };
    f1(i1);
    println!("main end");
}
