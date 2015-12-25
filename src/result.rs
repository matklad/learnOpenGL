use std::error::Error;
use std::{self, fmt};
use std::borrow::Borrow;

use glium;


#[derive(Debug)]
pub enum AppError {
    InitializationError {
        cause: Box<Error>,
    },
    MiscError(String),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            AppError::InitializationError { ref cause } => {
                write!(f, "Initialization error: {}", cause)
            }
            AppError::MiscError(ref s) => write!(f, "Error: {}", s),
        }
    }
}

impl Error for AppError {
    fn description(&self) -> &str {
        match *self {
            AppError::InitializationError { ref cause } => cause.description(),
            AppError::MiscError(ref s) => s,
        }
    }

    fn cause(&self) -> Option<&Error> {
        match *self {
            AppError::InitializationError { ref cause } => Some(cause.borrow()),
            AppError::MiscError(_) => None,
        }
    }
}

impl From<glium::SwapBuffersError> for AppError {
    fn from(_: glium::SwapBuffersError) -> AppError {
        AppError::MiscError("Failed to swap buffers".to_owned())
    }
}

pub type Result<T> = std::result::Result<T, AppError>;
