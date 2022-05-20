#![no_std]
#![feature(rustc_private)]
#![feature(abi_efiapi)]
#![feature(ptr_metadata)]
#![feature(alloc_error_handler)]
#![feature(type_alias_impl_trait)]

use core::panic::PanicInfo;

extern crate alloc;
extern crate compiler_builtins;

pub mod kernel;
pub mod memory;
pub mod term;

#[macro_use]
pub mod print;

use print::*;

pub fn stop_cpu() -> ! {
    loop {
        unsafe { core::arch::asm!("cli", "hlt") };
    }
}

#[panic_handler]
pub fn panic_handler(panic_info: &PanicInfo) -> ! {
    kprintln!("{}", panic_info);

    crate::stop_cpu();
}
