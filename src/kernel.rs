use crate::interrupts::init_idt;
use crate::kprintln;

use crate::print::*;
use crate::task::basic_executor::BasicExecutor;
use crate::task::keyboard;
use crate::task::Task;
use crate::KernelData;

pub fn kernel_main(kernel_data: KernelData<'static>) {
    TERM.lock().set_framebuffer(kernel_data.framebuffer);
    TERM.lock().clear();

    kprintln!("=== BOOT SEQUENCE START ===");
    kprintln!("Initialized early framebuffer terminal");

    let mut _page_table = unsafe { crate::paging::init() };

    init_idt();

    kprintln!("Initialized Interrupts");
    kprintln!();

    let mut executor = BasicExecutor::new();
    executor.spawn(Task::new(keyboard::print_keypresses()));
    executor.run();

    panic!("End of kernel_main");
}
