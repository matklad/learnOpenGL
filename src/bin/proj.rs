#![deny(warnings)]

#[macro_use]
extern crate log;
#[macro_use]
extern crate glium;
extern crate env_logger;
extern crate lights;

use std::io::Write;

use env_logger::LogBuilder;
use glium::{index, Surface, VertexBuffer, DrawError};
use glium::DrawParameters;
use glium::vertex::BufferCreationError;
use glium::backend::Facade;

use lights::{App, Api, Painter, Vertex};

fn init_log() {
    LogBuilder::new()
        .parse("info")
        .init()
        .expect("Failed to init the logger");
}


fn main() {
    if let Err(e) = run() {
        writeln!(std::io::stderr(), "{}\n=(", e).unwrap();
        writeln!(std::io::stderr(), "\nGuru meditation {:#?}", e).unwrap();
    }
}

fn run() -> Result<(), Box<std::error::Error>> {
    init_log();
    try!(App::<Matisse>::run());
    Ok(())
}

struct Matisse {
    vertex_buffer: VertexBuffer<Vertex>,
}

impl Painter for Matisse {
    #[cfg_attr(rustfmt, rustfmt_skip)]
    fn new<F: Facade>(facade: &F) -> Result<Matisse, BufferCreationError> {
        let shape = Vertex::many(vec![
             0.5,  0.5, 0.0,  // Top Right
             0.5, -0.5, 0.0,  // Bottom Right
            -0.5,  0.5, 0.0,  // Top Left

             0.5, -0.5, 0.0,  // Bottom Right
            -0.5, -0.5, 0.0,  // Bottom Left
            -0.5,  0.5, 0.0   // Top Left
        ]);

        let vertex_buffer = try!(VertexBuffer::new(facade, &shape));
        Ok(Matisse { vertex_buffer: vertex_buffer })
    }

    fn vertex_shader(&self) -> &str {
        include_str!("./shaders/vertex.glsl")
    }

    fn fragment_shader(&self) -> &'static str {
        include_str!("./shaders/fragment.glsl")

    }

    fn draw<S: Surface>(&self, api: &mut Api<S>) -> std::result::Result<(), DrawError> {

        let indices = index::NoIndices(index::PrimitiveType::TrianglesList);
        let params = DrawParameters { ..Default::default() };

        let uniforms = uniform!{
             ucolor: [0.0, api.time.sin() / 2.0 + 0.5, 0.0, 1.0]
        };
        try!(api.surface.draw(&self.vertex_buffer,
                              &indices,
                              api.program,
                              &uniforms,
                              &params));
        Ok(())
    }
}
