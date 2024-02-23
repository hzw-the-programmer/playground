# vec

C:\Users\Admin\.rustup\toolchains\stable-x86_64-pc-windows-msvc\lib\rustlib\src\rust\library\std\src\prelude\v1.rs
```
pub use crate::vec::Vec;
```

C:\Users\Admin\.rustup\toolchains\stable-x86_64-pc-windows-msvc\lib\rustlib\src\rust\library\std\src\lib.rs
```
extern crate alloc as alloc_crate;
pub use alloc_crate::vec;
```

C:\Users\Admin\.rustup\toolchains\stable-x86_64-pc-windows-msvc\lib\rustlib\src\rust\library\std\Cargo.toml
```
[dependencies]
alloc = { path = "../alloc", public = true }
```

# other collections

C:\Users\Admin\.rustup\toolchains\stable-x86_64-pc-windows-msvc\lib\rustlib\src\rust\library\std\src\lib.rs
```
pub mod collections;
```

C:\Users\Admin\.rustup\toolchains\stable-x86_64-pc-windows-msvc\lib\rustlib\src\rust\library\std\src\collections\mod.rs
```
pub use alloc_crate::collections::{binary_heap, btree_map, btree_set};
pub use alloc_crate::collections::{linked_list, vec_deque};
pub use alloc_crate::collections::{BTreeMap, BTreeSet, BinaryHeap};
pub use alloc_crate::collections::{LinkedList, VecDeque};
```
