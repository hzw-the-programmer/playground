fn main() {
    prost_build::compile_protos(
        &[
            "src/items.proto",
            "src/repeated.proto",
            "src/oneof.proto",
            "src/map.proto",
        ],
        &["src/"],
    )
    .unwrap();
}
