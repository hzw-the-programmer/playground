将所有宏放到单独的宏模块，在 lib.rs 文件开头，用 #[macro_use] 导出宏
如 mio-0.8.10\src\lib.rs
```
#[macro_use]
mod macros;
```
这样整个 crate 都可以使用宏，不需要额外 use

使用 #[macro_use]，可将模块里面的宏导出
```
mod a {
    // X!(); // undefined
}

#[macro_use]
mod b {
    macro_rules! X { () => {}; }
    X!(); // defined
}

mod c {
    X!(); // defined
}
```

参考：
https://danielkeep.github.io/tlborm/book/mbe-min-scoping.html
