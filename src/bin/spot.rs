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
use glium::index::{NoIndices, PrimitiveType};
use glium::{Surface, VertexBuffer, DrawError, Program};
use glium::glutin::Event;

use lights::{App, Api, Painter, slurp, Camera};
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
}


impl Painter for Matisse {
    fn new<F: Facade>(facade: &F) -> Result<Matisse, Box<Error>> {
        let shape = Vertex::many(models::cube());
        let vertex_buffer = try!(VertexBuffer::new(facade, &shape));
        let vertex_shader = slurp("./src/bin/shaders/vertex.glsl");
        let fragment_shader = slurp("./src/bin/shaders/fragment.glsl");

        let program = try!(Program::from_source(facade, &vertex_shader, &fragment_shader, None));

        Ok(Matisse {
            camera: Camera::new(vec3(0.0, 0.0, 3.0), vec3(0.0, 0.0, 0.0), Y),
            vertex_buffer: vertex_buffer,
            program: program,
        })
    }

    fn process_event(&mut self, event: Event, delta: f32) {
        self.camera.process_event(event, delta)
    }

    fn draw<S: Surface>(&self, api: &mut Api<S>) -> std::result::Result<(), DrawError> {
        let model = id();

        let projection = perspective(deg(45.0), api.aspect_ratio, 0.1, 100.0);

        let uniforms = uniform! {
                model: model,
                view: self.camera.view(),
                projection: projection,
                object_color: [1.0f32, 0.5, 0.31],
                light_color: [1.0f32, 1.0, 1.0],
            };

        try!(api.surface.draw(&self.vertex_buffer,
                              &NoIndices(PrimitiveType::TrianglesList),
                              &self.program,
                              &uniforms,
                              &api.default_params));

        Ok(())
    }
}
