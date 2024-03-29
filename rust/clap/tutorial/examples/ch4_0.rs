use clap::Parser;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Network port to use
    port: u16,
}

fn main() {
    let cli = Cli::parse();

    println!("PORT = {}", cli.port);
}

// cargo test --example ch4_0
#[test]
fn verify_cli() {
    use clap::CommandFactory;
    Cli::command().debug_assert()
}
