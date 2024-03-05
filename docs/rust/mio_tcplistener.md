C:\Users\Admin\.rustup\toolchains\stable-x86_64-pc-windows-msvc\lib\rustlib\src\rust\library\std\src\os\fd\net.rs
```
macro_rules! impl_from_raw_fd {
    ($($t:ident)*) => {$(
        #[stable(feature = "from_raw_os", since = "1.1.0")]
        impl FromRawFd for net::$t {
            #[inline]
            unsafe fn from_raw_fd(fd: RawFd) -> net::$t {
                unsafe {
                    let socket = sys::net::Socket::from_inner(FromInner::from_inner(OwnedFd::from_raw_fd(fd)));
                    net::$t::from_inner(sys_common::net::$t::from_inner(socket))
                }
            }
        }
    )*};
}
impl_from_raw_fd! { TcpStream TcpListener UdpSocket }
```

C:\Users\Admin\.rustup\toolchains\stable-x86_64-pc-windows-msvc\lib\rustlib\src\rust\library\std\src\net\tcp.rs
```
impl FromInner<net_imp::TcpStream> for TcpStream {
    fn from_inner(inner: net_imp::TcpStream) -> TcpStream {
        TcpStream(inner)
    }
}
```

C:\Users\Admin\.rustup\toolchains\stable-x86_64-pc-windows-msvc\lib\rustlib\src\rust\library\std\src\sys_common\net.rs
```
impl FromInner<Socket> for TcpListener {
    fn from_inner(socket: Socket) -> TcpListener {
        TcpListener { inner: socket }
    }
}
```

C:\Users\Admin\.rustup\toolchains\stable-x86_64-pc-windows-msvc\lib\rustlib\src\rust\library\std\src\sys\unix\net.rs
```
impl FromInner<FileDesc> for Socket {
    fn from_inner(file_desc: FileDesc) -> Self {
        Self(file_desc)
    }
}
```

C:\Users\Admin\.rustup\toolchains\stable-x86_64-pc-windows-msvc\lib\rustlib\src\rust\library\std\src\sys\unix\fd.rs
```
impl FromInner<OwnedFd> for FileDesc {
    fn from_inner(owned_fd: OwnedFd) -> Self {
        Self(owned_fd)
    }
}
```

C:\Users\Admin\.rustup\toolchains\stable-x86_64-pc-windows-msvc\lib\rustlib\src\rust\library\std\src\os\fd\owned.rs
```
impl FromRawFd for OwnedFd {
    unsafe fn from_raw_fd(fd: RawFd) -> Self {
        unsafe { Self { fd } }
    }
}
```
