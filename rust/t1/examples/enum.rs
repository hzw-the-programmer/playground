#[allow(unused)]
enum Color {
    Black = 0x00,
    Blue = 0x01,
    Green = 0x02,
    Cyan = 0x03,
    Red = 0x04,
    Magenta = 0x05,
    Brown = 0x06,
    Gray = 0x07,
    DarkGray = 0x08,
    BrightBlue = 0x09,
    BrightGreen = 0x0a,
    BrightCyan = 0x0b,
    BrightRed = 0x0c,
    BrightMagenta = 0x0d,
    Yellow = 0x0e,
    White = 0x0f,
}

#[allow(unused)]
#[repr(u8)]
enum Color2 {
    Black = 0x00,
    Blue = 0x01,
    Green = 0x02,
    Cyan = 0x03,
    Red = 0x04,
    Magenta = 0x05,
    Brown = 0x06,
    Gray = 0x07,
    DarkGray = 0x08,
    BrightBlue = 0x09,
    BrightGreen = 0x0a,
    BrightCyan = 0x0b,
    BrightRed = 0x0c,
    BrightMagenta = 0x0d,
    Yellow = 0x0e,
    White = 0x0f,
}

fn main() {
    println!("enum Color : {}", std::mem::size_of::<Color>());
    println!("enum Color2: {}", std::mem::size_of::<Color2>());
}
