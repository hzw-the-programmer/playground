use clap::{command, Arg, ArgAction};

fn main() {
    let matches = command!() // requires `cargo` feature
        .arg(
            Arg::new("name")
                .short('n')
                .long("name")
                .action(ArgAction::Append),
        )
        .get_matches();

    println!("name: {:?}", matches.get_one::<String>("name"));

    let args = matches
        .get_many::<String>("name")
        .unwrap_or_default()
        .map(|v| v.as_str())
        .collect::<Vec<_>>();

    println!("names: {:?}", &args);
}
