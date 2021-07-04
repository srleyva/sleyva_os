#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(sleyva_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use bootloader::{entry_point, BootInfo};
use core::panic::PanicInfo;
use sleyva_os::{memory::init, println};
use x86_64::{structures::paging::Translate, VirtAddr};

entry_point!(kernel_main);

fn kernel_main(boot_info: &'static BootInfo) -> ! {
    sleyva_os::init();

    let mapper = unsafe { init(boot_info) };
    let addresses = [
        // VGA Buffer page
        0xb8000,
        // code page
        0x201008,
        // stack page
        0x0100_0020_1a10,
        // virtual address that should be 0x0,
        boot_info.physical_memory_offset,
    ];

    for &address in &addresses {
        let virt = VirtAddr::new(address);
        let phys = mapper.translate_addr(virt);

        println!("Virtual Address: {:?} => PhysicalAddress: {:?}", virt, phys);
    }

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
