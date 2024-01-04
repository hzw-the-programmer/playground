brew install netcat
nc tcpbin.com 4242

brew install socat
socat -v tcp-l:1234,fork exec:'/bin/cat'
nc 127.0.0.1 1234

cargo new echo-client-std
cd echo-client-std
ls

rustup default
rustup default nightly

C:\Users\Admin\.rustup\toolchains\stable-x86_64-pc-windows-msvc\lib\rustlib\src\rust\library\core\src\marker.rs