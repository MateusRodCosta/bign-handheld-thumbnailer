use thiserror::Error;

use crate::n3ds::errors::N3DSParsingError;
use crate::nds::errors::NDSParsingError;

#[derive(Error, Debug)]
pub enum ThumbnailerError {
    #[error("Error parsing arguments: {0}")]
    ArgumentParsingError(#[from] pico_args::Error),
    #[error("File parameters are missing, aborting.")]
    MissingFileParams,
    #[error("Mime type detection error: {0}")]
    MimeTypeDetectionError(#[from] MimeTypeDetectionError),
    #[error("Found {0}, which is not a supported Nintendo DS (.nds) or Nintendo 3DS (.cia/.smdh/.3dsx/.cxi/.cci/.3ds) file.")]
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

#[derive(Error, Debug)]
pub enum MimeTypeDetectionError {
    #[error("Couldn't query file info: {0}")]
    FileInfoQueryFailure(#[from] gio::glib::Error),
    #[error("Failed to detect mime type.")]
    MimeTypeDetectionFailure,
}
