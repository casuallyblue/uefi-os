use uefi::table::boot::MemoryType;
use x86_64::instructions::interrupts::int3;
use x86_64::registers::control::Cr3;

use crate::term::framebuffer_color::FBColor;
use crate::{interrupts, kprint, kprintln};

use crate::{print::*, KernelDataInfo};

pub fn kernel_main(kernel_data: KernelDataInfo<'static>) {
    TERM.lock().set_framebuffer(kernel_data.framebuffer);

    TERM.lock().set_background(FBColor::Rgb(0, 0, 0));
    TERM.lock().clear();

    kprintln!("=== BOOT SEQUENCE START ===");
    kprintln!("Initialized early framebuffer terminal");

    kprintln!("Looking over memory map:");

    for mem in kernel_data.memory_map.clone().filter(|m| {
        m.ty != MemoryType::BOOT_SERVICES_DATA && m.ty != MemoryType::BOOT_SERVICES_CODE
    }) {
        kprintln!(
            "type: {:?}, phys_start: {:#x}, virt_start: {:#x}, pages: {}, attrs: {:?}",
            mem.ty,
            mem.phys_start,
            mem.virt_start,
            mem.page_count,
            mem.att
        );
    }

    let boot_services_pages = kernel_data
        .memory_map
        .clone()
        .filter(|m| {
            m.ty == MemoryType::BOOT_SERVICES_CODE || m.ty == MemoryType::BOOT_SERVICES_DATA
        })
        .fold(0, |n, mem| n + mem.page_count);

    kprintln!("Number of boot services pages: {}", boot_services_pages);

    interrupts::init_idt();

    let cr3flags = Cr3::read();
    let addr = cr3flags.0.start_address().as_u64();

    for mem in kernel_data
        .memory_map
        .filter(|m| m.phys_start <= addr && m.phys_start + (4096 * m.page_count) >= addr)
    {
        kprintln!("{:#x?}", mem);
    }

    kprintln!("{:?}", cr3flags);

    panic!("End of kernel_main");
}
