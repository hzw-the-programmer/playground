use clap::{arg, Arg, ArgAction, Command};

fn main() {
    let matches = Command::new("hzw")
        .arg(
            Arg::new("out")
                .short('o')
                .long("output")
                // .required(true)
                .action(ArgAction::Set)
                .default_value("-"),
        )
        .arg(
            Arg::new("config")
                .short('c')
                .long("config")
                .value_name("FILE")
                .help("Provides a config file")
                .default_value("config.toml"),
        )
        .arg(arg!(-i --input <FILE> "Provides an input file"))
        .subcommand(Command::new("hashmap"))
        .get_matches();

    if let Some(("hashmap", cmd)) = matches.subcommand() {
        hashmap();
    }
}

fn hashmap() {
    println!("hashmap called");
}
