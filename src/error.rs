use thiserror::Error;
use gio::glib;

use crate::n3ds;
use crate::nds;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Error parsing arguments: {0}")]
    ErrorParsingArguments(#[from] pico_args::Error),
    #[error("Missing File Parameters")]
    MissingFileParams,
    #[error("Error getting file mime type: {0}")]
    ErrorGettingMimeType(#[from] MimeTypeErrors),
    #[error("Found {0}, which is not a supported Nintendo DS (.nds) or Nintendo 3DS (.cia/.smdh/.3dsx/.cxi/.cci/.3ds) file")]
    IncompatibleMimeType(String),
    #[error("I/O error: {0}")]
    IoError(#[from] std::io::Error),
    #[error("Image eror: {0}")]
    ImageError(#[from] image::ImageError),
    #[error("NDS format parsing error: {0}")]
    NDSParsingError(#[from] nds::errors::ParsingError),
    #[error("3DS format parsing error: {0}")]
    N3DSParsingError(#[from] n3ds::errors::ParsingError),
}

#[derive(Error, Debug)]
pub enum MimeTypeErrors {
    #[error("Couldn't query file info: {0}")]
    FileInfoQueryFailed(#[from] glib::Error),
    #[error("Invalid Mime Type")]
    InvalidMimeType,
}
