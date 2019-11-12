use glium::glutin::{VirtualKeyCode, ElementState, ModifiersState};
use glium::{Display, Frame};
use glium_glyph::glyph_brush::{rusttype::Font, Section, GlyphCruncher};
use glium_glyph::GlyphBrush;
use glium_glyph::glyph_brush::rusttype::Rect;

use crate::layout_manager::View;
use crate::constants::{NO_MODIFIERS, CTRL_HOLD, SHIFT_HOLD, CMD_SHIFT_HOLD};

const BORDER_TEXT: &str = r#"┌──────────────────────────────────────────────────┐
│                                                  |
└──────────────────────────────────────────────────┘"#;

pub struct CmdlineView<'a, 'b> {
    glyph_brush: GlyphBrush<'a, 'b>,
    padding: f32,
    font_size: f32,
    letter_size: Rect<f32>,
    command_text: String,
    visible: bool
}

impl<'a, 'b> CmdlineView<'a, 'b> {
    pub fn new(display: &Display) -> CmdlineView<'a, 'b> {
        let font_regular: &[u8] = include_bytes!("../../assets/haskplex.ttf");
        let fonts = vec![Font::from_bytes(font_regular).unwrap()];
        let font_size = 16.0 * 2.0;
        let mut gb = GlyphBrush::new(display, fonts);
        let letter_size = gb.glyph_bounds(Section {
            text: "0",
            scale: glyph_brush::rusttype::Scale::uniform(font_size),
            ..Section::default()
        }).unwrap();
        CmdlineView {
          glyph_brush: gb,
          padding: 30.0,
          font_size: font_size,
          letter_size: letter_size,
          command_text: "".to_owned(),
          visible: false
        }
    }
}

impl<'a, 'b> View for CmdlineView<'a, 'b> {
    fn update(&mut self, display: &Display) {
        let screen_dims = display.get_framebuffer_dimensions();
        self.glyph_brush.queue(Section {
            text: BORDER_TEXT,
            bounds: (screen_dims.0 as f32 - self.padding, screen_dims.1 as f32),
            screen_position: (self.padding / 2.0 + screen_dims.0 as f32 / 4.0, screen_dims.1 as f32 / 2.0 - self.font_size * 2.0),
            scale: glyph_brush::rusttype::Scale::uniform(self.font_size * 1.2),
            color: [0.58, 0.89, 0.02, 1.0],
            ..Section::default()
        });

        self.glyph_brush.queue(Section {
            text: &self.command_text,
            bounds: (screen_dims.0 as f32 - self.padding, screen_dims.1 as f32),
            screen_position: (self.padding / 2.0 + screen_dims.0 as f32 / 3.8, screen_dims.1 as f32 / 2.0 - self.font_size),
            scale: glyph_brush::rusttype::Scale::uniform(self.font_size * 1.5),
            color: [0.58, 0.89, 0.02, 1.0],
            ..Section::default()
        });
    }

    fn draw(&mut self, display: &Display, target: &mut Frame) {
        if self.visible {
            self.glyph_brush.draw_queued(display, target);
        }
    }

    fn handle_input(&mut self, key_code: VirtualKeyCode, state: ElementState, modifiers: ModifiersState) {
        match (key_code, state, modifiers) {
            (VirtualKeyCode::P, ElementState::Pressed, CMD_SHIFT_HOLD) => {
              self.visible = true;
            },
            (VirtualKeyCode::Escape, ElementState::Pressed, NO_MODIFIERS) => {
              self.visible = false;
            },
            _ => ()
        }
        // TODO: Implement a proper key to text input handler
        if self.visible && state == ElementState::Pressed {
            self.command_text = format!("{}{:?}", self.command_text, key_code);
        }
    }
}
