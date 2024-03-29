use clap::Parser;
use git2::{Error, Repository};

fn main() -> Result<(), Error> {
    let cli = Cli::parse();
    println!("path: {}", cli.path);

    let repo = Repository::open(cli.path)?;
    let branches = repo.branches(None)?;
    for result in branches {
        let (branch, branch_type) = result?;
        println!("{}: {:?}", branch.name()?.unwrap_or("unknown"), branch_type);
    }

    Ok(())
}

#[derive(Parser)]
struct Cli {
    path: String,
}
