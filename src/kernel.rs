use crate::interrupts::init_idt;
use crate::kprintln;

use crate::print::*;
use crate::task::basic_executor::BasicExecutor;
use crate::task::Task;
use crate::KernelData;

pub fn kernel_main(kernel_data: KernelData<'static>) {
    TERM.lock().set_framebuffer(kernel_data.framebuffer);
    TERM.lock().clear();

    kprintln!("=== BOOT SEQUENCE START ===");
    kprintln!("Initialized early framebuffer terminal");

    init_idt();

    kprintln!("Initialized Interrupts");

    kprintln!("Beginning async runtime");

    let mut executor = BasicExecutor::new();
    executor.spawn(Task::new(kernel_async_main()));
    executor.run();

    panic!("End of kernel_main");
}

pub async fn kernel_async_main() {
    loop {}
}
