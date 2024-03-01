use super::n3ds_structures::N3DSCIAMetaSize;
use std::error::Error;
use std::fmt;

#[derive(Debug, Clone)]
pub struct N3DSCIAParsingErrorMetaNotPresentOrInvalidSize(pub N3DSCIAMetaSize);

impl fmt::Display for N3DSCIAParsingErrorMetaNotPresentOrInvalidSize {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            concat!(
                "3DS .cia Meta block is not present or meta size not a expected value.\n",
                "Found meta size value: {:?}"
            ),
            self.0,
        )
    }
}

impl Error for N3DSCIAParsingErrorMetaNotPresentOrInvalidSize {}

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
pub struct N3DSParsingError3DSXNoExtendedHeader {
    pub found_header_size: u16,
}

impl fmt::Display for N3DSParsingError3DSXNoExtendedHeader {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "No extended header on 3DSX file. Found header size is {}.",
            self.found_header_size
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
