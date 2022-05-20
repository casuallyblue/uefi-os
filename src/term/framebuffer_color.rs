#[repr(packed)]
#[allow(dead_code)]
#[derive(Clone)]
pub struct FramebufferPixelBGR {
    blue: u8,
    green: u8,
    red: u8,
    reserved: u8,
}

impl From<FBColor> for FramebufferPixelBGR {
    fn from(color: FBColor) -> Self {
        match color {
            FBColor::Pink => {
                FB_COLOR_PINK
            }
            FBColor::Rgb(r,g,b) => {
                Self::new(r,g,b)
            }
        }
    }
}

const FB_COLOR_PINK: FramebufferPixelBGR = FramebufferPixelBGR::new(0xFF, 0, 0xFF);

impl FramebufferPixelBGR {
    pub const fn new(red: u8, green: u8, blue: u8) -> FramebufferPixelBGR {
        FramebufferPixelBGR {
            reserved: 0,
            red, green, blue
        }
    }
}

#[derive(Debug, Clone)]
pub enum FBColor {
    Pink,
    Rgb(u8, u8, u8)
}
