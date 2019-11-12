extern crate glium;
extern crate glium_glyph;

use glium::{glutin, Surface};

mod constants;
mod layout_manager;
mod editor;
mod cmdline;

use layout_manager::{ LayoutManager };
use editor::EditorView;
use cmdline::CmdlineView;

fn main() {
    let mut events_loop = glutin::EventsLoop::new();
    let wb = glutin::WindowBuilder::new()
        .with_dimensions(glium::glutin::dpi::LogicalSize::new(960.0, 600.0))
        .with_title("SnarkyEd");
    let cb = glutin::ContextBuilder::new();
    let display = glium::Display::new(wb, cb, &events_loop).unwrap();

    let mut typewriter = String::new();

    let mut layout = LayoutManager {
        views: vec![
            Box::new(EditorView::new("assets/source.txt", &display)),
            Box::new(CmdlineView::new(&display))
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
                    // Store input content to a global buffer
                    glutin::WindowEvent::ReceivedCharacter('\r') => {
                        typewriter.clear();
                        layout.broadcast_typing(&typewriter);
                    },
                    glutin::WindowEvent::ReceivedCharacter(c) => {
                        if c as u32 == 127 {
                            typewriter.pop();
                        } else {
                            typewriter.push(c);
                        }
                        layout.broadcast_typing(&typewriter);
                    },
                    // Other window events
                    glutin::WindowEvent::Resized(logical_size) => {
                        let hidpi_factor = display.gl_window().window().get_hidpi_factor();
                        display.gl_window()
                            .resize(logical_size.to_physical(hidpi_factor));
                    },
                    glutin::WindowEvent::CloseRequested => closed = true,
                    glutin::WindowEvent::KeyboardInput {
                        input: glutin::KeyboardInput {
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
