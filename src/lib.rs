#![deny(warnings)]

#[macro_use]
extern crate log;

#[macro_use]
extern crate glium;

mod result;
mod app;
mod vertex;
mod painter;

pub use result::{AppError, Result};
pub use app::App;
pub use vertex::Vertex;
pub use painter::Painter;
