#![deny(warnings)]
#[macro_use]
extern crate log;
#[macro_use]
extern crate glium;
extern crate time;
extern crate image;

mod result;
mod app;
mod painter;
mod textures;

pub use result::{AppError, Result};
pub use app::App;
pub use painter::{Painter, Api};
pub use textures::load_texture_jpeg;
