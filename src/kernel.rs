use core::ffi::c_void;
use core::mem::transmute;

use alloc::vec;
use uefi::table::boot::MemoryMap;
use uefi::table::Runtime;
use uefi::table::SystemTable;

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
    pub memory_map: MemoryMap<'static>,
    pub system_table: SystemTable<Runtime>,
    pub kernel_image: &'a [u8],
    pub rsdt_ptr: *const c_void,
}

pub struct Kernel<'a> {
    pub kernel_image: &'a [u8],
    pub page_table: Option<OffsetPageTable<'a>>,
    pub frame_allocator: Option<BootInfoFrameAllocator<'a>>,
}

impl<'a> Kernel<'a> {
    /// Set the current framebuffer
    fn set_framebuffer(&self, framebuffer: EFIFrameBuffer<'static>) {
        TERM.lock().set_framebuffer(framebuffer);
        TERM.lock().clear();
    }

    /// Create a frame allocator from the efi memory map to allocate unused frames
    /// Warning: Should not be called more than once
    /// TODO: Do something to prevent it being called more than once
    unsafe fn init_frame_allocator(&mut self, memory_map: &'a MemoryMap<'a>) {
        let mut regions_to_skip = vec![self.kernel_image.as_ptr_range()];

        if let Some(framebuffer) = &TERM.lock().framebuffer {
            regions_to_skip.push(transmute::<
                core::ops::Range<*const crate::term::framebuffer_color::FramebufferPixelBGR>,
                core::ops::Range<*const u8>,
            >(framebuffer.pixels.as_ptr_range()));
        }

        self.frame_allocator = Some(BootInfoFrameAllocator::new(memory_map, regions_to_skip));
    }

    /// Load the existing page table from CR3
    /// TODO: Update the page table for the actual OS instead
    /// of keeping the UEFI page table
    fn init_paging(&mut self) {
        self.page_table = Some(unsafe { crate::paging::init() });
    }

    pub fn new(efi_data: &'a EFIStructures<'static>) -> Self {
        let mut kernel = Kernel {
            kernel_image: efi_data.kernel_image,
            page_table: None,
            frame_allocator: None,
        };

        kernel.set_framebuffer(unsafe { efi_data.framebuffer.unsafe_clone() });
        unsafe { kernel.init_frame_allocator(&efi_data.memory_map) };

        kernel
    }

    pub fn main(&mut self) {
        init_idt();

        kprintln!("=== BOOT SEQUENCE START ===");
        kprintln!("Initialized early framebuffer terminal");

        self.init_paging();

        // Set up a simple async executor to write the keyboard input to the terminal
        let mut executor = BasicExecutor::new();
        executor.spawn(Task::new(keyboard::print_keypresses()));
        executor.run();

        panic!("End of Kernel::main");
    }
}
