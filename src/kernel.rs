use crate::interrupts::init_idt;
use crate::kprintln;

use crate::print::*;
use crate::KernelData;

pub fn kernel_main(kernel_data: KernelData<'static>) {
    TERM.lock().set_framebuffer(kernel_data.framebuffer);
    TERM.lock().clear();

    kprintln!("=== BOOT SEQUENCE START ===");
    kprintln!("Initialized early framebuffer terminal");

    init_idt();

    loop {}

    panic!("End of kernel_main");
}
