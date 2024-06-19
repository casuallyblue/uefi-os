use pic8259::ChainedPics;
use spin::Mutex;

pub const PIC_1_OFFSET: u8 = 32;
pub const PIC_2_OFFSET: u8 = PIC_1_OFFSET + 8;

pub static PICS: Mutex<ChainedPics> =
    Mutex::new(unsafe { ChainedPics::new(PIC_1_OFFSET, PIC_2_OFFSET) });

pub fn load_pics() {
    unsafe { PICS.lock().initialize() };
    unsafe { PICS.lock().write_masks(0b11111100, 0xFF) };
    x86_64::instructions::interrupts::enable();
}
