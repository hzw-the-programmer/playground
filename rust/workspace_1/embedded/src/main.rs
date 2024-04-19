#![no_std]
#![no_main]
#![feature(lang_items)]
#![feature(rustc_private)]
#![feature(default_alloc_error_handler)]
#[global_allocator]
static ALLOCATOR: ::libc_alloc::LibcAlloc = ::libc_alloc::LibcAlloc;

#[macro_use]
extern crate alloc;
extern crate libc;

use alloc::{ffi::CString, string::String, string::ToString};
use core::{ffi::c_char, ffi::CStr};
use core_library::run;
use libc::c_int;

#[cfg_attr(
    all(windows, target_env = "msvc"),
    link(name = "legacy_stdio_definitions", kind = "dylib")
)]
extern "C" {
    pub fn printf(format: *const i8, ...) -> i32;
}

pub extern "C" fn log(message: *const c_char) {
    let format = format!("%s\n\0");
    unsafe {
        // libc::printf(format.as_ptr() as *const _, message);
        printf(format.as_ptr() as *const _, message);
    }
}

fn local_log(message: String) {
    let output = CString::new(message).expect("CString::new failed");
    log(output.as_ptr());
}

#[no_mangle]
pub extern "C" fn main(argc: c_int, argv: *const *const c_char) -> c_int {
    let message = String::from("Hello, embedded!");
    local_log(message);
    run(log);
    for i in 1..argc {
        let cstr = unsafe { CStr::from_ptr(*argv.offset(i as isize)) };
        let safe_string = String::from_utf8_lossy(cstr.to_bytes()).to_string();
        let output = format!("{}: {}", i, safe_string);
        local_log(output);
    }
    return 0;
}

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {}
}

#[lang = "eh_personality"]
extern "C" fn eh_personality() {}
