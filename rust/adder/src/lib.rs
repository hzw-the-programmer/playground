// cargo test -- --show-output
// cargo test one_hundred
// cargo test add
// cargo test -- --ignored

pub mod garden;

fn call_garden() {
    garden::do_something();
}

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

fn prints_and_returns_10(a: i32) -> i32 {
    println!("I got the value {}", a);
    10
}

fn add_3(a: i32) -> i32 {
    3 + a
}

pub fn adds_two(a: i32) -> i32 {
    2 + a
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn exporation() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn another() {
        panic!("Make this test fail");
    }

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };
        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn smaller_cannot_hold_larger() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };
        assert!(!smaller.can_hold(&larger));
    }

    #[test]
    fn it_adds_two() {
        assert_eq!(4, add_two(2))
    }

    #[test]
    fn greeting_contains_name() {
        let result = greet("Carol");
        assert!(
            result.contains("Carol"),
            "Greeting did not contain name, value was '{}'",
            result
        );
    }

    #[test]
    #[should_panic(expected = "less than or equal to 100")]
    fn greeter_than_100() {
        Guess::new(200);
    }

    #[test]
    fn it_works_1() -> Result<(), String> {
        if 2 + 2 != 4 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four"))
        }
    }

    #[test]
    fn this_test_will_passh() {
        assert_eq!(10, prints_and_returns_10(4));
    }

    #[test]
    fn this_test_will_fail() {
        assert_eq!(5, prints_and_returns_10(8));
    }

    #[test]
    fn add_3_and_4() {
        assert_eq!(7, add_3(4));
    }

    #[test]
    fn add_3_and_5() {
        assert_eq!(8, add_3(5));
    }

    #[test]
    fn one_hundred() {
        assert_eq!(103, add_3(100));
    }

    #[test]
    #[ignore]
    fn expensive_test() {}
}

struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width < other.width && self.height > other.height
    }
}

fn add_two(a: i32) -> i32 {
    3 + a
}

fn greet(name: &str) -> String {
    String::from("Hello")
}

struct Guess {
    value: i32,
}

impl Guess {
    fn new(value: i32) -> Guess {
        if value < 1 {
            panic!(
                "Guess value must be greeter than or equal to 1, got {}.",
                value
            );
        } else if value > 100 {
            panic!(
                "Guess value must be less than or equal to 100, got {}.",
                value
            );
        }
        Guess { value }
    }
}
