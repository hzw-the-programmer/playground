use super::DisjointSet;

#[test]
fn test_disjoint_set() {
    let mut ds = DisjointSet::new(5);
    println!("{ds:?}");

    assert!(!ds.connected(0, 1));
    assert!(!ds.connected(1, 2));
    assert!(!ds.connected(2, 3));
    assert!(!ds.connected(3, 4));

    ds.union(0, 1);
    println!("{ds:?}");
    ds.union(2, 3);
    println!("{ds:?}");

    assert!(ds.connected(0, 1));
    assert!(!ds.connected(0, 2));
    assert!(ds.connected(2, 3));
    assert!(!ds.connected(1, 3));

    ds.union(1, 3);
    println!("{ds:?}");
    assert!(ds.connected(0, 2));
    assert!(ds.connected(0, 3));
    assert!(ds.connected(1, 2));
    assert!(ds.connected(1, 3));
}

#[test]
fn test_union_1() {
    let mut ds = DisjointSet::new(9);
    println!("{ds:?}");
    ds.union(0, 1);
    println!("{ds:?}");
    ds.union(1, 2);
    println!("{ds:?}");
    ds.union(2, 3);
    println!("{ds:?}");
    ds.union(3, 4);
    println!("{ds:?}");
    ds.union(4, 5);
    println!("{ds:?}");
    ds.union(5, 6);
    println!("{ds:?}");
    ds.union(6, 7);
    println!("{ds:?}");
    ds.union(7, 8);
    println!("{ds:?}");
}

#[test]
fn test_union_2() {
    let mut ds = DisjointSet::new(9);
    println!("{ds:?}");
    ds.union(1, 0);
    println!("{ds:?}");
    ds.union(2, 1);
    println!("{ds:?}");
    ds.union(3, 2);
    println!("{ds:?}");
    ds.union(4, 3);
    println!("{ds:?}");
    ds.union(5, 4);
    println!("{ds:?}");
    ds.union(6, 5);
    println!("{ds:?}");
    ds.union(7, 6);
    println!("{ds:?}");
    ds.union(8, 7);
    println!("{ds:?}");
}

#[test]
fn test_union_3() {
    let mut ds = DisjointSet::new(10);

    ds.union(1, 3);
    ds.union(2, 3);
    ds.union(3, 0);
    ds.union(0, 4);
    ds.union(4, 9);
    println!("{ds:?}");

    ds.union(5, 9);
    println!("{ds:?}");

    ds.union(7, 6);
    ds.union(8, 6);
    ds.union(6, 9);
    println!("{ds:?}");
}
