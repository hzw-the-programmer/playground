struct Bar;

impl Bar {
    fn by_ref(&self) {
        println!("Bar::by_ref");
    }

    fn by_value(self) {
        println!("Bar::by_value");
    }
}

trait Foo {
    fn trait_by_ref(&self);
    fn trait_by_val(self);
}

impl Foo for Bar {
    fn trait_by_ref(&self) {
        println!("trait_by_ref");
    }

    fn trait_by_val(self) {
        println!("trait_by_val");
    }
}

fn fn1<T: Foo>(_t: T) {
    println!("fn1");
}

trait Foo2 {
    fn trait_foo2_by_ref(&self);
}

impl Foo2 for &Bar {
    fn trait_foo2_by_ref(&self) {
        println!("trait_foo2_by_ref");
    }
}

/*
error[E0390]: cannot define inherent `impl` for primitive types
impl &Bar {
    fn by_value(self) {
        println!("Bar::by_value");
    }

    fn by_ref(&self) {
        println!("Bar::by_ref");
    }
}
*/

pub fn test() {
    let bar = Bar;
    bar.by_ref();
    bar.by_value();

    let bar = Bar;
    bar.trait_by_ref();
    (&bar).trait_by_ref();

    bar.trait_by_val();

    // error[E0507]: cannot move out of a shared reference
    // let bar = Bar;
    // (&bar).trait_by_val();

    let bar = Bar;
    fn1(bar);

    // error[E0277]: the trait bound `&test3::Bar: test3::Foo` is not satisfied
    // the trait `test3::Foo` is not implemented for `&test3::Bar`
    // let bar = Bar;
    // fn1(&bar);

    // let bar = Bar;
    // bar.trait_foo2_by_ref();

    let bar = Bar;
    (&bar).trait_foo2_by_ref();
}
