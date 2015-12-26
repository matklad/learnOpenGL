#![deny(warnings)]

#[macro_use]
extern crate log;
#[macro_use]
extern crate glium;
extern crate env_logger;
extern crate lights;

use std::error::Error;
use std::io::Write;

use env_logger::LogBuilder;
use glium::backend::Facade;
use glium::DrawParameters;
use glium::texture::Texture2d;
use glium::index::{IndexBuffer, PrimitiveType};
use glium::{Surface, VertexBuffer, DrawError};

use lights::{App, Api, Painter, load_texture_jpeg};

mod vertex;

use vertex::Vertex;

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
    index_buffer: IndexBuffer<u16>,
    texture: Texture2d,
}

impl Painter for Matisse {
    #[cfg_attr(rustfmt, rustfmt_skip)]
    fn new<F: Facade>(facade: &F) -> Result<Matisse, Box<Error>> {
        let shape = Vertex::many(vec![
    // Positions          // Colors           // Texture Coords
             0.5,  0.5, 0.0,   1.0, 0.0, 0.0,   1.0, 1.0,   // Top Right
             0.5, -0.5, 0.0,   0.0, 1.0, 0.0,   1.0, 0.0,   // Bottom Right
            -0.5, -0.5, 0.0,   0.0, 0.0, 1.0,   0.0, 0.0,   // Bottom Left
            -0.5,  0.5, 0.0,   1.0, 1.0, 0.0,   0.0, 1.0    // Top Left
        ]);
        let indices = vec![
            0, 1, 3,
            1, 2, 3
        ];

        let vertex_buffer = try!(VertexBuffer::new(facade, &shape));
        let index_buffer = IndexBuffer::new(facade, PrimitiveType::TrianglesList, &indices)
            .expect("failed to crate an index buffer");

        let image = load_texture_jpeg(include_bytes!("./textures/container.jpg"));
        let texture = glium::texture::Texture2d::new(facade, image).unwrap();

        Ok(Matisse {
            vertex_buffer: vertex_buffer,
            index_buffer: index_buffer,
            texture: texture,
        })
    }

    fn vertex_shader(&self) -> &str {
        include_str!("./shaders/vertex.glsl")
    }

    fn fragment_shader(&self) -> &'static str {
        include_str!("./shaders/fragment.glsl")
    }

    fn draw<S: Surface>(&self, api: &mut Api<S>) -> std::result::Result<(), DrawError> {

        let params = DrawParameters { ..Default::default() };

        let uniforms = uniform! {
            tex: &self.texture
        };
        try!(api.surface.draw(&self.vertex_buffer,
                              &self.index_buffer,
                              api.program,
                              &uniforms,
                              &params));
        Ok(())
    }
}
