#![no_main]
#![no_std]
#![feature(rustc_private)]
#![feature(abi_efiapi)]
#![feature(ptr_metadata)]
#![feature(alloc_error_handler)]
#![feature(type_alias_impl_trait)]

extern crate alloc;
extern crate compiler_builtins;

use uefi::prelude::*;

#[entry]
fn main(image_handle: Handle, mut system_table: SystemTable<Boot>) -> Status {
    if uefi_services::init(&mut system_table).is_err() {
        return Status::ABORTED; 
    }

    if let Ok(framebuffer) = op_sys::term::framebuffer::EFIFrameBuffer::init_efi_framebuffer(&mut system_table) {
        let memory_map  = op_sys::memory::exit_and_get_runtime_memory_map(image_handle, system_table)?.1;

        unsafe { op_sys::memory::init_allocator(memory_map) };

        op_sys::kernel::kernel_main(framebuffer);
        op_sys::stop_cpu();
    }

    Status::ABORTED
}


