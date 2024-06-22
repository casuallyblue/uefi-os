use core::ffi::c_void;
use core::mem::transmute;

use alloc::vec;
use uefi::table::boot::MemoryMap;
use uefi::table::Runtime;
use uefi::table::SystemTable;
use x86_64::structures::paging::page_table;
use x86_64::structures::paging::FrameAllocator;
use x86_64::structures::paging::OffsetPageTable;

use crate::frame_allocator::BootInfoFrameAllocator;
use crate::interrupts::init_idt;
use crate::kprintln;

use crate::print::*;
use crate::task::basic_executor::BasicExecutor;
use crate::task::keyboard;
use crate::task::Task;
use crate::term::framebuffer::EFIFrameBuffer;

pub struct EFIStructures<'a> {
    pub framebuffer: EFIFrameBuffer<'static>,
    pub memory_map: MemoryMap<'a>,
    pub system_table: SystemTable<Runtime>,
    pub kernel_image: &'a [u8],
    pub rsdt_ptr: *const c_void,
}

pub struct Kernel<'a> {
    pub kernel_image: &'a [u8],
    pub framebuffer: EFIFrameBuffer<'static>,
    pub page_table: Option<OffsetPageTable<'a>>,
    pub frame_allocator: Option<BootInfoFrameAllocator<'a>>,
}

impl<'a> Kernel<'a> {
    fn set_framebuffer(&self) {
        TERM.lock()
            .set_framebuffer(unsafe { self.framebuffer.unsafe_clone() });
        TERM.lock().clear();
    }

    /// Create a frame allocator from the efi memory map to allocate unused frames
    /// Warning: Should not be called more than once
    /// TODO: Do something to prevent it being called more than once
    unsafe fn init_frame_allocator(&mut self, memory_map: &'a MemoryMap<'a>) {
        let regions_to_skip = vec![
            self.kernel_image.as_ptr_range(),
            transmute(self.framebuffer.pixels.as_ptr_range()),
        ];

        self.frame_allocator = Some(BootInfoFrameAllocator::new(memory_map, regions_to_skip));
    }

    fn init_paging(&mut self) {
        self.page_table = Some(unsafe { crate::paging::init() });
    }
}

pub fn kernel_main(efi_data: EFIStructures<'static>) {
    let mut kernel = Kernel {
        kernel_image: &efi_data.kernel_image,
        framebuffer: efi_data.framebuffer,
        page_table: None,
        frame_allocator: None,
    };

    kernel.set_framebuffer();

    kprintln!("=== BOOT SEQUENCE START ===");
    kprintln!("Initialized early framebuffer terminal");

    kernel.init_paging();

    init_idt();

    unsafe { kernel.init_frame_allocator(&efi_data.memory_map) };

    let mut executor = BasicExecutor::new();
    executor.spawn(Task::new(keyboard::print_keypresses()));
    executor.run();

    panic!("End of kernel_main");
}
