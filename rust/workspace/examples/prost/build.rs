fn main() {
    prost_build::compile_protos(
        &[
            "protos/items.proto",
            "protos/map.proto",
            "protos/oneof.proto",
            "protos/repeated.proto",
            "protos/submsg.proto",
        ],
        &["protos/"],
    )
    .unwrap();
}
