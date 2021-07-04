#![no_std]
#![feature(abi_x86_interrupt)]
#![no_main]

use core::panic::PanicInfo;
use lazy_static::lazy_static;
use sleyva_os::serial_print;
use x86_64::structures::idt::{InterruptDescriptorTable, PageFaultErrorCode};

lazy_static! {
    static ref TEST_IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        idt.page_fault.set_handler_fn(test_page_fault_handler);
        idt
    };
}

pub fn init_test_idt() {
    TEST_IDT.load();
}

use sleyva_os::{exit_qemu, serial_println, QemuExitCode};
use x86_64::structures::idt::InterruptStackFrame;

extern "x86-interrupt" fn test_page_fault_handler(
    _stack_frame: InterruptStackFrame,
    _error_code: PageFaultErrorCode,
) {
    use x86_64::registers::control::Cr2;
    use x86_64::VirtAddr;

    let actual_addr = Cr2::read();
    let expected_addr = VirtAddr::new(0xd3adb33f);

    assert_eq!(actual_addr, expected_addr);
    serial_println!("[ok]");
    exit_qemu(QemuExitCode::Success);
    sleyva_os::hlt_loop();
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    serial_print!("page_fault::page_fault...\t");
    sleyva_os::gdt::init();
    init_test_idt();

    let ptr = 0xd3adb33f as *mut u32;
    unsafe {
        *ptr = 42;
    }
    sleyva_os::hlt_loop();
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    sleyva_os::test_panic_handler(info)
}
