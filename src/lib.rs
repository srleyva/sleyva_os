#![no_std]
#![feature(asm)]
#![cfg_attr(test, no_main)]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]
#![feature(abi_x86_interrupt)]

extern crate rlibc;

use core::panic::PanicInfo;

pub mod cpu_info;
pub mod gdt;
pub mod interrupts;
pub mod memory;
pub mod serial;
pub mod vga_buffer;

pub fn init() {
    gdt::init();
    interrupts::init_idt();
    unsafe { interrupts::PICS.lock().initialize() };
    x86_64::instructions::interrupts::enable();
    banner();
}

fn banner() {
    let banner = r#"
   _____ __    ________  ___    _____        ____  _____
  / ___// /   / ____/\ \/ / |  / /   |      / __ \/ ___/
  \__ \/ /   / __/    \  /| | / / /| |     / / / /\__ \ 
 ___/ / /___/ /___    / / | |/ / ___ |    / /_/ /___/ / 
/____/_____/_____/   /_/  |___/_/  |_|____\____//____/  
                                    /_____/             "#;

    println!("{}", banner);
    println!("{}", *cpu_info::CPU_INFO);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum QemuExitCode {
    Success = 0x10, // 16 (00010000 << 1 | 1) == (00100000 | 00000001) == 00100001 == 33
    Failed = 0x11,  // 17 (00010001 << 1 | 1) == (00100010 | 00000001) == 00100011 == 35
}

pub fn hlt_loop() -> ! {
    loop {
        x86_64::instructions::hlt();
    }
}

pub fn exit_qemu(exit_code: QemuExitCode) {
    use x86_64::instructions::port::Port;

    unsafe {
        let mut port = Port::new(0xf4);
        port.write(exit_code as u32);
    }
}

pub trait Testable {
    fn run(&self) -> ();
}

impl<T> Testable for T
where
    T: Fn(),
{
    fn run(&self) {
        serial_print!("{}...\t", core::any::type_name::<T>());
        self();
        serial_println!("[ok]");
    }
}

pub fn test_runner(tests: &[&dyn Testable]) {
    serial_println!("Running {} tests", tests.len());
    for test in tests {
        test.run();
    }
    exit_qemu(QemuExitCode::Success);
}

pub fn test_panic_handler(info: &PanicInfo) -> ! {
    serial_println!("[failed]\n");
    serial_println!("Error: {}\n", info);
    exit_qemu(QemuExitCode::Failed);
    hlt_loop();
}

/// Entry point for `cargo test`
#[cfg(test)]
#[no_mangle]
pub extern "C" fn _start() -> ! {
    init();
    test_main();
    hlt_loop();
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    test_panic_handler(info)
}
