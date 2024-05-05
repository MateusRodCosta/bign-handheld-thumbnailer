use thiserror::Error;

#[derive(Error, Debug)]
pub enum NDSParsingError {
    #[error("Unknown Or Invalid NDS icon version. Found {0:#06x}")]
    UnknownOrInvalidNDSIconVersion(u16),
    #[error("Unable to extract NDS icon!")]
    UnableToExtractNDSIcon,
    #[error("IO error: {0}")]
    IOError(#[from] std::io::Error),
}
