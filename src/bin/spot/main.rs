#[macro_use]
extern crate log;
#[macro_use]
extern crate glium;
extern crate env_logger;
extern crate lights;
extern crate cgmath;

use std::error::Error;
use std::io::prelude::*;

use env_logger::LogBuilder;
use cgmath::{Matrix4, Point3};
use glium::{DrawError, Surface, Program, VertexBuffer};
use glium::index::{NoIndices, PrimitiveType};
use glium::backend::glutin_backend::GlutinFacade;
use glium::glutin::Event;

use lights::{App, Painter, Api, Model, Camera, load_program};
use lights::math::*;

mod models;
mod vertex;

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
    try!(App::<Bacon>::run());
    Ok(())
}

struct Bacon {
    projector: Projector,
    suite: Model,
    program: Program,
}

impl Bacon {
    fn view(&self) -> Mat4 {
        let eye = Point3::new(-3.0f32, 5.0, 12.0);
        let center = Point3::new(-1.0, -1.0, 0.0);

        Mat4(Matrix4::look_at(eye, center, Y))
    }
}

impl Painter for Bacon {
    fn new(facade: &GlutinFacade) -> Result<Bacon, Box<Error>> {
        let suite = try!(Model::load(facade, "ruins/house.obj"));
        Ok(Bacon {
            projector: try!(Projector::new(facade)),
            suite: suite,
            program: try!(load_program(facade, "ruins/vertex.glsl", "ruins/fragment.glsl")),
        })
    }

    fn draw<S: Surface>(&self, api: &mut Api<S>) -> std::result::Result<(), DrawError> {
        let uniforms = uniform! {
            model: id().translate(vec3(0.0, -2.0, 0.0)),
            view: self.view(),
            projection: api.projection(),
            light: [0.0f32, 0.0, 5.0],
        };
        try!(self.suite.draw(api, &self.program, &uniforms));
        self.projector.draw(api, self)
    }


    fn process_event(&mut self, event: Event, delta_t: f32) {
        self.projector.process_event(event, delta_t);
    }
}

struct Projector {
    camera: Camera,
    vertex_buffer: VertexBuffer<vertex::Vertex>,
    program: Program,
}

impl Projector {
    fn new(facade: &GlutinFacade) -> Result<Projector, Box<Error>> {
        let vertex_buffer = try!(VertexBuffer::new(facade, &vertex::Vertex::many(models::cube())));
        let program = try!(load_program(facade, "proj/vertex.glsl", "proj/fragment.glsl"));

        Ok(Projector {
            camera: Camera::new(vec3(0.0, 2.0, 0.0), vec3(0.0, 0.0, 0.0), Y),
            vertex_buffer: vertex_buffer,
            program: program,
        })
    }

    fn draw<S: Surface>(&self, api: &mut Api<S>, p: &Bacon) -> std::result::Result<(), DrawError> {
        let uniforms = uniform! {
            model: id().translate(self.camera.position()).scale(0.5),
            view: p.view(),
            projection: api.projection(),
        };
        api.surface.draw(&self.vertex_buffer,
                         &NoIndices(PrimitiveType::TrianglesList),
                         &self.program,
                         &uniforms,
                         &api.default_params)
    }

    pub fn process_event(&mut self, event: Event, delta_t: f32) {
        self.camera.process_event(event, delta_t);
    }
}
