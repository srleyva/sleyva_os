#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(sleyva_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use bootloader::{entry_point, BootInfo};
use core::panic::PanicInfo;
use sleyva_os::{exit_qemu, memory::init, serial_print, serial_println, QemuExitCode};
use x86_64::{structures::paging::Translate, PhysAddr, VirtAddr};

entry_point!(test_kernel_main);

#[cfg(feature = "map_physical_memory")]
fn test_kernel_main(boot_info: &'static BootInfo) -> ! {
    serial_println!("memory::map_physical_memory...\t");
    test_memory_mapping(boot_info);
    exit_qemu(QemuExitCode::Success);
    loop {}
}

#[cfg(feature = "recursive_mapped_memory")]
fn test_kernel_main(boot_info: &'static BootInfo) -> ! {
    serial_println!("memory::recursive_mapped_memory...\t");
    test_memory_mapping(boot_info);
    exit_qemu(QemuExitCode::Success);
    loop {}
}

fn test_memory_mapping(boot_info: &'static BootInfo) {
    let mapper = unsafe { init(boot_info) };
    let expected_address_mappings = [
        (
            "IdentityMapped",
            VirtAddr::new(0xb8000),
            PhysAddr::new(0xb8000),
        ),
        ("CodePage", VirtAddr::new(0x201008), PhysAddr::new(0x401008)),
        (
            "StackPage",
            VirtAddr::new(0x0100_0020_1a10),
            PhysAddr::new(0x278a10),
        ),
        (
            "Offset",
            VirtAddr::new(boot_info.physical_memory_offset),
            PhysAddr::new(0x0),
        ),
    ];

    for (name, virtual_address, physical_address) in &expected_address_mappings {
        serial_print!("memory_mapping::{}...\t", name);
        let mapped_physical_address = mapper.translate_addr(*virtual_address).unwrap();
        assert_eq!(mapped_physical_address, *physical_address);
        serial_println!("[Ok]");
    }
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    sleyva_os::test_panic_handler(info)
}
