use std::error::Error;
use std::fmt::{Display, Formatter, Result};

#[derive(Debug, Clone)]
pub struct N3DSParsingErrorByteOutOfRange {
    pub attempted: usize,
    pub maximum_size: usize,
    pub step: String,
}

impl Error for N3DSParsingErrorByteOutOfRange {}

impl Display for N3DSParsingErrorByteOutOfRange {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(
            f,
            concat!(
                "Byte out of range when parsing .cia file, please check if it's a valid 3DS CIA.\n",
                "Attempted index: {}, size of byte array: {}\n",
                "Step: {}"
            ),
            self.attempted, self.maximum_size, self.step
        )
    }
}

#[derive(Debug, Clone)]
pub struct N3DSParsingErrorMetaNotPresentOrInvalidSize;

impl Error for N3DSParsingErrorMetaNotPresentOrInvalidSize {}

impl Display for N3DSParsingErrorMetaNotPresentOrInvalidSize {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(
            f,
            "3DS .cia Meta block is not present or meta size not a expected value."
        )
    }
}

#[derive(Debug, Clone)]
pub struct N3DSParsingErrorSMDHMagicNotFound;

impl Error for N3DSParsingErrorSMDHMagicNotFound {}

impl Display for N3DSParsingErrorSMDHMagicNotFound {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "SMDH magic not found on SMDH block.")
    }
}

#[derive(Debug, Clone)]
pub struct N3DSParsingError3DSXMagicNotFound;

impl Error for N3DSParsingError3DSXMagicNotFound {}

impl Display for N3DSParsingError3DSXMagicNotFound {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "3DSX magic not found on 3DSX file.")
    }
}

#[derive(Debug, Clone)]
pub struct N3DSParsingError3DSXNoExtendedHeader;

impl Error for N3DSParsingError3DSXNoExtendedHeader {}

impl Display for N3DSParsingError3DSXNoExtendedHeader {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "No extended header on 3DSX file.")
    }
}

#[derive(Debug, Clone)]
pub struct UnableToExtractN3DSIcon;

impl Error for UnableToExtractN3DSIcon {}

impl Display for UnableToExtractN3DSIcon {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "Unable to extract 3DS icon!")
    }
}
