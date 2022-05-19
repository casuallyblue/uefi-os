pub mod allocator;

use uefi::{
    prelude::{Boot, SystemTable},
    table::boot::MemoryType,
    Handle,
};

pub type MemoryMap<'a> =
    impl ExactSizeIterator<Item = &'a uefi::table::boot::MemoryDescriptor> + Clone;

#[global_allocator]
static ALLOCATOR: allocator::Locked<allocator::BumpAllocator> =
    allocator::Locked::new(allocator::BumpAllocator::new());

fn get_memory_slice<'a>(
    size: usize,
    system_table: &mut SystemTable<Boot>,
) -> Result<&'a mut [u8], uefi::Error> {
    let boot_services = system_table.boot_services();

    let mem = boot_services.allocate_pool(MemoryType::LOADER_DATA, size)?;
    Ok(unsafe { core::slice::from_raw_parts_mut(mem, size) })
}

fn get_memory_map_memory<'a>(
    system_table: &mut SystemTable<Boot>,
) -> Result<&'a mut [u8], uefi::Error> {
    let boot_services = system_table.boot_services();

    let memory_map_size = boot_services.memory_map_size();
    let memory_map_alloc_size = memory_map_size.map_size * (memory_map_size.entry_size + 32);

    get_memory_slice(memory_map_alloc_size, system_table)
}

pub fn exit_and_get_runtime_memory_map<'a>(
    image_handle: Handle,
    mut system_table: SystemTable<Boot>,
) -> core::result::Result<(SystemTable<uefi::table::Runtime>, MemoryMap<'a>), uefi::Error> {
    let memory_map = get_memory_map_memory(&mut system_table)?;

    system_table.exit_boot_services(image_handle, memory_map)
}

pub unsafe fn init_allocator(memory_map: MemoryMap) {
    let rest_of_mem = if let Some(mem) = memory_map
        .filter(|entry| (**entry).ty == MemoryType::CONVENTIONAL && (**entry).page_count >= 128)
        .last()
    {
        mem
    } else {
        return;
    };

    let start = rest_of_mem.phys_start as usize;
    let extent = 4096 * 128;

    ALLOCATOR.lock().init(start, start + extent);
}
