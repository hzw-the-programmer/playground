use clap::{Args, Parser, Subcommand};

#[derive(Parser)]
#[command(version, about, long_about = None)]
// #[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Adds files to myapp
    Add(AddArgs),
}

#[derive(Args)]
struct AddArgs {
    name: Option<String>,
}

fn main() {
    let cli = Cli::parse();

    // You can check for the existence of subcommands, and if found use their
    // matches just as you would the top level cmd
    match &cli.command {
        Some(Commands::Add(name)) => {
            // let i: i32 = name;
            println!("'myapp add' was used, name is: {:?}", name.name)
        }
        None => {
            println!("'myapp add' was used")
        }
    }
}
