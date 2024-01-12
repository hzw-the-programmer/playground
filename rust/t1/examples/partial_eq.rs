#[derive(PartialEq)]
struct Book {
    isbn: i32,
}

fn test1() {
    let b1 = Book { isbn: 1 };
    let b2 = Book { isbn: 2 };
    let b1n = Book { isbn: 1 };

    println!("{}", b1 == b2);
    println!("{}", b1 == b1n);
}

// #[derive(PartialEq)]
enum BookFormat {
    Paperback,
    Hardback,
    Ebook,
}

// #[derive(PartialEq)]
struct Book1 {
    isbn: i32,
    format: BookFormat,
}

impl PartialEq for Book1 {
    fn eq(&self, other: &Self) -> bool {
        self.isbn == other.isbn
    }
}

fn test2() {
    let b1_p = Book1 {
        isbn: 1,
        format: BookFormat::Paperback,
    };
    let b1_h = Book1 {
        isbn: 1,
        format: BookFormat::Hardback,
    };
    println!("{}", b1_p == b1_h);
}

fn main() {
    test1();
    test2();
}
