macro_rules! tiny_map {
    // 基本情况：空列表
    ({}) => { ::std::collections::HashMap::new() };
    // 处理一个键值对
    ({ $($key:expr => $value:expr),* $(,)? }) => {
        {
            let mut map = ::std::collections::HashMap::new();
            $( map.insert($key, $value); )*
            map
        }
    };
}

fn main() {
    let m = tiny_map!({});
    let m = tiny_map!({ "a" => 1, "b" => 2 });
}

/*
cargo expand --example tiny_map

#![feature(prelude_import)]
#[macro_use]
extern crate std;
#[prelude_import]
use std::prelude::rust_2024::*;
fn main() {
    let m = ::std::collections::HashMap::new();
    let m = {
        let mut map = ::std::collections::HashMap::new();
        map.insert("a", 1);
        map.insert("b", 2);
        map
    };
}
*/
