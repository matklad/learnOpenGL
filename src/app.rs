use glium::{DisplayBuild, Surface, Program};
use glium::backend::glutin_backend::GlutinFacade;
use glium::glutin::{WindowBuilder, GlProfile, Event, VirtualKeyCode};

use {AppError, Result, Painter};

pub struct App<P: Painter> {
    facade: GlutinFacade,
    painter: P,
    program: Program,
}

impl<P: Painter> App<P> {
    pub fn run() -> Result<()> {
        info!("Starting the application");
        let facade = try!(build_display());
        let painter = try!(P::new(&facade));
        let program = try!(Program::from_source(&facade,
                                                painter.vertex_shader(),
                                                painter.fragment_shader(),
                                                None));
        let app = App {
            facade: facade,
            painter: painter,
            program: program,
        };
        try!(app.main_loop());
        info!("The application has stopped");
        Ok(())
    }

    fn main_loop(&self) -> Result<()> {
        info!("Starting the main loop");
        loop {
            debug!("Loop iteration");
            try!(self.draw());
            if self.process_events() {
                return Ok(())
            }
        }
    }

    fn draw(&self) -> Result<()> {
        let mut target = self.facade.draw();
        target.clear_color(0.2, 0.35, 0.35, 1.0);

        try!(self.painter.draw(&mut target, &self.program));
        try!(target.finish());
        Ok(())
    }

    fn process_events(&self) -> bool {
        for ev in self.facade.poll_events() {
            debug!("Event {:?}", ev);
            match ev {
                Event::Closed | Event::KeyboardInput(_, _, Some(VirtualKeyCode::Escape)) => {
                    return true;
                }
                _ => {}
            }
        }
        false
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
