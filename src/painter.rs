use std::error::Error;

use glium::{Surface, DrawError, DrawParameters};
use glium::glutin::Event;
use glium::backend::glutin_backend::GlutinFacade;
use math::*;


pub trait Painter: Sized {
    fn new(facade: &GlutinFacade) -> Result<Self, Box<Error>>;
    fn draw<S: Surface>(&self, api: &mut Api<S>) -> Result<(), DrawError>;
    fn process_event(&mut self, _event: Event, _delta_seconds: f32) {}
    fn clear_color() -> (f32, f32, f32) {
        (0.2, 0.02, 0.8)
    }
}

pub struct Api<'a, S: Surface + 'a> {
    pub surface: &'a mut S,
    pub aspect_ratio: f32,
    pub time: f32,
    pub default_params: DrawParameters<'static>,
}

impl<'a, S: Surface> Api<'a, S> {
    pub fn projection(&self) -> Mat4 {
        perspective(deg(45.0), self.aspect_ratio, 0.1, 100.0)
    }
}