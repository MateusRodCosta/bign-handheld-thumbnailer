use thiserror::Error;

use crate::n3ds::errors::N3DSParsingError;
use crate::nds::errors::NDSParsingError;
use crate::utils::RomFormat;

#[derive(Error, Debug)]
pub enum ThumbnailerError {
    #[error("Error parsing arguments: {0}")]
    ArgumentParsingError(#[from] pico_args::Error),
    #[error("Failed to detect rom format.")]
    RomFormatDetectionFailure,
    #[error("Incompatible rom format, {0:?} is not a supported Nintendo DS or 3DS file.")]
    IncompatibleRomFormat(RomFormat),
    #[error(transparent)]
    IoError(#[from] std::io::Error),
    #[error(transparent)]
    ImageError(#[from] image::ImageError),
    #[error("NDS format parsing error: {0}")]
    NDSParsingError(#[from] NDSParsingError),
    #[error("3DS format parsing error: {0}")]
    N3DSParsingError(#[from] N3DSParsingError),
}
