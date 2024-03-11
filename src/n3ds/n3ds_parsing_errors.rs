use std::error::Error;
use std::fmt;

use super::N3DSCIAMetaSize;

#[derive(Debug, Clone)]
pub struct N3DSCIAParsingErrorMetaInvalidSize(pub u32);

impl fmt::Display for N3DSCIAParsingErrorMetaInvalidSize {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "3DS .cia Meta block has a invalid size. Found meta size value: {:?}",
            self.0,
        )
    }
}

impl Error for N3DSCIAParsingErrorMetaInvalidSize {}

#[derive(Debug, Clone)]
pub struct N3DSCIAParsingErrorMetaNotExpectedValue(pub N3DSCIAMetaSize);

impl fmt::Display for N3DSCIAParsingErrorMetaNotExpectedValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            concat!(
                "3DS .cia Meta block is not present or doesn't contain a the expected value.\n",
                "Found meta size value: {:?}"
            ),
            self.0,
        )
    }
}

impl Error for N3DSCIAParsingErrorMetaNotExpectedValue {}

#[derive(Debug, Clone)]
pub struct N3DSParsingErrorSMDHMagicNotFound;

impl fmt::Display for N3DSParsingErrorSMDHMagicNotFound {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "SMDH magic not found on SMDH block.")
    }
}

impl Error for N3DSParsingErrorSMDHMagicNotFound {}

#[derive(Debug, Clone)]
pub struct N3DSParsingError3DSXMagicNotFound;

impl fmt::Display for N3DSParsingError3DSXMagicNotFound {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "3DSX magic not found on 3DSX file.")
    }
}

impl Error for N3DSParsingError3DSXMagicNotFound {}

#[derive(Debug, Clone)]
pub struct N3DSParsingError3DSXNoExtendedHeader(pub u16);

impl fmt::Display for N3DSParsingError3DSXNoExtendedHeader {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "No extended header on 3DSX file. Found header size is {}.",
            self.0
        )
    }
}

impl Error for N3DSParsingError3DSXNoExtendedHeader {}

#[derive(Debug, Clone)]
pub struct UnableToExtractN3DSIcon;

impl fmt::Display for UnableToExtractN3DSIcon {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Unable to extract 3DS icon!")
    }
}

impl Error for UnableToExtractN3DSIcon {}
