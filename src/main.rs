#![no_main]
#![no_std]
#![feature(abi_efiapi)]
#![feature(ptr_metadata)]
#![feature(alloc_error_handler)]
#![feature(type_alias_impl_trait)]

#[alloc_error_handler]
//TODO: Dont do this
pub fn handle(_arg: Layout) -> ! {
    stop_cpu()
}

mod framebuffer;
mod kernel;
mod memory;
mod term;

use core::{alloc::Layout, arch::asm};

use framebuffer::EFIFrameBuffer;
use uefi::prelude::*;

#[entry]
fn main(image_handle: Handle, mut system_table: SystemTable<Boot>) -> Status {
    uefi_services::init(&mut system_table).unwrap();

    if let Ok(framebuffer) = EFIFrameBuffer::init_efi_framebuffer(&mut system_table) {
        kernel::kernel_main(
            framebuffer,
            memory::exit_and_get_runtime_memory_map(image_handle, system_table)?.1,
        );
    } else {
        return Status::ABORTED;
    }

    stop_cpu();
}

fn stop_cpu() -> ! {
    unsafe {
        asm!("cli", "hlt");
    }

    #[allow(clippy::empty_loop)]
    loop {}
}
