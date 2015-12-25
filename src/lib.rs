#![deny(warnings)]

#[macro_use]
extern crate log;

#[macro_use]
extern crate glium;

use glium::DisplayBuild;
use glium::backend::glutin_backend::GlutinFacade;
use glium::glutin::WindowBuilder;
use glium::glutin::GlProfile;

mod result;

pub use result::{AppError, Result};

pub struct App;

impl App {
    pub fn run() -> Result<()> {
        info!("Starting the application");
        try!(build_display());
        info!("The application has stopped");
        Ok(())
    }
}

fn build_display() -> Result<GlutinFacade> {
    WindowBuilder::new()
        .with_dimensions(800, 600)
        .with_depth_buffer(24)
        .with_gl_profile(GlProfile::Core)
        .build_glium()
        .map_err(|e| AppError::InitializationError { cause: Box::new(e) })
}
