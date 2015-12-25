use glium::{DisplayBuild, Surface};
use glium::backend::glutin_backend::GlutinFacade;
use glium::glutin::{WindowBuilder, GlProfile, Event, VirtualKeyCode};

use {AppError, Result};

pub struct App {
    facade: GlutinFacade,
}

impl App {
    pub fn run() -> Result<()> {
        info!("Starting the application");
        let app = App { facade: try!(build_display()) };
        try!(app.main_loop());
        info!("The application has stopped");
        Ok(())
    }

    fn main_loop(&self) -> Result<()> {
        loop {
            let mut target = self.facade.draw();
            target.clear_color(0.2, 0.35, 0.35, 1.0);
            try!(target.finish());

            for ev in self.facade.poll_events() {
                match ev {
                    Event::Closed | Event::KeyboardInput(_, _, Some(VirtualKeyCode::Escape)) => {
                        return Ok(())
                    }
                    _ => {}
                }
            }
        }
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
