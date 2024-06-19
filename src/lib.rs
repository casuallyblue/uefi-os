#![no_std]
#![feature(rustc_private)]
#![feature(ptr_metadata)]
#![feature(alloc_error_handler)]
#![feature(type_alias_impl_trait)]
#![feature(abi_x86_interrupt)]

use core::panic::PanicInfo;

extern crate alloc;
extern crate compiler_builtins;

pub mod frame_allocator;
pub mod interrupts;
pub mod kernel;
pub mod memory;
pub mod paging;
pub mod task;
pub mod term;

#[macro_use]
pub mod print;

use print::*;
use term::framebuffer::EFIFrameBuffer;
use uefi::table::boot::MemoryMap;

pub struct KernelData<'a> {
    pub framebuffer: EFIFrameBuffer<'a>,
    pub memory_map: MemoryMap<'a>,
}

pub fn stop_cpu() -> ! {
    loop {
        unsafe { core::arch::asm!("cli", "hlt") };
    }
}

#[panic_handler]
pub fn panic_handler(panic_info: &PanicInfo) -> ! {
    x86_64::instructions::interrupts::disable();
    kprintln!("{}", panic_info);

    crate::stop_cpu();
}
