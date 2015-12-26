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
use glium::texture::Texture2d;
use glium::index::{NoIndices, PrimitiveType};
use glium::{Surface, VertexBuffer, DrawError};

use lights::{App, Api, Painter, load_texture_jpeg, load_texture_png, slurp, slurp_bytes};
use lights::math::*;

mod vertex;
mod models;

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
    texture1: Texture2d,
    texture2: Texture2d,
}


impl Painter for Matisse {
    fn new<F: Facade>(facade: &F) -> Result<Matisse, Box<Error>> {
        let shape = Vertex::many(models::cube());
        let vertex_buffer = try!(VertexBuffer::new(facade, &shape));

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

        let model = id().rotate(vec3(0.5, 1.0, 0.0), deg(-55.0) * api.time);
        let view = id().translate(Z * -3.0);
        let projection = perspective(deg(45.0), api.aspect_ratio, 0.1, 100.0);

        let uniforms = uniform! {
            tex1: &self.texture1,
            tex2: &self.texture2,
            model: model,
            view: view,
            projection: projection
        };

        try!(api.surface.draw(&self.vertex_buffer,
                              &NoIndices(PrimitiveType::TrianglesList),
                              api.program,
                              &uniforms,
                              &api.default_params));
        Ok(())
    }
}
