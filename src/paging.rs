use x86_64::structures::paging::{
    page_table::PageTableEntry, FrameAllocator, PageTable, PageTableFlags, PhysFrame,
};

use crate::frame_allocator::BootInfoFrameAllocator;

fn create_page_table_frame(
    frame_allocator: &mut BootInfoFrameAllocator,
) -> Option<(PhysFrame, &mut PageTable)> {
    let frame = frame_allocator.allocate_frame()?;

    let pt: *mut PageTable = frame.start_address().as_u64() as *mut PageTable;

    let table: &mut PageTable = unsafe { &mut *pt };

    Some((frame, table))
}

pub fn create_page_table(frame_allocator: &mut BootInfoFrameAllocator) -> Option<PhysFrame> {
    let (frame, table) = create_page_table_frame(frame_allocator)?;

    for (_, entry) in table.iter_mut().enumerate() {
        entry.set_unused();
    }

    Some(frame)
}

pub fn create_4_level_page_table(
    frame_allocator: &mut BootInfoFrameAllocator,
) -> Option<PhysFrame> {
    let (lvl4_frame, lvl4_table) = create_page_table_frame(frame_allocator)?;

    for entry in lvl4_table.iter_mut() {
        entry.set_unused();
    }

    lvl4_table[511] = PageTableEntry::new();
    lvl4_table[511].set_addr(
        lvl4_frame.start_address(),
        PageTableFlags::PRESENT & PageTableFlags::WRITABLE,
    );

    Some(lvl4_frame)
}
