#![deny(warnings)]
#[macro_use]
extern crate log;
#[macro_use]
extern crate glium;
extern crate gl;
extern crate cgmath;
extern crate time;
extern crate image;


mod result;
mod app;
mod painter;
mod assets;
mod camera;
pub mod math;

pub use result::{AppError, Result};
pub use app::App;
pub use painter::{Painter, Api};
pub use assets::{load_program, load_cubemap};
pub use camera::Camera;
