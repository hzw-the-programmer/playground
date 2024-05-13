struct Foo {
    x: i32,
    y: i32,
}

fn main() {
    let foo = &Foo { x: 1, y: 1 };
    let a = &[1, 2, 3];

    match (foo, a) {
        (&Foo { ref x, y }, &[ref first, second, _]) => {
            println!("x={}, y={}, first={}, second={}", *x, y, *first, second);
        }
    }

    match (foo, a) {
        (Foo { x, y }, [first, second, _]) => {
            println!("x={}, y={}, first={}, second={}", *x, *y, *first, *second);
        }
    }
}
