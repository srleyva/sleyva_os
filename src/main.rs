#![no_std]
#![no_main]

use core::panic::PanicInfo;

extern crate rlibc;
mod vga_buffer;

#[panic_handler]
fn panic_handler(_info: &PanicInfo) -> ! {
    loop{}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    vga_buffer::print_something();

    loop {}
}