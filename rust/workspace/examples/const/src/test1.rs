// struct S(i32);
struct S {
    x: i32,
}

// const S: S = S(2);
const S: S = S { x: 2 };

pub fn test() {
    let v = &mut S;
    // v.0 += 1;
    // S.0 += 1;
    // println!("{} {}", v.0, S.0);
    v.x += 1;
    S.x += 1;
    println!("{} {}", v.x, S.x);
}
