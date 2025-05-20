cargo run -p macrostest --example into_string_hash_map

rustup toolchain install nightly
cargo install cargo-expand
cargo expand
cargo expand --example constant_string
