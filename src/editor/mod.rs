mod buffer;
mod cursor;

use glium::glutin::{VirtualKeyCode, ElementState};
use glium::{Display, Frame};
use glium_glyph::glyph_brush::{rusttype::Font, Section, GlyphCruncher};
use glium_glyph::GlyphBrush;
use glium_glyph::glyph_brush::rusttype::Rect;

use buffer::Buffer;
use cursor::Cursor;

use crate::layout_manager::View;

pub struct EditorView<'a, 'b> {
    pub buffer: Buffer,
    cursor: Cursor,
    glyph_brush: GlyphBrush<'a, 'b>,
    padding: f32,
    font_size: f32,
    offset_y: usize,
    letter_size: Rect<f32>
}

impl<'a, 'b> EditorView<'a, 'b> {
    pub fn new(file: &str, display: &Display) -> EditorView<'a, 'b> {
        let font_regular: &[u8] = include_bytes!("../../assets/haskplex.ttf");
        let fonts = vec![Font::from_bytes(font_regular).unwrap()];
        let font_size = 16.0 * 2.0;
        let mut gb = GlyphBrush::new(display, fonts);
        let letter_size = gb.glyph_bounds(Section {
            text: "0",
            scale: glyph_brush::rusttype::Scale::uniform(font_size),
            ..Section::default()
        }).unwrap();

        EditorView {
            buffer: Buffer::new(file),
            cursor: Cursor::new(),
            glyph_brush: gb,
            padding: 30.0,
            font_size: font_size,
            offset_y: 0,
            letter_size: letter_size
        }
    }
}

impl<'a, 'b> View for EditorView<'a, 'b> {
    fn update(&mut self, display: &Display) {
        let screen_dims = display.get_framebuffer_dimensions();
        let viewport_rows = (screen_dims.1 as f32 / self.font_size) as usize;
        let content_to_draw = self.buffer.get_lines(self.offset_y, self.offset_y + viewport_rows);

        self.glyph_brush.queue(Section {
            text: &content_to_draw,
            bounds: (screen_dims.0 as f32 - self.padding, screen_dims.1 as f32 - self.padding),
            screen_position: ((self.padding / 2.0), (self.padding / 2.0)),
            scale: glyph_brush::rusttype::Scale::uniform(self.font_size),
            color: [0.92, 0.99, 0.99, 1.0],
            ..Section::default()
        });

        self.glyph_brush.queue(Section {
            text: "█",
            bounds: (screen_dims.0 as f32 - self.padding, screen_dims.1 as f32 - self.padding),
            screen_position: ((self.padding / 2.0) + (self.letter_size.width() * self.cursor.col as f32), (self.padding / 2.0) + (self.letter_size.height() * self.cursor.row as f32)),
            scale: glyph_brush::rusttype::Scale::uniform(self.font_size),
            color: [0.92, 0.99, 0.99, 0.4],
            ..Section::default()
        })
    }

    fn draw(&mut self, display: &Display, target: &mut Frame) {
        self.glyph_brush.draw_queued(display, target);
    }

    fn handle_input(&mut self, key_code: VirtualKeyCode, state: ElementState) {
        match (key_code, state) {
            (VirtualKeyCode::J, ElementState::Pressed) => {
                self.cursor.row += 1;
            },
            (VirtualKeyCode::K, ElementState::Pressed) => {
                if self.cursor.row > 0 {
                    self.cursor.row -= 1;
                }
            },
            (VirtualKeyCode::H, ElementState::Pressed) => {
                if self.cursor.col > 0 {
                    self.cursor.col -= 1;
                }
            },
            (VirtualKeyCode::L, ElementState::Pressed) => {
                self.cursor.col += 1;
            },
            _ => ()
        }
    }
}
