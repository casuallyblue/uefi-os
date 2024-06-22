#![no_main]
#![no_std]
#![feature(rustc_private)]
#![feature(ptr_metadata)]
#![feature(alloc_error_handler)]
#![feature(type_alias_impl_trait)]

extern crate alloc;

use op_sys::{
    kernel::{EFIStructures, Kernel},
    memory::init_allocator,
    stop_cpu,
    term::framebuffer::EFIFrameBuffer,
};
use uefi::{
    prelude::*,
    proto::loaded_image::LoadedImage,
    table::{boot::MemoryType, cfg::ACPI2_GUID},
};

#[entry]
fn main(image_handle: Handle, mut system_table: SystemTable<Boot>) -> Status {
    if uefi::helpers::init(&mut system_table).is_err() {
        return Status::ABORTED;
    }

    let rsdt_ptr = system_table
        .config_table()
        .iter()
        .filter(|entry| entry.guid == ACPI2_GUID)
        .next()
        .unwrap()
        .address;

    let (location, size) = system_table
        .boot_services()
        .open_protocol_exclusive::<LoadedImage>(image_handle)
        .unwrap()
        .info();

    if let Ok(framebuffer) = EFIFrameBuffer::init_efi_framebuffer(&mut system_table) {
        let (system_table, memory_map) =
            system_table.exit_boot_services(MemoryType::RUNTIME_SERVICES_DATA);

        unsafe { init_allocator(&memory_map) };

        let efi_data = EFIStructures {
            framebuffer,
            memory_map,
            system_table,
            rsdt_ptr,
            kernel_image: unsafe {
                core::slice::from_raw_parts(location as *const u8, size as usize)
            },
        };

        Kernel::new(&efi_data).main();

        stop_cpu();
    }

    Status::ABORTED
}
