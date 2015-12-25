use std::error::Error;
use std::{self, fmt};
use std::borrow::Borrow;


#[derive(Debug)]
pub enum AppError {
    InitializationError {
        cause: Box<Error>,
    },
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            AppError::InitializationError { ref cause } => {
                write!(f, "Initialization error: {}", cause)
            }
        }
    }
}

impl Error for AppError {
    fn description(&self) -> &str {
        match *self {
            AppError::InitializationError { ref cause } => cause.description(),
        }
    }

    fn cause(&self) -> Option<&Error> {
        match *self {
            AppError::InitializationError { ref cause } => Some(cause.borrow()),
        }
    }
}

pub type Result<T> = std::result::Result<T, AppError>;
