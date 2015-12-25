#![deny(warnings)]

#[macro_use]
extern crate log;
extern crate env_logger;
extern crate lights;
extern crate glium;

use std::io::Write;

use env_logger::LogBuilder;
use glium::{index, Surface, Program, VertexBuffer, DrawError};
use glium::vertex::BufferCreationError;
use glium::backend::Facade;

use lights::{App, Painter, Vertex};

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
    fn new<F: Facade>(facade: &F) -> Result<Matisse, BufferCreationError> {
        let vertex1 = Vertex::new(-0.5, -0.5);
        let vertex2 = Vertex::new(0.0, 0.5);
        let vertex3 = Vertex::new(0.5, -0.25);
        let shape = vec![vertex1, vertex2, vertex3];

        let vertex_buffer = try!(VertexBuffer::new(facade, &shape));
        Ok(Matisse { vertex_buffer: vertex_buffer })
    }

    fn vertex_shader(&self) -> &str {
        include_str!("./shaders/vertex.glsl")
    }

    fn fragment_shader(&self) -> &'static str {
        include_str!("./shaders/fragment.glsl")
    }

    fn draw<S: Surface>(&self,
                        target: &mut S,
                        program: &Program)
                        -> std::result::Result<(), DrawError> {

        let indices = index::NoIndices(index::PrimitiveType::TrianglesList);
        try!(target.draw(&self.vertex_buffer,
                         &indices,
                         &program,
                         &glium::uniforms::EmptyUniforms,
                         &Default::default()));
        Ok(())
    }
}
