use std::error::Error;
use std::fmt;

#[derive(Debug, Clone)]
pub struct UnknownOrInvalidNDSIconVersion(pub u16);

impl fmt::Display for UnknownOrInvalidNDSIconVersion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            concat!(
                "Unknown Or Invalid NDS icon version found.\n",
                "Found value: {:#06x}, expected values: 0x0001, 0x0002, 0x0003 or 0x0103"
            ),
            self.0
        )
    }
}

impl Error for UnknownOrInvalidNDSIconVersion {}

#[derive(Debug, Clone)]
pub struct UnableToExtractNDSIcon;

impl fmt::Display for UnableToExtractNDSIcon {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Unable to extract .nds icon!")
    }
}

impl Error for UnableToExtractNDSIcon {}
