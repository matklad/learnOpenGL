#[deny(warnings)]
#[macro_use]
extern crate log;
#[macro_use]
extern crate glium;
extern crate env_logger;
extern crate lights;

use std::error::Error;
use std::io::prelude::*;

use env_logger::LogBuilder;
use glium::backend::Facade;
use glium::DrawParameters;
use glium::texture::Texture2d;
use glium::index::{IndexBuffer, PrimitiveType};
use glium::{Surface, VertexBuffer, DrawError};

use lights::{App, Api, Painter, load_texture_jpeg, load_texture_png, slurp, slurp_bytes};
use lights::math::*;

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
    vertex_shader: String,
    fragment_shader: String,
    vertex_buffer: VertexBuffer<Vertex>,
    index_buffer: IndexBuffer<u16>,
    texture1: Texture2d,
    texture2: Texture2d,
}

#[cfg_attr(rustfmt, rustfmt_skip)]
fn prepare_shape<F: Facade>(facade: &F)
    -> Result<(VertexBuffer<Vertex>, IndexBuffer<u16>), Box<Error>> {
    let shape = Vertex::many(vec![
         // Positions      // Colors        // Texture Coords
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

    Ok((vertex_buffer, index_buffer))

}

impl Painter for Matisse {
    fn new<F: Facade>(facade: &F) -> Result<Matisse, Box<Error>> {
        let (vertex_buffer, index_buffer) = try!(prepare_shape(facade));

        info!("Start loading textures...");
        let image1 = load_texture_jpeg(&slurp_bytes("./src/bin/textures/container.jpg"));
        let texture1 = glium::texture::Texture2d::new(facade, image1)
                           .expect("Failed to load container texture");

        let image2 = load_texture_png(&slurp_bytes("./src/bin/textures/awesomeface.png"));
        let texture2 = glium::texture::Texture2d::new(facade, image2)
                           .expect("Failed to load container texture");
        info!("... textures loaded!");

        Ok(Matisse {
            vertex_shader: slurp("./src/bin/shaders/vertex.glsl"),
            fragment_shader: slurp("./src/bin/shaders/fragment.glsl"),
            vertex_buffer: vertex_buffer,
            index_buffer: index_buffer,
            texture1: texture1,
            texture2: texture2,
        })
    }

    fn vertex_shader(&self) -> &str {
        &self.vertex_shader
    }

    fn fragment_shader(&self) -> &str {
        &self.fragment_shader
    }

    fn draw<S: Surface>(&self, api: &mut Api<S>) -> std::result::Result<(), DrawError> {

        let params = DrawParameters { ..Default::default() };

        let transform = id().scale(0.5).rotate(vec3(0.0, 0.0, 1.0), Rad::turn_div_4());

        let uniforms = uniform! {
            tex1: &self.texture1,
            tex2: &self.texture2,
            transform: transform.into_uniform(),
        };

        try!(api.surface.draw(&self.vertex_buffer,
                              &self.index_buffer,
                              api.program,
                              &uniforms,
                              &params));
        Ok(())
    }
}
