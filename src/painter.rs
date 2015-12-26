use std::error::Error;

use glium::{Surface, DrawError, DrawParameters};
use glium::glutin::Event;
use glium::backend::Facade;


pub trait Painter: Sized {
    fn new<F: Facade>(facade: &F) -> Result<Self, Box<Error>>;
    fn draw<S: Surface>(&self, api: &mut Api<S>) -> Result<(), DrawError>;
    fn process_event(&mut self, _event: Event, _delta: f32) {

    }
}

pub struct Api<'a, S: Surface + 'a> {
    pub surface: &'a mut S,
    pub aspect_ratio: f32,
    pub time: f32,
    pub default_params: DrawParameters<'static>,
}
