use thiserror::Error;

use crate::n3ds::errors::N3DSParsingError;
use crate::nds::errors::NDSParsingError;

#[derive(Error, Debug)]
pub enum ThumbnailerError {
    #[error("Error parsing arguments: {0}")]
    ArgumentParsingError(#[from] pico_args::Error),
    #[error("Failed to detect rom format. Check if it's a supported Nintendo DS or 3DS file.")]
    RomFormatDetectionFailure,
    #[error(transparent)]
    IoError(#[from] std::io::Error),
    #[error(transparent)]
    ImageError(#[from] image::ImageError),
    #[error("NDS format parsing error: {0}")]
    NDSParsingError(#[from] NDSParsingError),
    #[error("3DS format parsing error: {0}")]
    N3DSParsingError(#[from] N3DSParsingError),
}
