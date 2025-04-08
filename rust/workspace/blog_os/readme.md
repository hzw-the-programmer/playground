cargo fmt -p blog_os
rustup target add thumbv7em-none-eabihf
cargo build --target thumbv7em-none-eabihf -p blog_os
