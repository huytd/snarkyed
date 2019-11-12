mod buffer;
mod cursor;

use glium::glutin::{VirtualKeyCode, ElementState, ModifiersState};
use glium::{Display, Frame};
use glium_glyph::glyph_brush::{rusttype::Font, Section, GlyphCruncher};
use glium_glyph::GlyphBrush;
use glium_glyph::glyph_brush::rusttype::Rect;

use buffer::Buffer;
use cursor::Cursor;

use crate::layout_manager::View;
use crate::constants::{ NO_MODIFIERS, CTRL_HOLD, SHIFT_HOLD, BASE_FONT_SIZE };

pub struct EditorView<'a, 'b> {
    pub buffer: Buffer,
    cursor: Cursor,
    glyph_brush: GlyphBrush<'a, 'b>,
    padding: f32,
    font_size: f32,
    offset_y: usize,
    viewport_rows: usize,
    letter_size: Rect<f32>,
    last_column: i32,
}

impl<'a, 'b> EditorView<'a, 'b> {
    pub fn new(file: &str, display: &Display) -> EditorView<'a, 'b> {
        let font_regular: &[u8] = include_bytes!("../../assets/haskplex.ttf");
        let fonts = vec![Font::from_bytes(font_regular).unwrap()];
        let hidpi_factor = display.gl_window().window().get_hidpi_factor() as f32;
        let font_size = BASE_FONT_SIZE * hidpi_factor;
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
            viewport_rows: 0,
            letter_size: letter_size,
            last_column: -1
        }
    }

    fn scroll_down(&mut self, step: usize) {
        if self.offset_y + self.cursor.row as usize + step < self.buffer.get_lines_count() {
            self.offset_y += step;
        }
    }

    fn scroll_up(&mut self, step: usize) {
        if self.offset_y as i32 - step as i32 >= 0 {
            self.offset_y -= step;
        }
    }

    fn move_cursor_down(&mut self) {
        if self.cursor.row as usize + 1 > self.viewport_rows - 1 {
            self.scroll_down(1);
        } else {
            self.cursor.row += 1;
        }
        if self.last_column != -1 {
            self.cursor.col = self.last_column;
        }
        self.last_column = -1;
        self.move_to_eol(true);
    }

    fn move_cursor_up(&mut self) {
        if self.cursor.row > 0 {
            self.cursor.row -= 1;
        } else {
            self.scroll_up(1);
        }
        if self.last_column != -1 {
            self.cursor.col = self.last_column;
        }
        self.last_column = -1;
        self.move_to_eol(true);
    }

    fn move_cursor_left(&mut self) {
        if self.cursor.col > 0 {
            self.cursor.col -= 1;
        }
        self.last_column = -1;
    }

    fn move_cursor_right(&mut self) {
        let current_line = self.buffer.get_line_at(self.offset_y + self.cursor.row as usize);
        if current_line.len() - 1 > self.cursor.col as usize {
            self.last_column = self.cursor.col;
            self.cursor.col += 1;
        }
        self.last_column = -1;
    }

    fn move_to_bol(&mut self) {
        self.cursor.col = 0;
        self.last_column = -1;
    }

    fn move_to_eol(&mut self, try_first: bool) {
        let current_line = self.buffer.get_line_at(self.offset_y + self.cursor.row as usize);
        if try_first {
            if self.cursor.col as usize > current_line.len() {
                self.last_column = self.cursor.col;
                self.cursor.col = current_line.len() as i32 - 1;
            }
        } else {
            self.cursor.col = current_line.len() as i32 - 1;
            self.last_column = -1;
        }
    }
}

impl<'a, 'b> View for EditorView<'a, 'b> {
    fn update(&mut self, display: &Display) {
        let hidpi_factor = display.gl_window().window().get_hidpi_factor() as f32;
        self.font_size = BASE_FONT_SIZE * hidpi_factor;

        let screen_dims = display.get_framebuffer_dimensions();
        self.viewport_rows = (screen_dims.1 as f32 / self.font_size) as usize;

        let content_to_draw = self.buffer.get_lines(self.offset_y, self.offset_y + self.viewport_rows);

        self.glyph_brush.queue(Section {
            text: &content_to_draw,
            bounds: (screen_dims.0 as f32 - self.padding, screen_dims.1 as f32),
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
        });
    }

    fn draw(&mut self, display: &Display, target: &mut Frame) {
        self.glyph_brush.draw_queued(display, target);
    }

    fn handle_input(&mut self, key_code: VirtualKeyCode, state: ElementState, modifiers: ModifiersState) {
        match (key_code, state, modifiers) {
            (VirtualKeyCode::J, ElementState::Pressed, NO_MODIFIERS) => {
                self.move_cursor_down();
            },
            (VirtualKeyCode::K, ElementState::Pressed, NO_MODIFIERS) => {
                self.move_cursor_up() ;
            },
            (VirtualKeyCode::H, ElementState::Pressed, NO_MODIFIERS) => {
                self.move_cursor_left();
            },
            (VirtualKeyCode::L, ElementState::Pressed, NO_MODIFIERS) => {
                self.move_cursor_right();
            },
            (VirtualKeyCode::Key0, ElementState::Pressed, NO_MODIFIERS) => {
                self.move_to_bol();
            },
            (VirtualKeyCode::Key4, ElementState::Pressed, SHIFT_HOLD) => {
                self.move_to_eol(false);
            },
            (VirtualKeyCode::J, ElementState::Pressed, CTRL_HOLD) => {
                self.scroll_down(10);
            },
            (VirtualKeyCode::K, ElementState::Pressed, CTRL_HOLD) => {
                self.scroll_up(10);
            },
            _ => ()
        }
    }
}
