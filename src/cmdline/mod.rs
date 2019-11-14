use glium::glutin::{ElementState, ModifiersState, VirtualKeyCode};
use glium::{Display, Frame};
use glium_glyph::glyph_brush::rusttype::Rect;
use glium_glyph::glyph_brush::{rusttype::Font, GlyphCruncher, Section};
use glium_glyph::GlyphBrush;

use crate::constants::{BASE_FONT_SIZE, CMD_SHIFT_HOLD, NO_MODIFIERS};
use crate::layout_manager::View;
use crate::ui::panel::Panel;
use crate::ui::color;

pub struct CmdlineView<'a, 'b> {
    glyph_brush: GlyphBrush<'a, 'b>,
    padding: f32,
    font_size: f32,
    letter_size: Rect<f32>,
    command_text: String,
    visible: bool,
    background: Panel,
}

impl<'a, 'b> CmdlineView<'a, 'b> {
    pub fn new(display: &Display) -> CmdlineView<'a, 'b> {
        let font_regular: &[u8] = include_bytes!("../../assets/haskplex.ttf");
        let fonts = vec![Font::from_bytes(font_regular).unwrap()];
        let mut gb = GlyphBrush::new(display, fonts);
        let hidpi_factor = display.gl_window().window().get_hidpi_factor() as f32;
        let font_size = BASE_FONT_SIZE * hidpi_factor;
        let letter_size = gb
            .glyph_bounds(Section {
                text: "0",
                scale: glyph_brush::rusttype::Scale::uniform(font_size),
                ..Section::default()
            })
            .unwrap();

        let screen_dims = display.get_framebuffer_dimensions();
        let bg_w = 600.0; let bg_h = 30.0;
        let bg_x = screen_dims.0 as f32 / hidpi_factor / 2.0 - bg_w / 2.0;
        let bg_y = screen_dims.1 as f32 / hidpi_factor / 2.0 - bg_h / 2.0;

        CmdlineView {
            glyph_brush: gb,
            padding: 30.0,
            font_size: font_size,
            letter_size: letter_size,
            command_text: "Hello".to_owned(),
            visible: false,
            background: Panel::new(&display, [bg_x, bg_y], [bg_w, bg_h], color::hex("#4A148C").as_slice()),
        }
    }
}

impl<'a, 'b> View for CmdlineView<'a, 'b> {
    fn update(&mut self, display: &Display) {
        let hidpi_factor = display.gl_window().window().get_hidpi_factor() as f32;
        self.font_size = BASE_FONT_SIZE * hidpi_factor;
        let screen_dims = display.get_framebuffer_dimensions();
        let text_x = screen_dims.0 as f32 / hidpi_factor / 2.0 - (300.0 / hidpi_factor) + self.padding * hidpi_factor;
        let text_y = screen_dims.1 as f32 / 2.0 - self.font_size / 2.0;

        self.glyph_brush.queue(Section {
            text: &self.command_text,
            bounds: (screen_dims.0 as f32 - self.padding, screen_dims.1 as f32),
            screen_position: (text_x, text_y),
            scale: glyph_brush::rusttype::Scale::uniform(self.font_size),
            color: color::hex("#FF5F56").as_slice(),
            ..Section::default()
        });
    }

    fn draw(&mut self, display: &Display, target: &mut Frame) {
        if self.visible {
            self.background.draw(target);
            self.glyph_brush.draw_queued(display, target);
        }
    }

    fn handle_input(
        &mut self,
        key_code: VirtualKeyCode,
        state: ElementState,
        modifiers: ModifiersState,
    ) {
        match (key_code, state, modifiers) {
            (VirtualKeyCode::P, ElementState::Pressed, CMD_SHIFT_HOLD) => {
                self.visible = true;
                self.command_text = String::new();
            }
            (VirtualKeyCode::Escape, ElementState::Pressed, NO_MODIFIERS) => {
                self.visible = false;
            }
            _ => (),
        }
    }

    fn push_char(&mut self, c: char) {
        if self.visible {
            self.command_text.push(c);
        }
    }

    fn pop_char(&mut self) {
        if self.visible {
            self.command_text.pop();
        }
    }
}
