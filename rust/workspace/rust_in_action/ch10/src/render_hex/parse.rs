use super::HEIGHT;
use Operation::*;

#[derive(Debug, PartialEq)]
enum Operation {
    Forward(isize),
    TurnLeft,
    TurnRight,
    Home,
    Noop(u8),
}

fn parse(s: &str) -> Vec<Operation> {
    let mut steps = Vec::with_capacity(s.len());
    for b in s.bytes() {
        let step = match b {
            b'0' => Home,
            b'1'..=b'9' => {
                let distance = (b - b'0') as isize;
                Forward(distance * (HEIGHT / 10))
            }
            b'a' | b'b' | b'c' => TurnLeft,
            b'd' | b'e' | b'f' => TurnRight,
            _ => Noop(b),
        };
        steps.push(step);
    }
    steps
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let w = HEIGHT / 10;
        let res = parse("0123456789abcdefg");
        let wanted = [
            Home,
            Forward(1 * w),
            Forward(2 * w),
            Forward(3 * w),
            Forward(4 * w),
            Forward(5 * w),
            Forward(6 * w),
            Forward(7 * w),
            Forward(8 * w),
            Forward(9 * w),
            TurnLeft,
            TurnLeft,
            TurnLeft,
            TurnRight,
            TurnRight,
            TurnRight,
            Noop(b'g'),
        ];
        assert_eq!(&wanted[..], res);
    }
}
