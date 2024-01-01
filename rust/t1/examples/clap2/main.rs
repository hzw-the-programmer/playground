use clap::Command;
mod hashmap;

fn main() {
    let matches = Command::new("root")
        .subcommand(Command::new("hashmap"))
        .get_matches();

    match matches.subcommand() {
        Some(("hashmap", _)) => hashmap::hashmap(),
        _ => unreachable!("parser should ensure only valid subcommand names are used"),
    }
}
