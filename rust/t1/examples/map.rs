use std::collections::HashMap;

use t1::Foo;

fn main() {
    test1();
    test2();
    test3();
}

fn test1() {
    print!("\n* test1\n");

    let mut map = HashMap::new();

    assert_eq!(map.insert(37, "a"), None);
    assert_eq!(map.is_empty(), false);

    map.insert(37, "b");

    assert_eq!(map.insert(37, "c"), Some("b"));
    assert_eq!(map[&37], "c");
}

fn test2() {
    print!("\n* test2\n");

    let mut map = HashMap::new();
    map.insert(1, Foo { id: 1 });
    map.insert(2, Foo { id: 2 });

    {
        let f = map.insert(1, Foo { id: 3 }).unwrap();
    }
    println!("finish");
}

fn test3() {
    print!("\n* test3\n");
    let mut map = HashMap::new();
    map.insert(1, Foo { id: 1 });
    map.insert(2, Foo { id: 2 });
    map.remove(&2);
    println!("finish");
}
