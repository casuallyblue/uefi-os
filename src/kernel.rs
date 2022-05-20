use core::panic::PanicInfo;

use crate::memory::MemoryMap;
use lazy_static::lazy_static;
use ab_glyph::FontRef;
use spin::Mutex;

use crate::term::{framebuffer::EFIFrameBuffer, fbterm::FBTerm};

macro_rules! kprint {
    ($($arg:tt)*) => (write!(TERM.lock(), "{}", format_args!($($arg)*)));
}

macro_rules! kprintln {
    () => {
        (kprint!("\n"));
    };
    ($($arg:tt)*) => {
        (kprint!("{}\n", format_args!($($arg)*)));
    };
}

lazy_static! {
    pub static ref TERM: Mutex<FBTerm<'static>> = Mutex::new(
        FBTerm::new_unset(
            FontRef::try_from_slice(include_bytes!("term/font.ttf")).unwrap()));
}

pub fn kernel_main(framebuffer: EFIFrameBuffer<'static>) {

    TERM.lock().set_framebuffer(framebuffer);

    TERM.lock().set_background(crate::term::framebuffer_color::FBColor::Rgb(0x50,0x20,0x50));
    TERM.lock().clear();

    kprintln!("=== BOOT SEQUENCE START ===");
    kprintln!("Initialized early framebuffer terminal");

    panic!("End of kernel_main");
}

#[panic_handler]
pub fn panic_handler(panic_info: &PanicInfo) -> ! {
    kprintln!("{}", panic_info);

    crate::stop_cpu();
}
