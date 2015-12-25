use glium::{self, index, DisplayBuild, Surface, VertexBuffer};
use glium::backend::glutin_backend::GlutinFacade;
use glium::glutin::{WindowBuilder, GlProfile, Event, VirtualKeyCode};


use {AppError, Result};
use vertex::Vertex;

pub struct App {
    facade: GlutinFacade,
}

impl App {
    pub fn run() -> Result<()> {
        info!("Starting the application");
        let app = App { facade: try!(build_display()) };
        try!(app.main_loop());
        info!("The application has stopped");
        Ok(())
    }

    fn main_loop(&self) -> Result<()> {
        let vertex1 = Vertex::new(-0.5, -0.5);
        let vertex2 = Vertex::new(0.0, 0.5);
        let vertex3 = Vertex::new(0.5, -0.25);
        let shape = vec![vertex1, vertex2, vertex3];

        let vertex_buffer = VertexBuffer::new(&self.facade, &shape)
                                .expect("Failed to create a vertex buffer");
        let indices = index::NoIndices(index::PrimitiveType::TrianglesList);
        let fragment_shader_src = r#"
            #version 140

            out vec4 color;

            void main() {
                color = vec4(1.0, 0.0, 0.0, 1.0);
            }
        "#;

        let vertex_shader_src = r#"
        #version 140

        in vec2 position;

        void main() {
            gl_Position = vec4(position, 0.0, 1.0);
        }
        "#;

        let program = glium::Program::from_source(&self.facade,
                                                  vertex_shader_src,
                                                  fragment_shader_src,
                                                  None)
                          .expect("Failed to crate a shade program");
        loop {
            let mut target = self.facade.draw();
            target.clear_color(0.2, 0.35, 0.35, 1.0);
            target.draw(&vertex_buffer,
                        &indices,
                        &program,
                        &glium::uniforms::EmptyUniforms,
                        &Default::default())
                  .unwrap();

            try!(target.finish());

            for ev in self.facade.poll_events() {
                match ev {
                    Event::Closed | Event::KeyboardInput(_, _, Some(VirtualKeyCode::Escape)) => {
                        return Ok(())
                    }
                    _ => {}
                }
            }
        }
    }
}

fn build_display() -> Result<GlutinFacade> {
    WindowBuilder::new()
        .with_dimensions(800, 600)
        .with_depth_buffer(24)
        .with_gl_profile(GlProfile::Core)
        .build_glium()
        .map_err(|e| AppError::InitializationError { cause: Box::new(e) })
}
