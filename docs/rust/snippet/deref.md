D:\rustup_home\toolchains\stable-x86_64-pc-windows-msvc\lib\rustlib\src\rust\library\core\src\task\wake.rs
```Rust
pub struct Context<'a> {
    waker: &'a Waker,
}

impl<'a> Context<'a> {
    pub const fn waker(&self) -> &'a Waker {
        &self.waker
    }
}
```

D:\rustup_home\toolchains\stable-x86_64-pc-windows-msvc\lib\rustlib\src\rust\library\core\src\ops\deref.rs
```Rust
impl<T: ?Sized> Deref for &T {
    type Target = T;

    #[rustc_diagnostic_item = "noop_method_deref"]
    fn deref(&self) -> &T {
        *self
    }
}
```

# another
D:\github\tokio-rs\tokio\tokio\src\runtime\task\waker.rs
```Rust
pub(super) struct WakerRef<'a, S: 'static> {
    waker: ManuallyDrop<Waker>,
}

impl<S> ops::Deref for WakerRef<'_, S> {
    type Target = Waker;

    fn deref(&self) -> &Waker {
        &self.waker
    }
}
```

D:\rustup_home\toolchains\stable-x86_64-pc-windows-msvc\lib\rustlib\src\rust\library\core\src\mem\manually_drop.rs
```Rust
pub struct ManuallyDrop<T: ?Sized> {
    value: T,
}

impl<T: ?Sized> Deref for ManuallyDrop<T> {
    type Target = T;
    #[inline(always)]
    fn deref(&self) -> &T {
        &self.value
    }
}
```
