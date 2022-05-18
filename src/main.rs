#![no_main]
#![no_std]
#![feature(abi_efiapi)]
#![feature(ptr_metadata)]
#![feature(alloc_error_handler)]

#[alloc_error_handler]
pub fn handle(_arg: Layout) -> ! {
    stop_cpu()
}

mod allocator;
mod framebuffer;
mod term;

use term::fbterm::FBTerm;

use log::info;
use rusttype::{point, Font, Rect};

use core::{alloc::Layout, arch::asm};

use uefi::{
    prelude::*,
    proto::console::gop::{GraphicsOutput, PixelFormat},
    table::boot::MemoryType,
};

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

fn exit_and_get_runtime_memory_map<'a>(
    image_handle: Handle,
    mut system_table: SystemTable<Boot>,
) -> core::result::Result<
    (
        SystemTable<uefi::table::Runtime>,
        impl ExactSizeIterator<Item = &'a uefi::table::boot::MemoryDescriptor> + Clone,
    ),
    uefi::Error,
> {
    let memory_map = get_memory_map_memory(&mut system_table)?;

    system_table.exit_boot_services(image_handle, memory_map)
}

#[entry]
fn main(image_handle: Handle, mut system_table: SystemTable<Boot>) -> Status {
    uefi_services::init(&mut system_table).unwrap();

    let graphics_output = unsafe {
        &mut *system_table
            .boot_services()
            .locate_protocol::<GraphicsOutput>()?
            .get()
    };

    let mode = if let Some(mode) = graphics_output
        .modes()
        .find(|mode| mode.info().pixel_format() == PixelFormat::Bgr)
    {
        mode
    } else {
        info!("Error, no supported graphics mode");
        return Status::ABORTED;
    };

    graphics_output.set_mode(&mode)?;

    let framebuffer_ptr = graphics_output.frame_buffer().as_mut_ptr();
    let (width, height) = mode.info().resolution();
    let framebuffer = framebuffer::EFIFrameBuffer::new(framebuffer_ptr, width, height);

    let (_rs, memory_map) = exit_and_get_runtime_memory_map(image_handle, system_table)?;

    let rest_of_mem = if let Some(mem) = memory_map
        .filter(|entry| (**entry).ty == MemoryType::CONVENTIONAL && (**entry).page_count >= 128)
        .last()
    {
        mem
    } else {
        return Status::BUFFER_TOO_SMALL;
    };

    unsafe {
        ALLOCATOR.lock().init(
            rest_of_mem.phys_start as usize,
            rest_of_mem.phys_start as usize + (128 * 4 * 1024) as usize,
        );
    }

    let font = Font::try_from_bytes(include_bytes!("font.ttf")).unwrap();

    let mut term = FBTerm::new(
        framebuffer,
        font,
        Rect {
            min: point(0, 0),
            max: point(width, height),
        },
    );
    term.print_at(0, 0, "Hello World!");

    stop_cpu();
}

fn stop_cpu() -> ! {
    unsafe {
        asm!("cli", "hlt");
    }
    #[allow(clippy::empty_loop)]
    loop {}
}
