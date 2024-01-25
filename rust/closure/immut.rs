// rustup default nightly
// rustc immut.rs -Z dump-mir=all

// mir_dump/immut.main.-------.nll.0.mir
// bb0: {
//     StorageLive(_1);
//     _1 = const 10_i32;
//     FakeRead(ForLet(None), _1);
//     AscribeUserType(_1, o, UserTypeProjection { base: UserType(1), projs: [] });
//     StorageLive(_2);
//     StorageLive(_3);
//     StorageLive(_4);
//     _4 = &_1;
//     _3 = {closure@immut.rs:9:13: 9:15} { x: move _4 };
//     StorageDead(_4);
//     _2 = closure::<{closure@immut.rs:9:13: 9:15}>(move _3) -> [return: bb1, unwind: bb5];
// }

fn closure(f: impl Fn()) {
    f();
}

fn main() {
    let x: i32 = 10;
    closure(|| println!("Hi {}", x));  // The closure just reads x.
    println!("Value of x after return {}", x);
}