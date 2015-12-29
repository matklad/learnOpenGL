#![deny(warnings)]
#[macro_use]
extern crate log;
#[macro_use]
extern crate glium;
#[macro_use]
extern crate quick_error;
extern crate gl;
extern crate cgmath;
extern crate time;
extern crate image;
extern crate tobj;


mod result;
mod app;
mod painter;
mod assets;
mod camera;
mod obj;
mod model;
pub mod math;

pub use result::{AppError, Result};
pub use app::App;
pub use painter::{Painter, Api};
pub use assets::{load_program, load_cubemap, load_obj, load_texture};
pub use model::Model;
pub use camera::Camera;
pub use obj::Obj;
