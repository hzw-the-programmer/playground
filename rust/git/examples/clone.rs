use git2::Repository;

fn main() {
    let url = "https://github.com/alexcrichton/git2-rs";
    let repo = match Repository::clone(url, "/path/to/a/repo") {
        Ok(repo) => repo,
        Err(e) => panic!("failed to clone: {}", e),
    };
}
