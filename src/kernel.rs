use x86_64::structures::paging::FrameAllocator;

use crate::frame_allocator::BootInfoFrameAllocator;
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

    let mut frame_allocator = unsafe { BootInfoFrameAllocator::new(&kernel_data.memory_map) };

    let frame = frame_allocator.allocate_frame();
    let frame_2 = frame_allocator.allocate_frame();

    kprintln!("{:?}", frame);
    kprintln!("{:?}", frame_2);

    let mut executor = BasicExecutor::new();
    executor.spawn(Task::new(keyboard::print_keypresses()));
    executor.run();

    panic!("End of kernel_main");
}
