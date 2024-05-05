use thiserror::Error;

use crate::n3ds::n3ds_parsing_errors::N3DSParsingError;
use crate::nds::nds_parsing_errors::NDSParsingError;

#[derive(Error, Debug)]
pub enum MainError {
    #[error(transparent)]
    ErrorParsingArguments(#[from] pico_args::Error),
    #[error("Found {0}, which is not a supported Nintendo DS (.nds) or Nintendo 3DS (.cia/.smdh/.3dsx/.cxi/.cci/.3ds) file")]
    InvalidContentType(String),
    #[error(transparent)]
    IOError(#[from] std::io::Error),
    #[error(transparent)]
    NDSParsingError(#[from] NDSParsingError),
    #[error(transparent)]
    N3DSParsingError(#[from] N3DSParsingError),
    #[error(transparent)]
    OtherError(#[from] Box<dyn std::error::Error>),
}
