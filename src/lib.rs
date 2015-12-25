#![deny(warnings)]

#[macro_use]
extern crate log;

#[macro_use]
extern crate glium;

mod result;
mod app;

pub use result::{AppError, Result};
pub use app::App;
