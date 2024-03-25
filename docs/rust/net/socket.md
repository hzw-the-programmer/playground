D:\rustup_home\toolchains\stable-x86_64-pc-windows-msvc\lib\rustlib\src\rust\library\std\src\os\windows\io\socket.rs
```Rust
impl FromRawSocket for OwnedSocket {
    #[inline]
    unsafe fn from_raw_socket(socket: RawSocket) -> Self {
        debug_assert_ne!(socket, sys::c::INVALID_SOCKET as RawSocket);
        Self { socket }
    }
}
impl Drop for OwnedSocket {
    #[inline]
    fn drop(&mut self) {
        unsafe {
            let _ = sys::c::closesocket(self.socket as sys::c::SOCKET);
        }
    }
}
```

D:\rustup_home\toolchains\stable-x86_64-pc-windows-msvc\lib\rustlib\src\rust\library\std\src\os\windows\io\raw.rs
```Rust
pub type RawSocket = raw::SOCKET;
```

D:\rustup_home\toolchains\stable-x86_64-pc-windows-msvc\lib\rustlib\src\rust\library\std\src\os\windows\raw.rs
```Rust
pub type SOCKET = u64;
```

D:\rustup_home\toolchains\stable-x86_64-pc-windows-msvc\lib\rustlib\src\rust\library\std\src\sys\windows\net.rs
```Rust
impl Socket {
    pub unsafe fn from_raw(raw: c::SOCKET) -> Self {
        debug_assert_eq!(mem::size_of::<c::SOCKET>(), mem::size_of::<RawSocket>());
        debug_assert_eq!(mem::align_of::<c::SOCKET>(), mem::align_of::<RawSocket>());
        Self::from_raw_socket(raw as RawSocket)
    }
}
impl FromRawSocket for Socket {
    unsafe fn from_raw_socket(raw_socket: RawSocket) -> Self {
        Self(FromRawSocket::from_raw_socket(raw_socket))
    }
}
```

D:\rustup_home\toolchains\stable-x86_64-pc-windows-msvc\lib\rustlib\src\rust\library\std\src\sys\windows\c\windows_sys.rs
```Rust
pub type SOCKET = usize;
```
