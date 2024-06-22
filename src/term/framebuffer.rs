use core::{fmt::Debug, slice::from_raw_parts_mut};

use uefi::{
    proto::console::gop::{GraphicsOutput, PixelFormat},
    table::{boot::SearchType, Boot, SystemTable},
    Identify, Status,
};

use super::framebuffer_color::FramebufferPixelBGR;

pub struct EFIFrameBuffer<'a> {
    pub pixels: &'a mut [FramebufferPixelBGR],
    height: usize,
    width: usize,
}

impl<'a> Debug for EFIFrameBuffer<'a> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "({}-{})", self.height, self.width)
    }
}

impl<'a> EFIFrameBuffer<'a> {
    fn new(ptr: *mut u8, width: usize, height: usize) -> Self {
        let fb_ptr = unsafe { from_raw_parts_mut(ptr as *mut FramebufferPixelBGR, width * height) };
        EFIFrameBuffer {
            pixels: fb_ptr,
            height,
            width,
        }
    }

    /// Get an unsafe copy of the framebuffer data
    /// TODO: rewrite the kernel startup code to not
    /// need this and instead just ask for a pointer
    /// or range
    pub unsafe fn unsafe_clone(&self) -> Self {
        let pixels = self.pixels.as_ptr();
        let size = self.pixels.len();

        EFIFrameBuffer {
            pixels: unsafe { from_raw_parts_mut(pixels as *mut FramebufferPixelBGR, size) },
            height: self.height,
            width: self.width,
        }
    }

    /// Move all the pixels in a row left
    /// TODO: make this take pixel format into account in case it is not
    /// four bytes
    pub fn shift_left(&mut self, offset: usize) {
        let num_pixels = (self.height * self.width) - offset;
        unsafe {
            compiler_builtins::mem::memcpy(
                self.pixels.as_mut_ptr() as *mut u8,
                (self.pixels.as_ptr() as usize + (offset * 4)) as *const u8,
                num_pixels * 4,
            )
        };
    }

    /// Draw to a specific pixel on the framebuffer memory
    /// TODO: make this generic over pixel format
    pub fn draw_pixel(&mut self, x: usize, y: usize, color: &FramebufferPixelBGR) {
        if x >= self.width || y >= self.height {
            return;
        }

        self.pixels[(y * self.width) + x] = color.clone();
    }

    pub fn get_width(&self) -> usize {
        self.width
    }

    pub fn get_height(&self) -> usize {
        self.height
    }

    /// Invoke EFI routines to set up an area of memory for the framebuffer.
    /// Currently we ask for a lower resolution than the max supported
    /// in order for it to fit well as a smaller window on my monitor
    pub fn init_efi_framebuffer(system_table: &mut SystemTable<Boot>) -> Result<Self, uefi::Error> {
        let graphics_output_handle = *system_table
            .boot_services()
            .locate_handle_buffer(SearchType::ByProtocol(&GraphicsOutput::GUID))?
            .first()
            .unwrap();

        let graphics_output = &mut *system_table
            .boot_services()
            .open_protocol_exclusive::<GraphicsOutput>(graphics_output_handle)?;

        // Locate the last mode fitting the required parameters
        // This was more important when not looking for a specific
        // resolution
        let mode = if let Some(mode) = graphics_output
            .modes(system_table.boot_services())
            .filter(|mode| mode.info().pixel_format() == PixelFormat::Bgr)
            .filter(|mode| mode.info().resolution() == (1024, 768))
            .last()
        {
            mode
        } else {
            return Err(uefi::Error::new(Status::ABORTED, ()));
        };

        graphics_output.set_mode(&mode)?;

        // Store the display size so we can refer to it later
        // after boot services go away
        let (width, height) = mode.info().resolution();

        Ok(Self::new(
            graphics_output.frame_buffer().as_mut_ptr(),
            width,
            height,
        ))
    }
}
