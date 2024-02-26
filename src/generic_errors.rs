use std::error::Error;
use std::fmt::{Display, Formatter, Result};

#[derive(Debug, Clone)]
pub struct InvalidMimeType;

impl Error for InvalidMimeType {}

impl Display for InvalidMimeType {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(
            f,
            "File is not a valid Nitendo DS .nds or Nitendo 3DS .cia file"
        )
    }
}
