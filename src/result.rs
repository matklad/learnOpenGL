use std::error::Error;
use std::{self, fmt};
use std::borrow::Borrow;

use glium;
use glium::program::ProgramCreationError;

pub type Result<T> = std::result::Result<T, AppError>;

#[derive(Debug)]
pub enum AppError {
    InitializationError {
        cause: Box<Error>,
    },
    ShaderError {
        cause: ProgramCreationError,
    },
    DrawError(String),
    MiscError(String),
}

pub fn initialization_error(err: Box<Error>) -> AppError {
    AppError::InitializationError { cause: err }
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            AppError::InitializationError { ref cause } => {
                write!(f, "Initialization error: {}", cause)
            }
            AppError::ShaderError { ref cause } => write!(f, "Shader error: {}", cause),
            AppError::DrawError(ref s) => write!(f, "Draw error: {}", s),
            AppError::MiscError(ref s) => write!(f, "Error: {}", s),
        }
    }
}

impl Error for AppError {
    fn description(&self) -> &str {
        match *self {
            AppError::InitializationError { ref cause } => cause.description(),
            AppError::ShaderError { ref cause } => cause.description(),

            AppError::DrawError(ref s) => s,
            AppError::MiscError(ref s) => s,
        }
    }

    fn cause(&self) -> Option<&Error> {
        match *self {
            AppError::InitializationError { ref cause } => Some(cause.borrow()),
            AppError::ShaderError { ref cause } => Some(cause),
            AppError::MiscError(..) | AppError::DrawError(..) => None,
        }
    }
}

impl From<ProgramCreationError> for AppError {
    fn from(e: ProgramCreationError) -> AppError {
        AppError::ShaderError { cause: e }
    }
}

impl From<glium::SwapBuffersError> for AppError {
    fn from(_: glium::SwapBuffersError) -> AppError {
        AppError::MiscError("Failed to swap buffers".to_owned())
    }
}

impl From<glium::DrawError> for AppError {
    fn from(e: glium::DrawError) -> AppError {
        AppError::MiscError(e.to_string())
    }
}
