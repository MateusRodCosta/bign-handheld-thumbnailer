use thiserror::Error;

use super::n3ds_structures::CIAMetaSize;

#[derive(Error, Debug)]
pub enum N3DSParsingError {
    #[error("{0} magic not found! Found: {0:#04X?}")]
    FileMagicNotFound(&'static str, [u8; 4]),
    #[error("No extended header on 3DSX file. Found header size is {0}")]
    N3DSXParsingError3DSXNoExtendedHeader(u16),
    #[error(transparent)]
    CIAParsingError(#[from] CIAParsingError),
    #[error("Error getting Executable Content (CXI) partition!")]
    CCIErrorGettingExecutableContentPartition,
    #[error(transparent)]
    CXIParsingError(#[from] CXIParsingError),
    #[error("Unable to extract 3DS icon!")]
    UnableToExtractN3DSIcon,
    #[error("IO error: {0}")]
    IOError(#[from] std::io::Error),
}

#[derive(Error, Debug)]
pub enum CIAParsingError {
    #[error("CIA Meta block size is ivalid. Found {0}")]
    MetaInvalidSize(u32),
    #[error("CIA Meta block not present or doesn't contain the expected value. Found {0:?}")]
    MetaNotExpectedValue(CIAMetaSize),
}

#[derive(Error, Debug)]
pub enum CXIParsingError {
    #[error("CXI file (usually internal to a CCI) is encrypted, consider using decrypted files instead.")]
    FileEncrypted,
    #[error("Error finding icon file inside ExeFS!")]
    ExeFSIconFileNotFound,
}
