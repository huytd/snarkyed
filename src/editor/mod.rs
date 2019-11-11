mod buffer;

use buffer::Buffer;
use glium::glutin::{VirtualKeyCode, ElementState};
use glium::{Display, Frame};

use glium_glyph::glyph_brush::{rusttype::Font, Section};
use glium_glyph::GlyphBrush;

use crate::layout_manager::View;

pub struct EditorView<'a, 'b> {
    pub buffer: Buffer,
    glyph_brush: GlyphBrush<'a, 'b>,
    padding: f32,
    font_size: f32,
    offset_y: usize
}

impl<'a, 'b> EditorView<'a, 'b> {
    pub fn new(file: &str, display: &Display) -> EditorView<'a, 'b> {
        let font_regular: &[u8] = include_bytes!("../../assets/haskplex.ttf");
        let font_italic: &[u8] = include_bytes!("../../assets/haskplex-italic.ttf");
        let fonts = vec![Font::from_bytes(font_regular).unwrap(), Font::from_bytes(font_italic).unwrap()];
        EditorView {
            buffer: Buffer::new(file),
            glyph_brush: GlyphBrush::new(display, fonts),
            padding: 30.0,
            font_size: 16.0 * 2.0,
            offset_y: 0
        }
    }
}

impl<'a, 'b> View for EditorView<'a, 'b> {
    fn update(&mut self, display: &Display) {
        let screen_dims = display.get_framebuffer_dimensions();
        let viewport_lines = (screen_dims.1 as f32 / self.font_size) as usize;
        let content_to_draw = self.buffer.get_lines(self.offset_y, self.offset_y + viewport_lines);
        self.glyph_brush.queue(Section {
            text: &content_to_draw,
            bounds: (screen_dims.0 as f32 - self.padding, screen_dims.1 as f32 - self.padding),
            screen_position: ((self.padding / 2.0), (self.padding / 2.0)),
            scale: glyph_brush::rusttype::Scale::uniform(self.font_size),
            color: [1.0, 1.0, 1.0, 1.0],
            ..Section::default()
        });
    }

    fn draw(&mut self, display: &Display, target: &mut Frame) {
        self.glyph_brush.draw_queued(display, target);
    }

    fn handle_input(&mut self, key_code: VirtualKeyCode, state: ElementState) {
        match (key_code, state) {
            (VirtualKeyCode::J, ElementState::Pressed) => {
                if self.buffer.get_lines_count() > self.offset_y {
                    self.offset_y += 1;
                }
            },
            (VirtualKeyCode::K, ElementState::Pressed) => {
                if self.offset_y > 0 {
                    self.offset_y -= 1;
                }
            },
            _ => ()
        }
    }
}
