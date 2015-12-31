use std::{self, fmt};
use std::error::Error;

use glium::index::BufferCreationError as IndexBufferCreationError;
use glium::program::ProgramCreationError;
use glium::vertex::BufferCreationError as VertexBufferCreationError;
use glium::{SwapBuffersError, GliumCreationError, DrawError};
use glium::texture::TextureCreationError;
use glium::glutin::CreationError;
use tobj::LoadError;
use image::ImageError;

pub type Result<T> = std::result::Result<T, Oops>;

#[derive(Debug)]
pub struct Oops {
    message: String,
    cause: Option<Box<Error>>,
    debug: Option<String>,
}

pub fn oops<E: Error + 'static, S: Into<String>>(message: S, err: E) -> Oops {
    Oops {
        message: message.into(),
        cause: Some(Box::new(err)),
        debug: None,
    }
}

impl Oops {
    pub fn from_debug<S: Into<String>, D: fmt::Debug>(message: S, err: D) -> Oops {
        Oops {
            message: message.into(),
            cause: None,
            debug: Some(format!("{:#?}", err)),
        }
    }

    pub fn message(&self) -> &str {
        &self.message
    }

    pub fn guru_info(&self) -> Option<String> {
        if let Some(ref cause) = self.cause {
            return Some(format!("{:#?}", cause))
        }
        if let Some(ref d) = self.debug {
            return Some(d.clone())
        }
        None
    }
}

impl fmt::Display for Oops {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Oops: {}", self.message())
    }
}

// impl Error for Oops {
//    fn description(&self) -> &str {
//        self.message()
//    }
//
//    fn cause(&self) -> Option<&Error> {
//        None
//    }
// }

// impl<E: Error> From<E> for Oops {
//    fn from(e: E) -> Oops {
//        Oops(e.to_string())
//    }
// }

impl From<SwapBuffersError> for Oops {
    fn from(err: SwapBuffersError) -> Oops {
        Oops::from_debug("Failed to swap buffers", err)
    }
}

impl From<GliumCreationError<CreationError>> for Oops {
    fn from(err: GliumCreationError<CreationError>) -> Oops {
        oops("failed to create OpenGL context", err)
    }
}

impl From<VertexBufferCreationError> for Oops {
    fn from(err: VertexBufferCreationError) -> Oops {
        oops("failed to create a vertex buffer", err)
    }
}

impl From<IndexBufferCreationError> for Oops {
    fn from(err: IndexBufferCreationError) -> Oops {
        Oops::from_debug("failed to create a index buffer", err)
    }
}

impl From<DrawError> for Oops {
    fn from(err: DrawError) -> Oops {
        Oops::from_debug("failed to draw a frame", err)
    }
}

impl From<LoadError> for Oops {
    fn from(err: LoadError) -> Oops {
        Oops::from_debug("failed to load a model", err)
    }
}

impl From<ProgramCreationError> for Oops {
    fn from(err: ProgramCreationError) -> Oops {
        oops("failed to create shader program", err)
    }
}

impl From<ImageError> for Oops {
    fn from(err: ImageError) -> Oops {
        oops("failed to read an image\n{}", err)
    }
}

impl From<TextureCreationError> for Oops{
    fn from (err: TextureCreationError) -> Oops{
        Oops::from_debug("failed to crate a texture", err)
    }
}