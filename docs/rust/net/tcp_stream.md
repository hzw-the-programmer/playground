D:\rustup_home\toolchains\stable-x86_64-pc-windows-msvc\lib\rustlib\src\rust\library\std\src\os\fd\net.rs
```Rust
macro_rules! impl_as_raw_fd {
    ($($t:ident)*) => {$(
        #[stable(feature = "rust1", since = "1.0.0")]
        impl AsRawFd for net::$t {
            #[inline]
            fn as_raw_fd(&self) -> RawFd {
                self.as_inner().socket().as_raw_fd()
            }
        }
    )*};
}
impl_as_raw_fd! { TcpStream TcpListener UdpSocket }
```

D:\rustup_home\toolchains\stable-x86_64-pc-windows-msvc\lib\rustlib\src\rust\library\std\src\os\windows\io\raw.rs
```Rust
impl AsRawSocket for net::TcpStream {
    #[inline]
    fn as_raw_socket(&self) -> RawSocket {
        self.as_inner().socket().as_raw_socket()
    }
}
```

D:\rustup_home\toolchains\stable-x86_64-pc-windows-msvc\lib\rustlib\src\rust\library\std\src\net\tcp.rs
```Rust
impl AsInner<net_imp::TcpStream> for TcpStream {
    #[inline]
    fn as_inner(&self) -> &net_imp::TcpStream {
        &self.0
    }
}
```

D:\rustup_home\toolchains\stable-x86_64-pc-windows-msvc\lib\rustlib\src\rust\library\std\src\sys_common\net.rs
```Rust
impl TcpStream {
    pub fn socket(&self) -> &Socket {
        &self.inner
    }
}
```

D:\rustup_home\toolchains\stable-x86_64-pc-windows-msvc\lib\rustlib\src\rust\library\std\src\sys\windows\net.rs
```Rust
pub struct Socket(OwnedSocket);
impl AsRawSocket for Socket {
    fn as_raw_socket(&self) -> RawSocket {
        self.0.as_raw_socket()
    }
}
```

D:\rustup_home\toolchains\stable-x86_64-pc-windows-msvc\lib\rustlib\src\rust\library\std\src\os\windows\io\socket.rs
```Rust
pub struct OwnedSocket {
    socket: RawSocket,
}
impl AsRawSocket for OwnedSocket {
    #[inline]
    fn as_raw_socket(&self) -> RawSocket {
        self.socket
    }
}
```

D:\rustup_home\toolchains\stable-x86_64-pc-windows-msvc\lib\rustlib\src\rust\library\std\src\sys\unix\net.rs
```Rust
pub struct Socket(FileDesc);
impl AsRawFd for Socket {
    #[inline]
    fn as_raw_fd(&self) -> RawFd {
        self.0.as_raw_fd()
    }
}
```

D:\rustup_home\toolchains\stable-x86_64-pc-windows-msvc\lib\rustlib\src\rust\library\std\src\sys\unix\fd.rs
```Rust
pub struct FileDesc(OwnedFd);
impl AsRawFd for FileDesc {
    #[inline]
    fn as_raw_fd(&self) -> RawFd {
        self.0.as_raw_fd()
    }
}
```

D:\rustup_home\toolchains\stable-x86_64-pc-windows-msvc\lib\rustlib\src\rust\library\std\src\os\unix\io\mod.rs
```Rust
pub use crate::os::fd::*;
```

D:\rustup_home\toolchains\stable-x86_64-pc-windows-msvc\lib\rustlib\src\rust\library\std\src\os\fd\owned.rs
```Rust
pub struct OwnedFd {
    fd: RawFd,
}
impl AsRawFd for OwnedFd {
    #[inline]
    fn as_raw_fd(&self) -> RawFd {
        self.fd
    }
}
```
