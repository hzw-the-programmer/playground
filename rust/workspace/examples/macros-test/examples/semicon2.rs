macro_rules! demo {
    () => {
        println!("hello");
    };
}

fn main() {
    demo!(); // 正确
    let _x = 0;
    // demo!()
    // let _x = 0;
}
