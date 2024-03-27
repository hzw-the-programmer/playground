cargo new clap_01
cd clap_01/
cargo add clap --features derive
mkdir src/bin
cargo run --bin t1
cargo run --bin t1 -- --config hello hzw -dd test
