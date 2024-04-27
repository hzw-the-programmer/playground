fn main() {
    for i in Range::new(0, 10) {
        println!("{i}");
    }
}

struct Range<T> {
    start: T,
    end: T,
}

impl<T> Range<T> {
    fn new(start: T, end: T) -> Self {
        Range { start, end }
    }
}

use std::ops::AddAssign;

impl<T: PartialOrd + AddAssign<i32> + Copy> Iterator for Range<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.start >= self.end {
            None
        } else {
            let result = Some(self.start);
            self.start += 1;
            result
        }
    }
}
