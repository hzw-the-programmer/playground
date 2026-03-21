cd workspace
cargo fmt
cargo run --bin atomic_counter
cargo test --bin atomic_counter
cargo test --lib atomic_counter
