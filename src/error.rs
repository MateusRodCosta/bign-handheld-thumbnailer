use thiserror::Error;

use crate::n3ds::errors::N3DSParsingError;
use crate::nds::errors::NDSParsingError;

#[derive(Error, Debug)]
pub enum ThumbnailerError {
    #[error("Error parsing arguments: {0}")]
    ArgumentParsingError(#[from] pico_args::Error),
    #[error("Couldn't query file info: {0}")]
    FileInfoQueryFailure(#[from] gio::glib::Error),
    #[error("Failed to detect mime type.")]
    MimeTypeDetectionFailure,
    #[error("Incompatible mime type, {0} is not a supported Nintendo DS or 3DS file.")]
    IncompatibleMimeType(String),
    #[error(transparent)]
    IoError(#[from] std::io::Error),
    #[error(transparent)]
    ImageError(#[from] image::ImageError),
    #[error("NDS format parsing error: {0}")]
    NDSParsingError(#[from] NDSParsingError),
    #[error("3DS format parsing error: {0}")]
    N3DSParsingError(#[from] N3DSParsingError),
}
