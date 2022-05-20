use crate::{term::framebuffer::EFIFrameBuffer, kprintln, kprint};

use crate::print::*;

pub fn kernel_main(framebuffer: EFIFrameBuffer<'static>) {

    TERM.lock().set_framebuffer(framebuffer);

    TERM.lock().set_background(crate::term::framebuffer_color::FBColor::Rgb(0x50,0x20,0x50));
    TERM.lock().clear();

    kprintln!("=== BOOT SEQUENCE START ===");
    kprintln!("Initialized early framebuffer terminal");

    panic!("End of kernel_main");
}
