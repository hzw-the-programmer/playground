RawVec负责alloc，grow，shrink，dealloc，不负责元素的drop。 
D:\rustup_home\toolchains\stable-x86_64-pc-windows-msvc\lib\rustlib\src\rust\library\alloc\src\raw_vec.rs
```
fn allocate_in(capacity: usize, init: AllocInit, alloc: A) -> Self
fn grow_amortized(&mut self, len: usize, additional: usize) -> Result<(), TryReserveError>
fn grow_exact(&mut self, len: usize, additional: usize) -> Result<(), TryReserveError>
fn finish_grow<A>(
    new_layout: Result<Layout, LayoutError>,
    current_memory: Option<(NonNull<u8>, Layout)>,
    alloc: &mut A,
) -> Result<NonNull<[u8]>, TryReserveError>
fn shrink(&mut self, cap: usize) -> Result<(), TryReserveError>
fn drop(&mut self)
```