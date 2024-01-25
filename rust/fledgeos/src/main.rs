#![no_std]
#![no_main]
#![feature(core_intrinsics)]
#![feature(lang_items)]

use core::intrinsics;
use core::panic::PanicInfo;
use x86_64::instructions::hlt;

#[panic_handler]
#[no_mangle]
pub fn panic(_info: &PanicInfo) -> ! {
    intrinsics::abort();
}

#[lang = "eh_personality"]
#[no_mangle]
pub extern "C" fn eh_personality() {}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    let text = b"Rust in Action";

    let mut cursor = Cursor {
        position: 0,
        foreground: Color::BrightCyan,
        background: Color::Black,
    };
    cursor.print(text);

    loop {
        hlt();
    }
}

struct Cursor {
    position: isize,
    foreground: Color,
    background: Color,
}

impl Cursor {
    fn print(&mut self, text: &[u8]) {
        let framebuffer = 0xb8000 as *mut u8;
        let color = self.color();

        for &character in text {
            unsafe {
                framebuffer.offset(self.position).write_volatile(character);
                framebuffer.offset(self.position + 1).write_volatile(color);
            }
            self.position += 2;
        }
    }

    fn color(&self) -> u8 {
        let fg = self.foreground as u8;
        let bg = (self.background as u8) << 4;
        fg | bg
    }
}

#[allow(unused)]
#[derive(Copy, Clone)]
#[repr(u8)]
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
