use x86_64::{
    structures::paging::{
        PageTable,
        page_table::FrameError,
        OffsetPageTable,
        RecursivePageTable,
    },
    registers::control::Cr3,
    VirtAddr,
    PhysAddr,
};
use bootloader::BootInfo;

#[cfg(feature = "map_physical_memory")]
pub unsafe fn init(boot_info: &'static BootInfo) -> OffsetPageTable<'static> {
    let mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let l4_table = active_level_4_table(mem_offset);
    OffsetPageTable::new(l4_table, mem_offset)
}

#[cfg(feature = "map_physical_memory")]
unsafe fn active_level_4_table(physical_memory_offset: VirtAddr) -> &'static mut PageTable {
    // Read the active level 4 frame from the CR3 register
    let (active_level_4_frame, _) = Cr3::read();

    // Grab the start address and add the physical_memory_offset to derive the mapped Virtual Address
    let phys_addr = active_level_4_frame.start_address();
    let virtual_address = physical_memory_offset + phys_addr.as_u64();

    // return mutable reference to the Page Table
    let page_tbl_ptr: *mut PageTable = virtual_address.as_mut_ptr();
    &mut *page_tbl_ptr
}

#[cfg(feature = "recursive_mapped_memory")]
pub unsafe fn init(boot_info: &'static BootInfo) -> RecursivePageTable<'static> {
    let l4_virtual_address = VirtAddr::new(boot_info.recursive_page_table_addr);
    let l4_table = active_level_4_table(l4_virtual_address);
    RecursivePageTable::new(l4_table).expect("invalid page table")
}

#[cfg(feature = "recursive_mapped_memory")]
unsafe fn active_level_4_table(l4_virtual_address: VirtAddr) -> &'static mut PageTable {
    // return mutable reference to the Page Table
    let page_tbl_ptr: *mut PageTable = l4_virtual_address.as_mut_ptr();
    &mut *page_tbl_ptr
}