#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(sleyva_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use sleyva_os::{exit_qemu, QemuExitCode, serial_print, serial_println, memory::init};
use x86_64::{VirtAddr, structures::paging::MapperAllSizes, PhysAddr};
use bootloader::{BootInfo, entry_point};
use core::panic::PanicInfo;

entry_point!(test_kernel_main);

fn test_kernel_main(boot_info: &'static BootInfo) -> ! {
    serial_println!("memory::memory_mapping...\t");
    test_memory_mapping(boot_info.physical_memory_offset);
    exit_qemu(QemuExitCode::Success);
    loop {}
}

fn test_memory_mapping(physical_memory_offset: u64) {
    let mapper = unsafe {init(VirtAddr::new(physical_memory_offset))};
    let expected_address_mappings = [
        ("IdentityMapped", VirtAddr::new(0xb8000), PhysAddr::new(0xb8000)),
        ("CodePage", VirtAddr::new(0x201008), PhysAddr::new(0x401008)),
        ("StackPage", VirtAddr::new(0x0100_0020_1a10), PhysAddr::new(0x278a10)),
        ("Offset", VirtAddr::new(physical_memory_offset), PhysAddr::new(0x0)),
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