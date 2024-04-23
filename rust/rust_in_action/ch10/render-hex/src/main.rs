use self::Operation::*;

fn main() {
    println!("Hello, world!");
}

const WIDTH: isize = 400;
const HEIGHT: isize = WIDTH;

enum Operation {
    Forward(isize),
    TurnLeft,
    TurnRight,
    Home,
    Noop(u8),
}

fn parse(input: &str) -> Vec<Operation> {
    let mut steps = Vec::new();

    for byte in input.bytes() {
        let step = match byte {
            b'0' => Home,
            b'1'..=b'9' => {
                let distance = (byte - 0x30) as isize;
                Forward(distance * HEIGHT / 2)
            }
            b'a' | b'b' | b'c' => TurnLeft,
            b'd' | b'e' | b'f' => TurnRight,
            _ => Noop(byte),
        };

        steps.push(step);
    }

    steps
}
