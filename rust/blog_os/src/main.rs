#![no_std]
#![no_main]

// https://os.phil-opp.com/freestanding-rust-binary/

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

static HELLO: &[u8] = b"hello";

#[no_mangle]
pub extern "C" fn _start() -> ! {
    let vga_buf = 0xb8000 as *mut u8;

    for (i, &c) in HELLO.iter().enumerate() {
        unsafe {
            vga_buf.offset(i as isize * 2).write_volatile(c);
            vga_buf.offset(i as isize * 2 + 1).write_volatile(0x0b);
        }
    }

    loop {}
}
