use glium::{DisplayBuild, Surface, Program};
use glium::backend::glutin_backend::GlutinFacade;
use glium::glutin::{WindowBuilder, GlProfile, Event, VirtualKeyCode};
use time;

use {AppError, Result, Painter, Api};
use result::initialization_error;


pub struct App<P: Painter> {
    facade: GlutinFacade,
    painter: P,
    program: Program,
}

impl<P: Painter> App<P> {
    pub fn run() -> Result<()> {
        info!("Starting the application");
        let facade = try!(build_display());
        let painter = try!(P::new(&facade).map_err(initialization_error));
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
        let start = time::precise_time_s();
        loop {
            let current_time = (time::precise_time_s() - start) as f32;
            debug!("Loop iteration");
            try!(self.draw(current_time));
            if self.process_events() {
                return Ok(());
            }
        }
    }

    fn draw(&self, time: f32) -> Result<()> {
        let mut target = self.facade.draw();
        let (width, height) = target.get_dimensions();
        let aspect_ratio = width as f32 / height as f32;
        target.clear_color(0.2, 0.35, 0.35, 1.0);
        {
            let mut api = Api {
                surface: &mut target,
                program: &self.program,
                time: time,
                aspect_ratio: aspect_ratio
            };
            try!(self.painter.draw(&mut api));
        }
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
