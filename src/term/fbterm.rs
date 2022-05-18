use rusttype::{Font, Rect, Scale, point};

use crate::framebuffer::EFIFrameBuffer;

pub struct FBTerm<'a>{
    framebuffer: EFIFrameBuffer, 
    term_font: Font<'a>,
    _bounds: Rect<usize>,
}

impl<'a> FBTerm<'a> {
    pub fn new(framebuffer: EFIFrameBuffer, font: Font<'a>, size: Rect<usize>) -> Self {
        FBTerm {
            framebuffer,
            term_font: font,
            _bounds: size
        }
    }

    pub fn print_at<S>(&mut self, _x: usize, _y: usize, s: S) where S: AsRef<str> {
    let mut x_offset = 0;

    let ascent = self.term_font.v_metrics(Scale::uniform(25.0)).ascent;
    let glyphs = self.term_font.layout(s.as_ref(), Scale::uniform(25.0), point(0.0, ascent));

    for glyph in glyphs {
        let (height, _) = glyph.pixel_bounding_box().map(|bb| {(bb.height(), bb.width())}).unwrap_or((0,0));
        let width = glyph.unpositioned().h_metrics().advance_width;

        x_offset += width as u32;

        let y_offset = (glyph.scale().y - (height as f32)) as u32;

        glyph.draw(|x,y,v|{self.framebuffer.draw_pixel(x + x_offset, y + y_offset, v)});

    }
       
    }
}
