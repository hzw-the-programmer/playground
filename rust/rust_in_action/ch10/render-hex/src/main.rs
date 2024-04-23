use self::Operation::*;

fn main() {
    println!("Hello, world!");
}

enum Operation {
    Forward(isize),
    TurnLeft,
    TurnRight,
    Home,
    Invalid(char),
}

fn parse(hex: &str) -> Vec<Operation> {
    let mut ops = vec![];
    for b in hex.chars() {
        ops.push(match b {
            '0' => Home,
            '1'..='9' => Forward(b as isize - '0' as isize),
            'a' | 'b' | 'c' => TurnLeft,
            'd' | 'e' | 'f' => TurnRight,
            _ => Invalid(b),
        })
    }
    ops
}
