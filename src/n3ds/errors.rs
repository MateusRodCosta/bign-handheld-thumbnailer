use thiserror::Error;

use super::structures::CIAMetaSize;

#[derive(Error, Debug)]
pub enum ParsingError {
    #[error("{0} magic not found! Found: {0:#04X?}")]
    FileMagicNotFound(&'static str, [u8; 4]),
    #[error("No extended header on 3DSX file. Found header size is {0}")]
    N3DSXParsingError3DSXNoExtendedHeader(u16),
    #[error("CIA Meta block size is ivalid. Found {0}")]
    CIAMetaInvalidSize(u32),
    #[error("CIA Meta block not present or doesn't contain the expected value. Found {0:?}")]
    CIAMetaNotExpectedValue(CIAMetaSize),
    #[error("Error getting Executable Content (CXI) partition!")]
    CCIErrorGettingExecutableContentPartition,
    #[error("CXI file (usually internal to a CCI) is encrypted, consider using decrypted files instead.")]
    CXIFileEncrypted,
    #[error("Error finding icon file inside ExeFS!")]
    CXIExeFSIconFileNotFound,
    #[error("Unable to extract 3DS icon!")]
    UnableToExtractN3DSIcon,
    #[error("IO error: {0}")]
    IOError(#[from] std::io::Error),
}
