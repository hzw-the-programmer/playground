fn main() {
    capnpc::CompilerCommand::new()
        .src_prefix("schemas/")
        .file("schemas/addressbook.capnp")
        .run()
        .expect("compiling schema");
}
