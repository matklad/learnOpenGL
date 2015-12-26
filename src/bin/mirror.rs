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
use glium::backend::glutin_backend::GlutinFacade;
use glium::index::{NoIndices, PrimitiveType};
use glium::{Surface, VertexBuffer, DrawError, Program};
use glium::glutin::Event;
use glium::texture::cubemap::Cubemap;

use lights::{App, Api, Painter, load_program, Camera, load_cubemap};
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
    camera: Camera,
    vertex_buffer: VertexBuffer<Vertex>,
    program: Program,
    cubemap: Cubemap,
}


impl Painter for Matisse {
    fn new(facade: &GlutinFacade) -> Result<Matisse, Box<Error>> {
        let shape = Vertex::many(models::skybox());
        let vertex_buffer = try!(VertexBuffer::new(facade, &shape));
        let program = try!(load_program(facade, "skybox/vertex.glsl", "skybox/fragment.glsl"));
        let cubemap = load_cubemap(facade, "skybox");

        Ok(Matisse {
            camera: Camera::new(vec3(0.0, 0.0, 3.0), vec3(0.0, 0.0, 0.0), Y),
            vertex_buffer: vertex_buffer,
            program: program,
            cubemap: cubemap,
        })
    }

    fn process_event(&mut self, event: Event, delta: f32) {
        self.camera.process_event(event, delta)
    }

    fn draw<S: Surface>(&self, api: &mut Api<S>) -> std::result::Result<(), DrawError> {
        try!(self.draw_sky(api));
        Ok(())
    }
}

impl Matisse {
    fn draw_sky<S: Surface>(&self,
                            api: &mut Api<S>)
                            -> std::result::Result<(), DrawError> {
        let uniforms = uniform! {
                model: id(),
                view: self.camera.view(),
                projection: self.projection(api),
                skybox: &self.cubemap,
            };

        try!(api.surface.draw(&self.vertex_buffer,
                              &NoIndices(PrimitiveType::TrianglesList),
                              &self.program,
                              &uniforms,
                              &api.default_params));

        Ok(())
    }

    fn projection<S: Surface>(&self, api: &mut Api<S>) -> Mat4 {
        perspective(deg(45.0), api.aspect_ratio, 0.1, 100.0)
    }
}