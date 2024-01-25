// rustup default nightly
// rustc mut.rs -Z dump-mir=all

// mir_dump/mut.main.-------.nll.0.mir
// bb0: {
//     StorageLive(_1);
//     _1 = const 10_i32;
//     FakeRead(ForLet(None), _1);
//     AscribeUserType(_1, o, UserTypeProjection { base: UserType(1), projs: [] });
//     StorageLive(_2);
//     StorageLive(_3);
//     StorageLive(_4);
//     _4 = &mut _1;
//     _3 = {closure@mut.rs:7:13: 7:15} { x: move _4 };
//     StorageDead(_4);
//     _2 = closure::<{closure@mut.rs:7:13: 7:15}>(move _3) -> [return: bb1, unwind: bb5];
// }

fn closure(mut f: impl FnMut()) {
    f();
}

fn main() {
    let mut x: i32 = 10;
    closure(|| {
        x += 10;  // The closure mutates the value of x
        println!("Hi {}", x)
    });
    println!("Value of x after return {}", x);
}