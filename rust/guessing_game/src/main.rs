use std::io;
use std::cmp::Ordering;
use rand::Rng;

// vi ~/.cargo/config.toml
// [source.crates-io]
// replace-with = "sjtu"
//
// [source.sjtu]
// registry = "https://mirrors.sjtug.sjtu.edu.cn/git/crates.io-index"
//

fn main() {
    println!("Guess the number!");

    let secret_num = rand::thread_rng().gen_range(1..101);

    println!("The secret number is: {}", secret_num);

    loop {
        println!("Please input your number.");
    
        let mut guess = String::new();
        
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
    
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
    
        match guess.cmp(&secret_num) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
        
        println!("You guessed: {}", guess);
    }
}
