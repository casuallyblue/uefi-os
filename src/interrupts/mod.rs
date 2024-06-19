use lazy_static::lazy_static;
use pics::{load_pics, PICS};
use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame, PageFaultErrorCode};

use crate::{kprint, kprintln, TERM};

pub mod pics;

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum InterruptIndex {
    Timer = 32,
    Keyboard = 33,
}

lazy_static! {
    static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        idt.breakpoint.set_handler_fn(breakpoint_handler);
        idt.page_fault.set_handler_fn(page_fault_handler);
        idt[InterruptIndex::Timer as usize].set_handler_fn(timer_handler);
        idt
    };
}

pub fn init_idt() {
    x86_64::instructions::interrupts::disable();
    IDT.load();
    load_pics();
}

extern "x86-interrupt" fn breakpoint_handler(stack_frame: InterruptStackFrame) {
    kprintln!("EXCEPTION: BREAKPOINT\n{:#?}", stack_frame);
}

extern "x86-interrupt" fn page_fault_handler(
    stack_frame: InterruptStackFrame,
    error_code: PageFaultErrorCode,
) {
    kprintln!(
        "error code: {:?} stack_frame: {:?}",
        error_code,
        stack_frame
    )
}

extern "x86-interrupt" fn timer_handler(_stack_frame: InterruptStackFrame) {
    x86_64::instructions::interrupts::disable();
    kprint!(".");
    x86_64::instructions::interrupts::enable();
    unsafe {
        PICS.lock()
            .notify_end_of_interrupt(InterruptIndex::Timer as u8);
    }
}
