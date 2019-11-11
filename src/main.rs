extern crate glium;
extern crate glium_glyph;
use glium::{glutin, Surface};

mod layout_manager;
mod editor;

use layout_manager::{ LayoutManager };
use editor::EditorView;

fn main() {
    let mut events_loop = glutin::EventsLoop::new();
    let wb = glutin::WindowBuilder::new()
        .with_dimensions(glium::glutin::dpi::LogicalSize::new(960.0, 600.0))
        .with_title("SnarkyEd");
    let cb = glutin::ContextBuilder::new();
    let display = glium::Display::new(wb, cb, &events_loop).unwrap();

    let mut layout = LayoutManager {
        views: vec![
            Box::new(EditorView::new("assets/source.txt", &display))
        ]
    };

    let mut closed = false;

    while !closed {
        layout.update_views(&display);

        let mut target = display.draw();
        target.clear_color_srgb(0.14, 0.18, 0.21, 1.0);
        layout.draw(&display, &mut target);
        target.finish().unwrap();

        events_loop.poll_events(|ev| {
            match ev {
                glutin::Event::WindowEvent { event, .. } => match event {
                    glutin::WindowEvent::CloseRequested => closed = true,
                    glutin::WindowEvent::KeyboardInput {
                        input: glutin::KeyboardInput {
                            virtual_keycode: Some(virtual_code),
                            state,
                            ..
                        },
                        ..
                    } => layout.handle_input(virtual_code, state),
                    _ => (),
                },
                _ => (),
            }
        });
    }
}
