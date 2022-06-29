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

