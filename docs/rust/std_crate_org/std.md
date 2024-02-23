C:\Users\Admin\.rustup\toolchains\stable-x86_64-pc-windows-msvc\lib\rustlib\src\rust\library\std\src\lib.rs

```
extern crate alloc as alloc_crate;
pub use alloc_crate::slice;
pub use alloc_crate::string;
pub use alloc_crate::str;
pub use alloc_crate::vec;
pub use alloc_crate::fmt;

pub use alloc_crate::borrow;
pub use alloc_crate::rc;

pub use core::any;
pub use core::array;
pub use core::char;

pub use core::i8;
pub use core::i16;
pub use core::i32;
pub use core::i64;
pub use core::i128;
pub use core::isize;

pub use core::u8;
pub use core::u16;
pub use core::u32;
pub use core::u64;
pub use core::u128;
pub use core::usize;

pub use core::marker;

pub use core::cmp;
pub use core::hash;
pub use core::iter;
pub use core::ops;
pub use core::option;
pub use core::result;

pub use core::hint;
pub use core::primitive;

pub use core::cell;
pub use core::clone;
pub use core::convert;
pub use core::default;
pub use core::mem;
pub use core::pin;
pub use core::ptr;

pub use core::future;

pub mod collections;
pub mod f32;
pub mod f64;
pub mod ascii;
pub mod num;
pub mod error;
pub mod panic;
pub mod ffi;
pub mod prelude;
pub mod alloc;
pub mod env;
pub mod process;
pub mod sync;
pub mod thread;
pub mod fs;
pub mod path;
pub mod io;
pub mod net;
pub mod os;
pub mod time;
pub mod task {
    pub use core::task::*;
    pub use alloc::task::*;
}
```

C:\Users\Admin\.rustup\toolchains\stable-x86_64-pc-windows-msvc\lib\rustlib\src\rust\library\std\Cargo.toml

```
[dependencies]
alloc = { path = "../alloc", public = true }
core = { path = "../core", public = true }
```
