use crate::Vertex;
use glium::{Display, Frame, Surface};

pub struct Panel {
    pub position: [f32; 2],
    pub size: [f32; 2],
    pub color: [f32; 4],
    pub vb: glium::VertexBuffer<Vertex>,
    pub ib: glium::IndexBuffer<u16>,
    pub pg: glium::Program,
}

impl Panel {
    pub fn new(display: &Display, position: [f32; 2], size: [f32; 2], color: [f32; 4]) -> Panel {
        let (screen_width, screen_height) = display.get_framebuffer_dimensions();
        let hidpi_factor = display.gl_window().window().get_hidpi_factor() as f32;
        let (origin_width, origin_height) = (
            screen_width as f32 / hidpi_factor / 2.0,
            screen_height as f32 / hidpi_factor / 2.0,
        );
        let [x, y] = position;
        let [width, height] = size;
        let v_top_left = Vertex {
            position: [
                (x - origin_width) / origin_width,
                (origin_height - y) / origin_height,
            ],
        };
        let v_top_right = Vertex {
            position: [
                (x + width - origin_width) / origin_width,
                (origin_height - y) / origin_height,
            ],
        };
        let v_bottom_right = Vertex {
            position: [
                (x + width - origin_width) / origin_width,
                (origin_height - (y + height)) / origin_height,
            ],
        };
        let v_bottom_left = Vertex {
            position: [
                (x - origin_width) / origin_width,
                (origin_height - (y + height)) / origin_height,
            ],
        };
        let shape = vec![v_top_left, v_top_right, v_bottom_right, v_bottom_left];
        let indices: [u16; 6] = [0, 1, 2, 2, 0, 3];
        let vb = glium::VertexBuffer::new(display, &shape).unwrap();
        let ib = glium::IndexBuffer::new(
            display,
            glium::index::PrimitiveType::TrianglesList,
            &indices,
        )
        .unwrap();
        let vertex_shader_src = r#"
        #version 140
        in vec2 position;
        void main() {
            gl_Position = vec4(position, 0.0, 1.0);
        }
    "#;

        let fragment_shader_src = &format!(
            r#"
        #version 140
        out vec4 color;
        void main() {{
            color = vec4({}, {}, {}, {});
        }}
    "#,
            color[0], color[1], color[2], color[3]
        );
        let pg = glium::Program::from_source(display, vertex_shader_src, fragment_shader_src, None)
            .unwrap();
        Panel {
            position: position,
            size: size,
            color: color,
            vb: vb,
            ib: ib,
            pg: pg,
        }
    }

    pub fn draw(&mut self, target: &mut Frame) {
        target
            .draw(
                &self.vb,
                &self.ib,
                &self.pg,
                &glium::uniforms::EmptyUniforms,
                &Default::default(),
            )
            .unwrap();
    }
}
