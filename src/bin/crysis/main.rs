#[macro_use]
extern crate log;
#[macro_use]
extern crate glium;
extern crate env_logger;
extern crate lights;

use std::io::prelude::*;

use env_logger::LogBuilder;
use glium::{Surface, Program};
use glium::backend::glutin_backend::GlutinFacade;
use glium::glutin::Event;

use lights::{App, Painter, Api, Camera, Model, load_program, Result};
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
        if let Some(info) = e.guru_info() {
            writeln!(std::io::stderr(), "\nGuru meditation:\n{}", info).unwrap();
        }
    }
}

fn run() -> Result<()> {
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
    fn new(facade: &GlutinFacade) -> Result<Bacon> {
        let suite = try!(Model::load(facade, "nanosuit/nanosuit.obj"));

        Ok(Bacon {
            camera: Camera::new(vec3(0.0, 1.0, 3.0), vec3(0.0, 1.0, 0.0), Y),
            program: try!(load_program(facade, "suit/vertex.glsl", "suit/fragment.glsl")),
            suite: suite,
        })
    }

    fn process_event(&mut self, event: Event, delta_seconds: f32) {
        self.camera.process_event(event, delta_seconds)
    }

    fn draw<S: Surface>(&self, api: &mut Api<S>) -> Result<()> {
        let radius = 8.0;
        let light_position = [api.time.sin() * radius,
                              2.0 * api.time.sin(),
                              api.time.cos() * radius];
        let uniforms = uniform! {
            model: id().scale(0.1),
            view: self.camera.view(),
            projection: api.projection(),
            light: light_position,
        };
        self.suite.draw(api.surface, &api.default_params, &self.program, &uniforms)
    }
}
