pub mod allocator;

use uefi::{
    prelude::{Boot, SystemTable},
    table::boot::{MemoryMap, MemoryType},
};

#[global_allocator]
static ALLOCATOR: allocator::Locked<allocator::BumpAllocator> =
    allocator::Locked::new(allocator::BumpAllocator::new());

#[allow(unused)]
fn get_memory_slice<'a>(
    size: usize,
    system_table: &mut SystemTable<Boot>,
) -> Result<&'a mut [u8], uefi::Error> {
    let boot_services = system_table.boot_services();

    let mem = boot_services.allocate_pool(MemoryType::LOADER_DATA, size)?;
    Ok(unsafe { core::slice::from_raw_parts_mut(mem, size) })
}

pub unsafe fn init_allocator(memory_map: &MemoryMap) {
    let rest_of_mem = if let Some(mem) = memory_map
        .entries()
        .filter(|entry| entry.ty == MemoryType::CONVENTIONAL && entry.page_count >= 128)
        .last()
    {
        mem
    } else {
        return;
    };

    let start = rest_of_mem.phys_start as usize;
    let extent = 4096 * rest_of_mem.page_count as usize;

    ALLOCATOR.lock().init(start, start + extent);
}

#[alloc_error_handler]
//TODO: Dont do this
pub fn handle(_arg: core::alloc::Layout) -> ! {
    crate::stop_cpu()
}
