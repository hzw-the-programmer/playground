编译报错：
```
error: `cargo` feature flag is required
 --> examples\ch0_0.rs:6:19
  |
6 |     let matches = command!() // requires `cargo` feature
  |                   ^^^^^^^^^^
  |
```

use 表明 command 在 clap 包：
```
use clap::{arg, command, value_parser, ArgAction, Command};
```

clap 源码路径：
$CARGO_HOME\registry\src\index.crates.io-6f17d22bba15001f\clap-4.4.18\src\lib.rs
$CARGO_HOME 默认为 C:\Users\Admin\.cargo 


clap-4.4.18\src\lib.rs
```
pub use clap_builder::*;
#[cfg(feature = "derive")]
#[doc(hidden)]
pub use clap_derive::{self, *};

#[cfg(feature = "unstable-doc")]
pub mod _cookbook;
#[cfg(feature = "unstable-doc")]
pub mod _derive;
#[cfg(feature = "unstable-doc")]
pub mod _faq;
#[cfg(feature = "unstable-doc")]
pub mod _features;
#[cfg(feature = "unstable-doc")]
pub mod _tutorial;
```

并没有声明 clap_builder、clap_derive 模块，为啥可以 use 呢？
因为她两是依赖，可以在 Cargo.toml 看到

clap-4.4.18\Cargo.toml.orig
```
[dependencies]
clap_builder = { path = "./clap_builder", version = "=4.4.18", default-features = false }
clap_derive = { path = "./clap_derive", version = "=4.4.7", optional = true }
```

clap_builder-4.4.18\src\lib.rs
```
#[macro_use]
#[allow(missing_docs)]
mod macros;
```

clap_builder-4.4.18\src\macros.rs
```
#[cfg(feature = "cargo")]
#[macro_export]
macro_rules! command {
}
```
要打开 cargo feature，command 宏才定义

在项目的 Cargo.toml 中加入
```
[dependencies]
clap = { version = "4.4.18", features = ["cargo"] }
```
