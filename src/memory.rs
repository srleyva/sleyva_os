use x86_64::{
    structures::paging::{
        PageTable,
        page_table::FrameError,
        OffsetPageTable,
    },
    registers::control::Cr3,
    VirtAddr,
    PhysAddr,
};

pub unsafe fn init(physical_memory_offset: VirtAddr) -> OffsetPageTable<'static> {
    let l4_table = active_level_4_table(physical_memory_offset);
    OffsetPageTable::new(l4_table, physical_memory_offset)
}

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