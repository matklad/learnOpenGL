use glium::{DisplayBuild, Surface, DrawParameters, Depth};
use glium::draw_parameters::DepthTest;
use glium::backend::glutin_backend::GlutinFacade;
use glium::glutin::{WindowBuilder, GlProfile, Event, VirtualKeyCode};
use time;

use {Result, Painter, Api};


pub struct App<P: Painter> {
    facade: GlutinFacade,
    painter: P,
}

impl<P: Painter> App<P> {
    pub fn run() -> Result<()> {
        info!("Starting the application");
        let facade = try!(build_display());
        let painter = try!(P::new(&facade));

        let app = App {
            facade: facade,
            painter: painter,
        };
        try!(app.main_loop());
        info!("The application has stopped");
        Ok(())
    }

    fn main_loop(mut self) -> Result<()> {
        info!("Starting the main loop");
        let start = time::precise_time_s();
        let mut prev_time = 0.0;
        loop {
            let time = (time::precise_time_s() - start) as f32;
            let delta = time - prev_time;
            prev_time = time;
            debug!("Loop iteration");
            try!(self.draw(time));

            if self.process_events(delta) {
                return Ok(());
            }
        }
    }

    fn draw(&self, time: f32) -> Result<()> {
        let mut target = self.facade.draw();
        let (width, height) = target.get_dimensions();
        let aspect_ratio = width as f32 / height as f32;
        let (r, g, b) = P::clear_color();
        target.clear_color_and_depth((r, g, b, 1.0), 1.0);
        let result =
        {
            let mut api = Api {
                facade: &self.facade,
                surface: &mut target,
                time: time,
                aspect_ratio: aspect_ratio,
                default_params: DrawParameters {
                    depth: Depth {
                        test: DepthTest::IfLess,
                        write: true,
                        ..Default::default()
                    },
                    ..Default::default()
                },
            };
            self.painter.draw(&mut api)
        };
        try!(target.finish());
        try!(result);
        Ok(())
    }

    fn process_events(&mut self, delta: f32) -> bool {
        for ev in self.facade.poll_events() {
            debug!("Event {:?}", ev);
            match ev {
                Event::Closed | Event::KeyboardInput(_, _, Some(VirtualKeyCode::Escape)) => {
                    return true;
                }
                _ => self.painter.process_event(ev, delta),
            }
        }
        false
    }
}

fn build_display() -> Result<GlutinFacade> {
    Ok(try!(WindowBuilder::new()
                .with_dimensions(800, 600)
                .with_depth_buffer(24)
                .with_multisampling(4)
                .with_gl_profile(GlProfile::Core)
                .with_vsync()
                .build_glium()))
}
