extern crate capnpc;

fn main() {
    capnpc::CompilerCommand::new()
        .output_path("src/")
        .src_prefix("schemas/")
        .file("schemas/person_schema.capnp")
        .run()
        .unwrap();
}
