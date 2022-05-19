use rusttype::{point, Font, Scale};

use crate::framebuffer::EFIFrameBuffer;

pub struct FBTerm<'a> {
    framebuffer: EFIFrameBuffer<'a>,
    term_font: Font<'a>,
    current_column: usize,
    current_row: usize,

    character_height: usize,
    character_width: usize,

    ascent: f32
}

impl<'a> FBTerm<'a> {
    pub fn new(framebuffer: EFIFrameBuffer<'a>, font: Font<'a>) -> Self {
        let tc = font.glyph(' ').scaled(Scale::uniform(25.0));
        let character_width = tc.h_metrics().advance_width as usize;
        let character_height  = tc.scale().y as usize;
        let ascent = font.v_metrics(Scale::uniform(25.0)).ascent;
        FBTerm {
            framebuffer,
            term_font: font,
            current_column: 0,
            current_row: 0,

            character_height,
            character_width,

            ascent,
        }
    }

    pub fn print_at(&mut self, x: usize, y: usize, s: &str)
    {

        for (index, c) in s.chars().enumerate() {
            self.print_char_at(x + index, y, c);
        }
    }

    pub fn print_char_at(&mut self, x: usize, y: usize, c: char) {
        let glyph = self.term_font.glyph(c).scaled(Scale::uniform(25.0)).positioned(point(0.0, self.ascent));

        let height = glyph
            .pixel_bounding_box()
            .map(|bb| bb.height())
            .unwrap_or(0);

        let x_offset = self.character_width * x;
        let y_offset = self.ascent - height as f32;
        let y_offset = y_offset as usize + (self.character_height * y);

        glyph.draw(|x,y,v| self.framebuffer.draw_pixel(x as usize + x_offset, y as usize + y_offset, v));
    }

    pub fn print_ref_str(&mut self, s: &str) {
        for c in s.chars() {
            if self.term_font.glyph(c).id() != self.term_font.glyph('\0').id() {
                self.print_char_at(self.current_column, self.current_row, c);
            }
            self.update_location(c);
        }
    }

    pub fn print(&mut self, s: impl AsRef<str>) {
        self.print_ref_str(s.as_ref());
    }

    fn update_location(&mut self, character: char) {
        match character {
            '\n' => self.current_row += 1,
            '\r' => self.current_column = 0,
            _ => self.current_column += 1,
        } 
    }
}
