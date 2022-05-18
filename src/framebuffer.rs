use core::slice::from_raw_parts_mut;

pub struct EFIFrameBuffer {
    ptr: *mut u8,
    width: usize,
    height: usize,
}
impl EFIFrameBuffer {
    pub fn new(ptr: *mut u8, width: usize, height: usize) -> Self {
        EFIFrameBuffer { ptr, width, height }
    }

    pub fn draw_pixel(&mut self, x: u32, y: u32, coverage: f32) {
        let x = x as usize;
        let y = y as usize;
        let ptr = unsafe { from_raw_parts_mut(self.ptr as *mut u32, self.width * self.height) };
        if coverage >= 0.5 {
            ptr[(y * self.width) + x] = 0x00FF00FF;
        } else {
            ptr[(y * self.width) + x] = 0x00000000;
        }
    }
}
