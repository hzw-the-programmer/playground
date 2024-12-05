use std::io::{BufRead, Cursor};

pub fn test() {
    test1();
}

fn test1() {
    let c = Cursor::new("lorem\nipsum\r\ndolor");
    let mut lines = c.lines().map(|l| l.unwrap());
    assert_eq!(Some(String::from("lorem")), lines.next());
}
