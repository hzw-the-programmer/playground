use std::env;
use std::fs;
use std::process;

fn main() {
    let mut args = env::args();
    let filename = match (args.next(), args.next(), args.next()) {
        (_, Some(filename), None) => filename,
        _ => {
            println!("Usages: dump_syntax path/to/file");
            process::exit(1);
        }
    };
    let src = fs::read_to_string(filename).expect("Unable to read file");
    let syntax = syn::parse_file(&src).expect("Unable to parse file");
    println!("{syntax:#?}");
}
