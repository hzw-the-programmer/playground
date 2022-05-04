use drop::Object;

fn main() {
    let tests: Vec<fn()> = vec![test0, test1, test2, test3, test4, test5];

    drop::tests(&tests);
}

fn test0() {
    let objs = vec![
        Object { id: 0 },
        Object { id: 1 },
        Object { id: 2 },
        Object { id: 3 },
    ];

    for obj in objs {
        println!("{:?}", obj);
    }

    println!("test 0 end");
}

fn test1() {
    let objs = vec![
        Object { id: 0 },
        Object { id: 1 },
        Object { id: 2 },
        Object { id: 3 },
    ];

    for obj in objs {
        if obj.id == 2 {
            break;
        }
        println!("{:?}", obj);
    }

    println!("test 1 end");
}

fn test2() {
    let objs = vec![
        Object { id: 0 },
        Object { id: 1 },
        Object { id: 2 },
        Object { id: 3 },
    ];

    for obj in &objs {
        println!("{:?}", obj);
    }

    println!("test 2 end");
}

fn test3() {
    let objs = vec![0, 1, 2, 3];

    let mut i = 0;
    for obj in objs {
        assert_eq!(i, obj);
        i += 1;
    }
}

fn test4() {
    let objs = vec![0, 1, 2, 3];

    let mut i = 0;
    for obj in &objs {
        assert_eq!(&i, obj);
        i += 1;
    }
}

fn test5() {
    let objs = vec![0, 1, 2, 3];

    let mut i = 0;
    for &obj in &objs {
        assert_eq!(i, obj);
        i += 1;
    }
}

fn t1(objs: &[Object]) {
    for obj in objs {
        println!("{:?}", obj);
    }
}

/*
fn t2(objs: &[Object]) {
    for &obj in objs {
        println!("{:?}", obj);
    }
}
 */

fn largest(items: &[i32]) -> i32 {
    let mut largest = items[0];
    for item in items {
        if largest > *item {
            largest = *item;
        }
    }
    largest
}

fn largest_1(items: &[i32]) -> i32 {
    let mut largest = items[0];
    for &item in items {
        if largest > item {
            largest = item;
        }
    }
    largest
}
