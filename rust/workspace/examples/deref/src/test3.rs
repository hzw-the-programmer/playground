#![allow(dead_code)]

pub fn test() {}

enum Foo<C> {
    One(C),
    Two(C),
}

struct Bar(Foo<Box<Baz>>);

impl Bar {
    fn data(&self) -> Foo<&Baz> {
        match &self.0 {
            Foo::One(c) => {
                // let i: i32 = c;
                Foo::One(c)
            }
            Foo::Two(c) => Foo::Two(&*c),
        }
    }

    fn data_mut(&mut self) -> Foo<&mut Baz> {
        match &mut self.0 {
            Foo::One(c) => {
                // let i: i32 = c;
                Foo::One(c)
            }
            Foo::Two(c) => {
                // let i: i32 = c;
                Foo::Two(&mut *c)
            }
        }
    }

    // fn data_1(&self) -> Foo<&Baz> {
    //     match self.0 {
    //         Foo::One(c) => {
    //             // let i: i32 = c;
    //             Foo::One(&c)
    //         }
    //         Foo::Two(c) => {
    //             Foo::Two(&*c)
    //         }
    //     }
    // }
}

struct Baz;
