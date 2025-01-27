use thiserror::Error;

#[derive(Error, Debug)]
pub enum NDSParsingError {
    #[error("Unknown Or Invalid NDS icon version. Found {0:#06x}")]
    UnknownOrInvalidNDSIconVersion(u16),
    #[error(transparent)]
    IoError(#[from] std::io::Error),
}
