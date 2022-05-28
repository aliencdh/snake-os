#![no_std]
#![no_main]

#[macro_use]
extern crate lazy_static;

use core::panic::PanicInfo;

mod vga_buffer;

static MSG: &[u8] = b"Hello World!";

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello world{}", "!");

    loop {}
}

