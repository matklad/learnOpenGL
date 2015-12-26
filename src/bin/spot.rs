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

use lights::{App, Api, Painter, load_program, Camera};
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
    light_program: Program,
}

impl Matisse {
    fn draw_box<S: Surface>(&self,
                            api: &mut Api<S>,
                            light_position: Vec3)
                            -> std::result::Result<(), DrawError> {
        let light_position: [f32; 3] = light_position.into();
        let uniforms = uniform! {
                model: id().rotate(Y, deg(60.0))
                           .translate(Y * -1.0),
                view: self.camera.view(),
                projection: self.projection(api),
                object_color: [1.0f32, 0.5, 0.31],
                light_color: [1.0f32, 1.0, 1.0],
                light: light_position
            };

        try!(api.surface.draw(&self.vertex_buffer,
                              &NoIndices(PrimitiveType::TrianglesList),
                              &self.program,
                              &uniforms,
                              &api.default_params));

        Ok(())
    }

    fn draw_light<S: Surface>(&self,
                              api: &mut Api<S>,
                              light_position: Vec3)
                              -> std::result::Result<(), DrawError> {
        let uniforms = uniform! {
                model: id().scale(0.2).translate(light_position),
                view: self.camera.view(),
                projection: self.projection(api),
            };

        try!(api.surface.draw(&self.vertex_buffer,
                              &NoIndices(PrimitiveType::TrianglesList),
                              &self.light_program,
                              &uniforms,
                              &api.default_params));

        Ok(())
    }

    fn projection<S: Surface>(&self, api: &mut Api<S>) -> Mat4 {
        perspective(deg(45.0), api.aspect_ratio, 0.1, 100.0)
    }
}


impl Painter for Matisse {
    fn new<F: Facade>(facade: &F) -> Result<Matisse, Box<Error>> {
        let shape = Vertex::many(models::cube());
        let vertex_buffer = try!(VertexBuffer::new(facade, &shape));
        let program = try!(load_program(facade, "vertex.glsl", "fragment.glsl"));
        let light_program = try!(load_program(facade, "vertex.glsl", "fragment_light.glsl"));

        Ok(Matisse {
            camera: Camera::new(vec3(0.0, 0.0, 5.0), vec3(0.0, 0.0, 0.0), Y),
            vertex_buffer: vertex_buffer,
            program: program,
            light_program: light_program,
        })
    }

    fn process_event(&mut self, event: Event, delta: f32) {
        self.camera.process_event(event, delta)
    }

    fn draw<S: Surface>(&self, api: &mut Api<S>) -> std::result::Result<(), DrawError> {
        let radius = 8.0;
        let light_position = vec3(api.time.sin() * radius,
                                  2.0 * api.time.sin(),
                                  api.time.cos() * radius);

        try!(self.draw_box(api, light_position));
        try!(self.draw_light(api, light_position));
        Ok(())
    }
}
