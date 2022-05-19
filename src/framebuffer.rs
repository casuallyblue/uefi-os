use core::slice::from_raw_parts_mut;

use log::info;
use uefi::{
    proto::console::gop::{GraphicsOutput, PixelFormat},
    table::{Boot, SystemTable},
    Status,
};

pub struct EFIFrameBuffer {
    ptr: *mut u8,
    width: usize,
    height: usize,
}

impl EFIFrameBuffer {
    fn new(ptr: *mut u8, width: usize, height: usize) -> Self {
        EFIFrameBuffer { ptr, width, height }
    }

    pub fn draw_pixel(&mut self, x: u32, y: u32, coverage: f32) {
        let x = x as usize;
        let y = y as usize;
        let ptr = unsafe { from_raw_parts_mut(self.ptr as *mut u32, self.width * self.height) };
        if coverage >= 0.5 {
            ptr[(y * self.width) + x] = 0xFFFFFF00;
        } else {
            ptr[(y * self.width) + x] = 0x00000000;
        }
    }

    pub fn init_efi_framebuffer(system_table: &mut SystemTable<Boot>) -> Result<Self, uefi::Error> {
        let graphics_output = unsafe {
            &mut *system_table
                .boot_services()
                .locate_protocol::<GraphicsOutput>()?
                .get()
        };

        let mode = if let Some(mode) = graphics_output
            .modes()
            .find(|mode| mode.info().pixel_format() == PixelFormat::Bgr)
        {
            mode
        } else {
            info!("Error, no supported graphics mode");
            return Err(uefi::Error::new(Status::ABORTED, ()));
        };

        graphics_output.set_mode(&mode)?;

        let framebuffer_ptr = graphics_output.frame_buffer().as_mut_ptr();
        let (width, height) = mode.info().resolution();

        Ok(Self::new(framebuffer_ptr, width, height))
    }
}
