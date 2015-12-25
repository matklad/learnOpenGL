#![deny(warnings)]

#[macro_use]
extern crate log;

#[macro_use]
extern crate glium;

mod result;
mod app;
mod vertex;

pub use result::{AppError, Result};
pub use app::App;
