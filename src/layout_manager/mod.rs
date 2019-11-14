extern crate glium;
use glium::glutin::{ElementState, ModifiersState, VirtualKeyCode};
use glium::{Display, Frame};

pub trait View {
    fn update(&mut self, display: &Display);
    fn draw(&mut self, display: &Display, target: &mut Frame);
    fn handle_input(
        &mut self,
        key_code: VirtualKeyCode,
        state: ElementState,
        modifiers: ModifiersState,
    );
    fn push_char(&mut self, c: char);
    fn pop_char(&mut self);
}

pub struct LayoutManager {
    pub views: Vec<Box<dyn View>>,
}

impl LayoutManager {
    pub fn update_views(&mut self, display: &Display) {
        for view in self.views.iter_mut() {
            view.update(display);
        }
    }

    pub fn draw(&mut self, display: &Display, target: &mut Frame) {
        for view in self.views.iter_mut() {
            view.draw(display, target);
        }
    }

    pub fn handle_input(
        &mut self,
        key_code: VirtualKeyCode,
        state: ElementState,
        modifiers: ModifiersState,
    ) {
        for view in self.views.iter_mut() {
            view.handle_input(key_code, state, modifiers);
        }
    }

    pub fn push_char(&mut self, c: char) {
        for view in self.views.iter_mut() {
            view.push_char(c);
        }
    }

    pub fn pop_char(&mut self) {
        for view in self.views.iter_mut() {
            view.pop_char();
        }
    }
}
