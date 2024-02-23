C:\Users\Admin\.rustup\toolchains\stable-x86_64-pc-windows-msvc\lib\rustlib\src\rust\library\std\src\prelude\v1.rs
```
pub use crate::string::{String, ToString};
```

C:\Users\Admin\.rustup\toolchains\stable-x86_64-pc-windows-msvc\lib\rustlib\src\rust\library\std\src\lib.rs
```
extern crate alloc as alloc_crate;
pub use alloc_crate::string;
```

C:\Users\Admin\.rustup\toolchains\stable-x86_64-pc-windows-msvc\lib\rustlib\src\rust\library\std\Cargo.toml
```
[dependencies]
alloc = { path = "../alloc", public = true }
```
