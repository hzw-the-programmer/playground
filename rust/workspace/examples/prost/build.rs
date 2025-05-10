fn main() {
    prost_build::compile_protos(&["src/items.proto", "src/test4.proto"], &["src/", "src/"])
        .unwrap();
}
