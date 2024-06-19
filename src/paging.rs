use x86_64::{
    structures::paging::{OffsetPageTable, PageTable},
    VirtAddr,
};

pub unsafe fn active_level_4_table() -> &'static mut PageTable {
    use x86_64::registers::control::Cr3;

    let (level_4_table_frame, _) = Cr3::read();

    let phys = level_4_table_frame.start_address();
    let virt = VirtAddr::new(0) + phys.as_u64();
    let page_table_ptr: *mut PageTable = virt.as_mut_ptr();

    &mut *page_table_ptr // unsafe
}

pub unsafe fn init() -> OffsetPageTable<'static> {
    let l4_table = active_level_4_table();
    OffsetPageTable::new(l4_table, VirtAddr::new(0))
}
