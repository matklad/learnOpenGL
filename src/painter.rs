use glium::{Surface, Program, DrawError};
use glium::backend::Facade;
use glium::vertex::BufferCreationError;

pub trait Painter: Sized {
    fn new<F: Facade>(facade: &F) -> Result<Self, BufferCreationError>;
    fn vertex_shader(&self) -> &str;
    fn fragment_shader(&self) -> &str;
    fn draw<S: Surface>(&self, api: &mut Api<S>) -> Result<(), DrawError>;
}

pub struct Api<'a, S: Surface + 'a> {
    pub surface: &'a mut S,
    pub program: &'a Program,
    pub time: f32,
}
