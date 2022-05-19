use core::slice::from_raw_parts_mut;

use log::info;
use uefi::{
    proto::console::gop::{GraphicsOutput, PixelFormat},
    table::{Boot, SystemTable},
    Status,
};

pub struct EFIFrameBuffer<'a> {
    ptr: &'a mut[FramebufferPixelBGR],
    height: usize,
    width: usize,
}

#[repr(packed)]
#[allow(dead_code)]
pub struct FramebufferPixelBGR {
    blue: u8,
    green: u8,
    red: u8,
    reserved: u8,
}

const FB_COLOR_PINK: FramebufferPixelBGR = FramebufferPixelBGR::new(0xFF, 0, 0xFF);

impl FramebufferPixelBGR {
    const fn new(red: u8, green: u8, blue: u8) -> FramebufferPixelBGR {
        FramebufferPixelBGR {
            reserved: 0,
            red, green, blue
        }
    }
}

impl<'a> EFIFrameBuffer<'a> {
    fn new(ptr: *mut u8, width: usize, height: usize) -> Self {
        let fb_ptr = unsafe{ from_raw_parts_mut(ptr as *mut FramebufferPixelBGR, width * height) };
        EFIFrameBuffer { ptr: fb_ptr, height, width }
    }

    pub fn draw_pixel(&mut self, x: usize, y: usize, coverage: f32) {
        if x >= self.width || y >= self.height {return;}

        self.ptr[(y * self.width) + x] = if coverage > 0.5 {
             FB_COLOR_PINK
        } else {
            FramebufferPixelBGR::new(0, 0, 0)
        };
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
            .filter(|mode| mode.info().pixel_format() == PixelFormat::Bgr).last()
        {
            mode
        } else {
            info!("Error, no supported graphics mode");
            return Err(uefi::Error::new(Status::ABORTED, ()));
        };

        graphics_output.set_mode(&mode)?;

        let (width, height) = mode.info().resolution();

        Ok(Self::new(graphics_output.frame_buffer().as_mut_ptr(), width, height))
    }
}
