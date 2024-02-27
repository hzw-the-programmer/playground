C:\Users\Admin\.rustup\toolchains\stable-x86_64-pc-windows-msvc\lib\rustlib\src\rust\library\std\src\sys_common\net.rs
```
pub struct TcpListener {
    inner: Socket,
}
pub struct TcpStream {
    inner: Socket,
}
```

C:\Users\Admin\.rustup\toolchains\stable-x86_64-pc-windows-msvc\lib\rustlib\src\rust\library\std\src\sys\unix\net.rs
```
pub struct Socket(FileDesc);
```

C:\Users\Admin\.rustup\toolchains\stable-x86_64-pc-windows-msvc\lib\rustlib\src\rust\library\std\src\sys\unix\fd.rs
```
pub struct FileDesc(OwnedFd);
```

C:\Users\Admin\.rustup\toolchains\stable-x86_64-pc-windows-msvc\lib\rustlib\src\rust\library\std\src\os\fd\owned.rs
```
pub struct OwnedFd {
    fd: RawFd,
}
```

C:\Users\Admin\.rustup\toolchains\stable-x86_64-pc-windows-msvc\lib\rustlib\src\rust\library\std\src\os\fd\raw.rs
```
pub type RawFd = raw::c_int;
```
