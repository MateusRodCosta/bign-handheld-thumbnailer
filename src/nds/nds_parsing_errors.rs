use std::error::Error;
use std::fmt;

#[derive(Debug, Clone)]
pub struct UnknownOrInvalidNDSIconVersion(pub u16);

impl fmt::Display for UnknownOrInvalidNDSIconVersion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Unknown Or Invalid NDS icon version. Found {:#06x}",
            self.0
        )
    }
}

impl Error for UnknownOrInvalidNDSIconVersion {}

#[derive(Debug, Clone)]
pub struct UnableToExtractNDSIcon;

impl fmt::Display for UnableToExtractNDSIcon {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Unable to extract NDS icon!")
    }
}

impl Error for UnableToExtractNDSIcon {}
