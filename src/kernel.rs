use crate::memory::MemoryMap;
use rusttype::Font;

use crate::{framebuffer::EFIFrameBuffer, term::fbterm::FBTerm};

pub fn kernel_main(framebuffer: EFIFrameBuffer, memory_map: MemoryMap) {
    unsafe { crate::memory::init_allocator(memory_map) };

    let font = Font::try_from_bytes(include_bytes!("term/font.ttf")).unwrap();

    let mut term = FBTerm::new(framebuffer, font);

    term.print("Hello World!\r\n");
    term.print("Second Line");
}
