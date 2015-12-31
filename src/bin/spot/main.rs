#[macro_use]
extern crate log;
#[macro_use]
extern crate glium;
extern crate env_logger;
extern crate lights;
extern crate cgmath;

use std::io::prelude::*;

use env_logger::LogBuilder;
use cgmath::{Matrix4, Point3};
use glium::{Surface, Program, VertexBuffer, Texture2d};
use glium::draw_parameters::{DrawParameters, PolygonMode};
use glium::index::{NoIndices, PrimitiveType};
use glium::backend::glutin_backend::GlutinFacade;
use glium::glutin::Event;

use lights::{App, Painter, Api, Model, Camera, load_program, Result, load_texture};
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
    projector: Projector,
    ruins: Model,
    awesome: Texture2d,
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
    fn new(facade: &GlutinFacade) -> Result<Bacon> {
        let ruins = try!(Model::load(facade, "ruins/house.obj"));
        let awesome = load_texture("./assets/textures/awesomeface.png");
        Ok(Bacon {
            projector: try!(Projector::new(facade)),
            ruins: ruins,
            awesome: try!(Texture2d::new(facade, awesome)),
            program: try!(load_program(facade, "ruins/vertex.glsl", "ruins/fragment.glsl")),
        })
    }

    fn draw<S: Surface>(&self, api: &mut Api<S>) -> Result<()> {
        let uniforms = uniform! {
            model: id().translate(vec3(0.0, -2.0, 0.0)),
            view: self.view(),
            projection: api.projection(),
            projector_view: self.projector.camera.view(),
            light: [0.0f32, 0.0, 5.0],
            awesome: &self.awesome,
        };
        try!(self.ruins.draw(api, &self.program, &uniforms));
        self.projector.draw(api, self)
    }


    fn process_event(&mut self, event: Event, delta_t: f32) {
        self.projector.process_event(event, delta_t);
    }
}

struct Projector {
    camera: Camera,
    frustrum: Frustrum,
    vertex_buffer: VertexBuffer<vertex::Vertex>,
    program: Program,
}

impl Projector {
    fn new(facade: &GlutinFacade) -> Result<Projector> {
        let vertex_buffer = try!(VertexBuffer::new(facade, &vertex::Vertex::many(models::cube())));
        let program = try!(load_program(facade, "proj/vertex.glsl", "proj/fragment.glsl"));

        Ok(Projector {
            camera: Camera::new(vec3(0.0, 2.0, 3.0), vec3(0.0, 0.0, 3.0), Y),
            frustrum: try!(Frustrum::new(facade)),
            vertex_buffer: vertex_buffer,
            program: program,
        })
    }

    fn draw<S: Surface>(&self, api: &mut Api<S>, p: &Bacon) -> Result<()> {
        try!(self.frustrum.draw(api, p));
        let uniforms = uniform! {
            model: self.model(),
            view: p.view(),
            projection: api.projection(),
        };
        Ok(try!(api.surface.draw(&self.vertex_buffer,
                                 &NoIndices(PrimitiveType::TrianglesList),
                                 &self.program,
                                 &uniforms,
                                 &api.default_params)))
    }

    fn model(&self) -> Mat4 {
        id().translate(self.camera.position()).scale(0.25) * self.camera.rotation()
    }

    fn process_event(&mut self, event: Event, delta_t: f32) {
        self.camera.process_event(event, delta_t);
    }
}


struct Frustrum {
    vertex_buffer: VertexBuffer<vertex::Vertex>,
    program: Program,
}

impl Frustrum {
    fn new(facade: &GlutinFacade) -> Result<Frustrum> {
        let vertex_buffer = try!(VertexBuffer::new(facade, &vertex::Vertex::many(models::cube())));
        let program = try!(load_program(facade, "frustrum/vertex.glsl", "frustrum/fragment.glsl"));

        Ok(Frustrum {
            vertex_buffer: vertex_buffer,
            program: program,
        })
    }

    fn draw<S: Surface>(&self, api: &mut Api<S>, p: &Bacon) -> Result<()> {
        let uniforms = uniform! {
            model: id(),
            view: p.view(),
            projection: api.projection(),
            projector_view: p.projector.camera.view(),
        };
        Ok(try!(api.surface.draw(&self.vertex_buffer,
                                 &NoIndices(PrimitiveType::TrianglesList),
                                 &self.program,
                                 &uniforms,
                                 &DrawParameters {
                                     polygon_mode: PolygonMode::Line,
                                     ..api.default_params.clone()
                                 })))
    }
}
