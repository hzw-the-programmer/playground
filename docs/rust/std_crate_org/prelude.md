编译器自动导入

C:\Users\Admin\.rustup\toolchains\stable-x86_64-pc-windows-msvc\lib\rustlib\src\rust\library\std\src\lib.rs
```
pub mod prelude;
```

C:\Users\Admin\.rustup\toolchains\stable-x86_64-pc-windows-msvc\lib\rustlib\src\rust\library\std\src\prelude\mod.rs
```
pub mod v1;
```

C:\Users\Admin\.rustup\toolchains\stable-x86_64-pc-windows-msvc\lib\rustlib\src\rust\library\std\src\prelude\v1.rs
```
pub use crate::marker::{Send, Sized, Sync, Unpin};
pub use core::prelude::v1::{
    assert, cfg, column, compile_error, concat, concat_idents, env, file, format_args,
    format_args_nl, include, include_bytes, include_str, line, log_syntax, module_path, option_env,
    stringify, trace_macros, Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd,
};
```

编译器不自动导入，需要
```
use std::io::prelude::*;
```

C:\Users\Admin\.rustup\toolchains\stable-x86_64-pc-windows-msvc\lib\rustlib\src\rust\library\std\src\io\mod.rs
```
pub mod prelude;
```

C:\Users\Admin\.rustup\toolchains\stable-x86_64-pc-windows-msvc\lib\rustlib\src\rust\library\std\src\io\prelude.rs
```
pub use super::{BufRead, Read, Seek, Write};
```
