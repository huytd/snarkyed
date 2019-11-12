extern crate glium;
use glium::glutin::{VirtualKeyCode, ElementState, ModifiersState};
use glium::{Display, Frame};

pub trait View {
  fn update(&mut self, display: &Display);
  fn draw(&mut self, display: &Display, target: &mut Frame);
  fn handle_input(&mut self, key_code: VirtualKeyCode, state: ElementState, modifiers: ModifiersState);
  fn typewriting(&mut self, content: &str);
}

pub struct LayoutManager {
  pub views: Vec<Box<dyn View>>,
}

#[allow(dead_code)]
impl LayoutManager {
  pub fn add_view(&mut self, view: Box<dyn View>) {
    self.views.push(view)
  }

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

  pub fn handle_input(&mut self, key_code: VirtualKeyCode, state: ElementState, modifiers: ModifiersState) {
    for view in self.views.iter_mut() {
      view.handle_input(key_code, state, modifiers);
    }
  }

  pub fn broadcast_typing(&mut self, content: &str) {
    for view in self.views.iter_mut() {
      view.typewriting(&content);
    }
  }
}
