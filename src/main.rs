#[macro_use]
extern crate glium;
extern crate glium_glyph;

use glium::{glutin, Surface};

mod cmdline;
mod constants;
mod editor;
mod layout_manager;
mod ui;

use cmdline::CmdlineView;
use editor::EditorView;
use layout_manager::LayoutManager;

#[derive(Copy, Clone)]
pub struct Vertex {
    position: [f32; 2],
}
implement_vertex!(Vertex, position);

fn main() {
    let mut events_loop = glutin::EventsLoop::new();
    let wb = glutin::WindowBuilder::new()
        .with_dimensions(glium::glutin::dpi::LogicalSize::new(960.0, 600.0))
        .with_title("SnarkyEd");
    let cb = glutin::ContextBuilder::new();
    let display = glium::Display::new(wb, cb, &events_loop).unwrap();

    let mut layout = LayoutManager {
        views: vec![
            Box::new(EditorView::new("assets/source.txt", &display)),
            Box::new(CmdlineView::new(&display)),
        ],
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
                    // Broadcast input event
                    glutin::WindowEvent::ReceivedCharacter(c) => {
                        if c as u32 == 127 {
                            layout.pop_char();
                        } else {
                            layout.push_char(c);
                        }
                    }
                    // Other window events
                    glutin::WindowEvent::Resized(logical_size) => {
                        let hidpi_factor = display.gl_window().window().get_hidpi_factor();
                        display
                            .gl_window()
                            .resize(logical_size.to_physical(hidpi_factor));
                    }
                    glutin::WindowEvent::CloseRequested => closed = true,
                    glutin::WindowEvent::KeyboardInput {
                        input:
                            glutin::KeyboardInput {
                                virtual_keycode: Some(virtual_code),
                                state,
                                modifiers,
                                ..
                            },
                        ..
                    } => layout.handle_input(virtual_code, state, modifiers),
                    _ => (),
                },
                _ => (),
            }
        });
    }
}
