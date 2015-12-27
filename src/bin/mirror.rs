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
use glium::index::{NoIndices, PrimitiveType, IndexBuffer};
use glium::{Surface, VertexBuffer, DrawError, Program, DrawParameters, Depth};
use glium::glutin::Event;
use glium::texture::cubemap::Cubemap;

use lights::{App, Api, Painter, load_program, Camera, load_cubemap, load_obj};
use lights::math::*;

mod vertex;
mod models;

use vertex::{Vertex, VertexNormal};

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
    skybox: SkyBox,
    cube: Cube,
}

impl Painter for Matisse {
    fn new(facade: &GlutinFacade) -> Result<Matisse, Box<Error>> {

        Ok(Matisse {
            camera: Camera::new(vec3(0.0, 0.0, 3.0), vec3(0.0, 0.0, 0.0), Y),
            skybox: try!(SkyBox::new(facade)),
            cube: try!(Cube::new(facade)),
        })
    }

    fn process_event(&mut self, event: Event, delta: f32) {
        self.camera.process_event(event, delta)
    }

    fn draw<S: Surface>(&self, api: &mut Api<S>) -> std::result::Result<(), DrawError> {
        try!(self.skybox.draw(api, self));
        try!(self.cube.draw(api, self));
        Ok(())
    }
}

impl Matisse {
    fn projection<S: Surface>(&self, api: &mut Api<S>) -> Mat4 {
        perspective(deg(45.0), api.aspect_ratio, 0.1, 100.0)
    }
}

struct SkyBox {
    vertex_buffer: VertexBuffer<Vertex>,
    program: Program,
    cubemap: Cubemap,
}

impl SkyBox {
    fn new(facade: &GlutinFacade) -> Result<SkyBox, Box<Error>> {
        let shape = Vertex::many(models::skybox());
        Ok(SkyBox {
            vertex_buffer: try!(VertexBuffer::new(facade, &shape)),
            program: try!(load_program(facade, "skybox/vertex.glsl", "skybox/fragment.glsl")),
            cubemap: load_cubemap(facade, "skybox"),
        })
    }

    fn draw<S: Surface>(&self,
                        api: &mut Api<S>,
                        p: &Matisse)
                        -> std::result::Result<(), DrawError> {
        let uniforms = uniform! {
            view: p.camera.view(),
            projection: p.projection(api),
            skybox: &self.cubemap,
        };

        try!(api.surface.draw(&self.vertex_buffer,
                              &NoIndices(PrimitiveType::TrianglesList),
                              &self.program,
                              &uniforms,
                              &DrawParameters {
                                  depth: Depth { write: false, ..Default::default() },
                                  ..api.default_params.clone()
                              }));
        Ok(())
    }
}

struct Cube {
    vertex_buffer: VertexBuffer<VertexNormal>,
    index_buffer: IndexBuffer<u16>,
    program: Program,
}

impl Cube {
    fn new(facade: &GlutinFacade) -> Result<Cube, Box<Error>> {
        let model = load_obj("bunny_with_normals.obj");
        let shape = VertexNormal::from_obj(&model);
        Ok(Cube {
            vertex_buffer: try!(VertexBuffer::new(facade, &shape)),
            index_buffer: IndexBuffer::new(facade, PrimitiveType::TrianglesList, &model.indices)
                              .expect("Failed to crate an index buffer"),

            program: try!(load_program(facade, "cube/vertex.glsl", "cube/fragment.glsl")),
        })
    }

    fn draw<S: Surface>(&self,
                        api: &mut Api<S>,
                        p: &Matisse)
                        -> std::result::Result<(), DrawError> {
        let uniforms = uniform! {
            model: id().scale(5.0),
            view: p.camera.view(),
            projection: p.projection(api),
            camera_position: p.camera.position(),
            skybox: &p.skybox.cubemap,
        };

        try!(api.surface.draw(&self.vertex_buffer,
                              &self.index_buffer,
                              &self.program,
                              &uniforms,
                              &api.default_params));
        Ok(())
    }
}
