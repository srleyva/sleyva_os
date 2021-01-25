#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(sleyva_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use bootloader::{BootInfo, entry_point};
use core::panic::PanicInfo;
use sleyva_os::println;

entry_point!(kernel_main);

fn kernel_main(boot_info: &'static BootInfo) -> ! {
    sleyva_os::init();

    #[cfg(test)]
    test_main();
    sleyva_os::hlt_loop();
}

/// This function is called on panic.
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    sleyva_os::hlt_loop();
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    sleyva_os::test_panic_handler(info)
}

