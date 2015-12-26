#![deny(warnings)]
#[macro_use]
extern crate log;
#[macro_use]
extern crate glium;
extern crate cgmath;
extern crate time;
extern crate image;


mod result;
mod app;
mod painter;
mod textures;
mod assets;
mod camera;
pub mod math;

pub use result::{AppError, Result};
pub use app::App;
pub use painter::{Painter, Api};
pub use textures::{load_texture_jpeg, load_texture_png};
pub use assets::load_program;
pub use camera::Camera;
