use std::error::Error;
use std::fmt::{Display, Formatter, Result};

#[derive(Debug, Clone)]
pub struct UnknownOrInvalidNDSIconVersion;

impl Error for UnknownOrInvalidNDSIconVersion {}

impl Display for UnknownOrInvalidNDSIconVersion {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "Unknown Or Invalid NDS Rom Version!")
    }
}

#[derive(Debug, Clone)]
pub struct NDSParsingErrorByteOutOfRange {
    pub attempted: usize,
    pub maximum_size: usize,
    pub step: String,
}

impl Error for NDSParsingErrorByteOutOfRange {}

impl Display for NDSParsingErrorByteOutOfRange {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(
            f,
            concat!(
                "Byte out of range when parsing .nds file, please check if it's a valid NDS ROM.\n",
                "Attempted index: {}, size of byte array: {}\n",
                "Step: {}"
            ),
            self.attempted, self.maximum_size, self.step
        )
    }
}

#[derive(Debug, Clone)]
pub struct UnableToExtractNDSIcon;

impl Error for UnableToExtractNDSIcon {}

impl Display for UnableToExtractNDSIcon {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "Unable to extract NDS icon!")
    }
}
