use std::process;
use std::thread::sleep;
use std::time;

// kill -SIGSTOP 23221
// kill -SIGCONT 23221

fn main() {
    let delay = time::Duration::from_secs(1);
    let pid = process::id();
    println!("{}", pid);
    for i in 1..=60 {
        sleep(delay);
        println!(". {}", i);
    }
}
