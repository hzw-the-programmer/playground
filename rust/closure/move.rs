// rustup default nightly
// rustc move.rs -Z dump-mir=all

// mir_dump/move.main.-------.nll.0.mir
// bb4: {
//     StorageDead(_2);
//     FakeRead(ForLet(None), _1);
//     StorageLive(_8);
//     StorageLive(_9);
//     _9 = {closure@move.rs:7:13: 7:15} { x: move _1 };
//     _8 = closure::<{closure@move.rs:7:13: 7:15}>(move _9) -> [return: bb5, unwind: bb7];
// }

fn closure(f: impl FnOnce()) {
    f();
}

fn main() {
    let x = vec![21];
    closure(|| {
        drop(x);  // Makes x unusable after the fact.
    });
    // println!("Value of x after return {:?}", x);
}