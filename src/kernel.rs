use uefi::table::boot::MemoryType;

use crate::frame_allocator::BootInfoFrameAllocator;
use crate::kprintln;

use crate::paging::create_page_table;
use crate::{print::*, KernelDataInfo};

pub fn kernel_main(kernel_data: KernelDataInfo<'static>) {
    TERM.lock().set_framebuffer(kernel_data.framebuffer);
    TERM.lock().clear();

    kprintln!("=== BOOT SEQUENCE START ===");
    kprintln!("Initialized early framebuffer terminal");

    let mut frame_allocator =
        unsafe { BootInfoFrameAllocator::new(kernel_data.memory_map.clone()) };

    kprintln!("creating page table");
    let _page_table = create_page_table(&mut frame_allocator).unwrap();
    kprintln!("page table created");

    let mut etr = None;
    let mut fbs = 0;

    for entry in kernel_data
        .memory_map
        .clone()
        .filter(|e| e.ty == MemoryType::ACPI_NON_VOLATILE)
    {
        kprintln!(
            "memory found with type {:?} and start {:#x} with {} pages",
            entry.ty,
            entry.phys_start,
            entry.page_count
        );
    }

    match &TERM.lock().framebuffer {
        Some(framebuffer) => {
            let ptr = framebuffer.pixels.as_ptr() as *mut u8;

            for entry in kernel_data.memory_map.filter(|e| {
                e.phys_start <= ptr as u64 && (e.phys_start + (4096 * e.page_count)) >= ptr as u64
            }) {
                etr = Some(entry);
                fbs = ptr as u64;
            }
        }
        _ => {}
    }

    kprintln!("1920x1080 = {}", 1920 * 1080);
    kprintln!("framebuffer starts at {:#x}", fbs);

    kprintln!(
        "framebuffer descriptor at {:#x}, extent {} pixels, type {:?}",
        etr.unwrap().phys_start,
        (etr.unwrap().page_count * 4096) / 4,
        etr.unwrap().ty
    );

    panic!("End of kernel_main");
}
