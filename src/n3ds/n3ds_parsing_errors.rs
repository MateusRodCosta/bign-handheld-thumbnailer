use std::error::Error;
use std::fmt;

use super::N3DSCIAMetaSize;

#[derive(Debug, Clone)]
pub struct N3DSCIAParsingErrorMetaInvalidSize(pub u32);

impl fmt::Display for N3DSCIAParsingErrorMetaInvalidSize {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "CIA Meta block size is invalid. Found {:?}", self.0,)
    }
}

impl Error for N3DSCIAParsingErrorMetaInvalidSize {}

#[derive(Debug, Clone)]
pub struct N3DSCIAParsingErrorMetaNotExpectedValue(pub N3DSCIAMetaSize);

impl fmt::Display for N3DSCIAParsingErrorMetaNotExpectedValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "CIA Meta block not present or doesn't contain the expected value. Found {:?}",
            self.0,
        )
    }
}

impl Error for N3DSCIAParsingErrorMetaNotExpectedValue {}

#[derive(Debug, Clone)]
pub struct N3DSParsingErrorSMDHMagicNotFound;

impl fmt::Display for N3DSParsingErrorSMDHMagicNotFound {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "SMDH magic not found!")
    }
}

impl Error for N3DSParsingErrorSMDHMagicNotFound {}

#[derive(Debug, Clone)]
pub struct N3DSParsingError3DSXMagicNotFound;

impl fmt::Display for N3DSParsingError3DSXMagicNotFound {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "3DSX magic not found!")
    }
}

impl Error for N3DSParsingError3DSXMagicNotFound {}

#[derive(Debug, Clone)]
pub struct N3DSParsingError3DSXNoExtendedHeader(pub u16);

impl fmt::Display for N3DSParsingError3DSXNoExtendedHeader {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "No extended header on 3DSX file. Found header size is {}",
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

#[derive(Debug, Clone)]
pub struct N3DSParsingErrorCXIMagicNotFound;

impl fmt::Display for N3DSParsingErrorCXIMagicNotFound {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "NCCH magic not found for CXI!")
    }
}

impl Error for N3DSParsingErrorCXIMagicNotFound {}

#[derive(Debug, Clone)]
pub struct N3DSParsingErrorCCIMagicNotFound;

impl fmt::Display for N3DSParsingErrorCCIMagicNotFound {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "NCSD magic not found for CCI!")
    }
}

impl Error for N3DSParsingErrorCCIMagicNotFound {}

#[derive(Debug, Clone)]
pub struct N3DSParsingErrorCCIErrorGettingExecutableContentPartition;

impl fmt::Display for N3DSParsingErrorCCIErrorGettingExecutableContentPartition {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Error getting Executable Content (CXI) partition!")
    }
}

impl Error for N3DSParsingErrorCCIErrorGettingExecutableContentPartition {}

#[derive(Debug, Clone)]
pub struct N3DSParsingErrorExeFSIconFileNotFound;

impl fmt::Display for N3DSParsingErrorExeFSIconFileNotFound {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Error finding icon file inside ExeFS!")
    }
}

impl Error for N3DSParsingErrorExeFSIconFileNotFound {}
