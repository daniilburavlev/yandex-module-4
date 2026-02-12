//! `image_processor` error
use thiserror::Error;

/// Main crate error
#[derive(Debug, Clone, Error)]
pub enum ProcessorError {
    /// Wraps IO error
    #[error("{0}")]
    IO(String),
    /// Wraps `image` crate's errors
    #[error("{0}")]
    Image(String),
    /// Wraps `libloading` crate's errors
    #[error("{0}")]
    Lib(String),
    /// Wraps foreign function calls errors
    #[error("{0}")]
    FFI(String),
}

impl From<std::io::Error> for ProcessorError {
    fn from(err: std::io::Error) -> Self {
        Self::IO(err.to_string())
    }
}

impl From<image::error::ImageError> for ProcessorError {
    fn from(err: image::error::ImageError) -> Self {
        Self::Image(err.to_string())
    }
}

impl From<libloading::Error> for ProcessorError {
    fn from(err: libloading::Error) -> Self {
        Self::Lib(err.to_string())
    }
}
