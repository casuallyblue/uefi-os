#![no_main]
#![no_std]
#![feature(rustc_private)]
#![feature(ptr_metadata)]
#![feature(alloc_error_handler)]
#![feature(type_alias_impl_trait)]

extern crate alloc;
extern crate compiler_builtins;

use op_sys::{
    kernel::kernel_main,
    memory::{exit_and_get_runtime_memory_map, init_allocator},
    stop_cpu,
    term::framebuffer::EFIFrameBuffer,
    KernelDataInfo,
};
use uefi::prelude::*;

#[entry]
fn main(image_handle: Handle, mut system_table: SystemTable<Boot>) -> Status {
    if uefi_services::init(&mut system_table).is_err() {
        return Status::ABORTED;
    }

    if let Ok(framebuffer) = EFIFrameBuffer::init_efi_framebuffer(&mut system_table) {
        let memory_map = exit_and_get_runtime_memory_map(image_handle, system_table)?.1;

        unsafe { init_allocator(memory_map.clone()) };

        kernel_main(KernelDataInfo {
            memory_map,
            framebuffer,
        });
        stop_cpu();
    }

    Status::ABORTED
}
