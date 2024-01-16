use std::ops::Not;

// $(RUSTUP_HOME)\toolchains\stable-x86_64-pc-windows-msvc\lib\rustlib\src\rust\library\core\src\ops\bit.rs

#[derive(PartialEq, Debug)]
enum Answer {
    Yes,
    No,
}

impl Not for Answer {
    type Output = Self;

    fn not(self) -> Self::Output {
        match self {
            Answer::Yes => Answer::No,
            Answer::No => Answer::Yes,
        }
    }
}

fn main() {
    assert_eq!(!Answer::Yes, Answer::No);
    // assert_eq!(!123, 0);
}
