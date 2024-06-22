use core::ops::Mul;

// TODO: factor out this code per pixel format supported by EFI

#[repr(packed)]
#[allow(dead_code)]
#[derive(Clone, PartialEq)]
pub struct FramebufferPixelBGR {
    blue: u8,
    green: u8,
    red: u8,
    reserved: u8,
}

impl From<FBColor> for FramebufferPixelBGR {
    fn from(color: FBColor) -> Self {
        match color {
            FBColor::Pink => FB_COLOR_PINK,
            FBColor::Rgb(r, g, b) => Self::new(r, g, b),
        }
    }
}

impl From<FramebufferPixelBGR> for FBColor {
    fn from(pixel: FramebufferPixelBGR) -> Self {
        match pixel {
            p if p == FB_COLOR_PINK => FBColor::Pink,
            p => FBColor::Rgb(p.red, p.green, p.blue),
        }
    }
}

const FB_COLOR_PINK: FramebufferPixelBGR = FramebufferPixelBGR::new(0xFF, 0, 0xFF);

impl FramebufferPixelBGR {
    pub const fn new(red: u8, green: u8, blue: u8) -> FramebufferPixelBGR {
        FramebufferPixelBGR {
            reserved: 0,
            red,
            green,
            blue,
        }
    }
}

/// Colors supported by this framebuffer terminal
/// TODO: make this apply specifically to the
/// framebuffer terminal rather than be stored
/// with the pixel format code
#[derive(Debug, Copy, Clone)]
pub enum FBColor {
    Pink,
    Rgb(u8, u8, u8),
}

impl FBColor {
    fn fbcolor_to_rgb(color: Self) -> Self {
        match color {
            FBColor::Pink => FB_COLOR_PINK.into(),
            FBColor::Rgb(r, g, b) => FBColor::Rgb(r, g, b),
        }
    }
}

/// TODO: unsure how this is used (its almost certainly wrong)
impl Mul<f32> for FBColor {
    type Output = FBColor;

    fn mul(self, rhs: f32) -> Self::Output {
        let (r, g, b) = if let FBColor::Rgb(r, g, b) = FBColor::fbcolor_to_rgb(self) {
            (r as f32, g as f32, b as f32)
        } else {
            (0.0, 0.0, 0.0)
        };

        let (r, g, b) = (r * rhs, g * rhs, b * rhs);
        let (r, g, b) = (r as u8, g as u8, b as u8);

        FBColor::Rgb(r, g, b)
    }
}
