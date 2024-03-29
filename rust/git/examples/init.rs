use git2::Repository;

fn main() {
    let repo = match Repository::init("/path/to/a/repo") {
        Ok(repo) => repo,
        Err(e) => panic!("failed to init: {}", e),
    };
}
