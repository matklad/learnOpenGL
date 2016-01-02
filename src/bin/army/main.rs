#[macro_use]
extern crate log;
#[macro_use]
extern crate glium;
extern crate env_logger;
extern crate lights;

use std::io::prelude::*;

use env_logger::LogBuilder;
use glium::{Surface, Program, Texture2d};
use glium::texture::{UncompressedFloatFormat, DepthTexture2d, MipmapsOption, DepthFormat};
use glium::backend::glutin_backend::GlutinFacade;
use glium::glutin::Event;
use glium::framebuffer::MultiOutputFrameBuffer;

use lights::{App, Painter, Api, Camera, Model, load_program, Result};
use lights::math::*;

mod quad;

use quad::Quad;

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
    App::<Bacon>::run()
}

struct Bacon {
    camera: Camera,
    suite: Model,
    program: Program,
    quad: Quad,
    g_buffer: GBuffer,
}

struct GBuffer {
    albedo: Texture2d,
    specular_shininess: Texture2d,
    position: Texture2d,
    normal: Texture2d,
    depth: DepthTexture2d,
}

impl GBuffer {
    fn new(facade: &GlutinFacade) -> Result<GBuffer> {
        let (width, height) = (800, 600);

        let make_texture = || {
            Texture2d::empty_with_format(facade,
                                         UncompressedFloatFormat::F32F32F32F32,
                                         MipmapsOption::NoMipmap,
                                         width,
                                         height)
        };

        Ok(GBuffer {
            albedo: try!(make_texture()),
            specular_shininess: try!(make_texture()),
            position: try!(make_texture()),
            normal: try!(make_texture()),
            depth: DepthTexture2d::empty_with_format(facade,
                                                     DepthFormat::F32,
                                                     MipmapsOption::NoMipmap,
                                                     width,
                                                     height)
                       .unwrap(),
        })
    }

    fn buffer(&self, facade: &GlutinFacade) -> MultiOutputFrameBuffer {
        let output = &[("albedo", &self.albedo),
                       ("specular_shininess", &self.specular_shininess),
                       ("position", &self.position),
                       ("normal", &self.normal)];
        MultiOutputFrameBuffer::with_depth_buffer(facade, output.iter().cloned(), &self.depth)
            .unwrap()
    }
}

impl Painter for Bacon {
    fn new(facade: &GlutinFacade) -> Result<Bacon> {
        let suite = try!(Model::load(facade, "nanosuit/nanosuit.obj"));
        Ok(Bacon {
            camera: Camera::new(vec3(0.0, 1.0, 3.0), vec3(0.0, 1.0, 0.0), Y),
            program: try!(load_program(facade, "army/geom/vertex.glsl", "army/geom/fragment.glsl")),
            suite: suite,
            quad: try!(Quad::new(facade)),
            g_buffer: try!(GBuffer::new(facade)),
        })
    }

    fn process_event(&mut self, event: Event, delta_seconds: f32) {
        self.camera.process_event(event, delta_seconds)
    }

    fn draw<S: Surface>(&self, api: &mut Api<S>) -> Result<()> {
        let mut g_buffer = self.g_buffer.buffer(api.facade);
        g_buffer.clear_color_and_depth((0.0, 0.0, 0.0, 1.0), 1.0);

        let positions = [vec3(0.0, -3.0, -3.0),
                         vec3(3.0, -3.0, -3.0),
                         vec3(-3.0, -3.0, 0.0),
                         vec3(0.0, -3.0, 0.0),
                         vec3(3.0, -3.0, 0.0),
                         vec3(-3.0, -3.0, 3.0),
                         vec3(0.0, -3.0, 3.0),
                         vec3(3.0, -3.0, 3.0)];

        for &pos in positions.into_iter() {
            let uniforms = uniform! {
                model: id().translate(pos).scale(0.2),
                view: self.camera.view(),
                projection: api.projection(),
            };

            try!(self.suite.draw(&mut g_buffer, &api.default_params, &self.program, &uniforms));
        }

        let radius = 8.0;
        let light_position = [api.time.sin() * radius,
                              2.0 * api.time.sin(),
                              api.time.cos() * radius];

        let uniforms = uniform! {
            albedo: &self.g_buffer.albedo,
            specular_shininess: &self.g_buffer.specular_shininess,
            position: &self.g_buffer.position,
            normal: &self.g_buffer.normal,
            view: self.camera.view(),
            projection: api.projection(),
            light: light_position,
            light_color: [0.2f32, 0.2, 0.2],
        };
        self.quad.draw(api.surface, &api.default_params, &uniforms)
    }
}
