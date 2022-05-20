#![no_main]
#![no_std]
#![feature(rustc_private)]
#![feature(abi_efiapi)]
#![feature(ptr_metadata)]
#![feature(alloc_error_handler)]
#![feature(type_alias_impl_trait)]

extern crate alloc;
extern crate compiler_builtins;

mod kernel;
mod memory;
mod term;

use uefi::prelude::*;

#[entry]
fn main(image_handle: Handle, mut system_table: SystemTable<Boot>) -> Status {
    if uefi_services::init(&mut system_table).is_err() {
        return Status::ABORTED; 
    }

    if let Ok(framebuffer) = term::framebuffer::EFIFrameBuffer::init_efi_framebuffer(&mut system_table) {
        let memory_map  = memory::exit_and_get_runtime_memory_map(image_handle, system_table)?.1;

        unsafe { crate::memory::init_allocator(memory_map) };
        kernel::kernel_main(framebuffer);
        stop_cpu();
    }

    Status::ABORTED
}

fn stop_cpu() -> ! {
    loop {
        unsafe { core::arch::asm!("cli", "hlt") };
    }
}
