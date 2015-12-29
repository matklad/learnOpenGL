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
use glium::{DrawError, Surface, Program};
use glium::backend::glutin_backend::GlutinFacade;
use glium::glutin::Event;

use lights::{App, Painter, Api, Camera, Model, load_program};
use lights::math::*;

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
    camera: Camera,
    suite: Model,
    program: Program,
}

impl Painter for Bacon {
    fn new(facade: &GlutinFacade) -> Result<Bacon, Box<Error>> {
        let suite = try!(Model::load(facade, "ruins/house.obj"));

        Ok(Bacon {
            camera: Camera::new(vec3(0.0, 1.0, 3.0), vec3(0.0, 1.0, 0.0), Y),
            program: try!(load_program(facade, "ruins/vertex.glsl", "ruins/fragment.glsl")),
            suite: suite,
        })
    }

    fn process_event(&mut self, event: Event, delta_seconds: f32) {
        self.camera.process_event(event, delta_seconds)
    }

    fn draw<S: Surface>(&self, api: &mut Api<S>) -> std::result::Result<(), DrawError> {
        let eye = Point3::new(-3.0f32, 5.0, 12.0);
        let center = Point3::new(-1.0, -1.0, 0.0);
        let uniforms = uniform! {
            model: id().translate(vec3(0.0, -2.0, 0.0)),
            view: Mat4(Matrix4::look_at(eye, center, Y)),
            projection: api.projection(),
            light: [0.0f32, 0.0, 5.0],
        };
        self.suite.draw(api, &self.program, &uniforms)
    }
}
