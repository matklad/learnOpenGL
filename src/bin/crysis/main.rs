#[macro_use]
extern crate log;
#[macro_use]
extern crate glium;
extern crate env_logger;
extern crate lights;

use std::error::Error;
use std::io::prelude::*;

use env_logger::LogBuilder;
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
        let suite = try!(Model::load(facade, "nanosuite/nanosuit.obj"));

        Ok(Bacon {
            camera: Camera::new(vec3(0.0, 1.0, 3.0), vec3(0.0, 1.0, 0.0), Y),
            program: try!(load_program(facade, "suite/vertex.glsl", "suite/fragment.glsl")),
            suite: suite,
        })
    }

    fn clear_color() -> (f32, f32, f32) {
        (0.4, 0.4, 0.1)
    }

    fn process_event(&mut self, event: Event, delta_seconds: f32) {
        println!("{}", delta_seconds);
        self.camera.process_event(event, delta_seconds)
    }

    fn draw<S: Surface>(&self, api: &mut Api<S>) -> std::result::Result<(), DrawError> {
        let uniforms = uniform! {
            model: id().scale(0.1),
            view: self.camera.view(),
            projection: api.projection(),
        };
        self.suite.draw(api, &self.program, &uniforms)
    }
}
