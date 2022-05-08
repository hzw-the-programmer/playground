use drop::Object;

fn main() {
    let mut v1 = vec![
        Object { id: 0 },
        Object { id: 1 },
        Object { id: 2 },
        Object { id: 3 },
        Object { id: 4 },
    ];
    println!(
        "v1.len() == {}, v1.capacity() = {}",
        v1.len(),
        v1.capacity(),
    );
    let e = &v1[0];
    //let i = v1.iter();
    v1.push(Object { id: 5 });
    println!(
        "v1.len() == {}, v1.capacity() = {}",
        v1.len(),
        v1.capacity(),
    );
    println!("{:?}", e);
    // for e in i {
    //     println!("{:?}", e);
    // }
}