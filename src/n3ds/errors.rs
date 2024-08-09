use thiserror::Error;

#[derive(Error, Debug)]
pub enum ParsingError {
    #[error("{0} magic not found! Found: {1:#04X?}")]
    FileMagicNotFound(&'static str, [u8; 4]),
    #[error("No extended header on 3DSX file. Found header size is {0}")]
    N3DSXParsingError3DSXNoExtendedHeader(u16),
    #[error("CIA Meta block size is invalid. Found {0}")]
    CIAMetaInvalidSize(u32),
    #[error("CIA has no icon available either on Meta section or on CXI.")]
    CIAHasNoIconAvailable,
    #[error("CIA Title Metadata contains no signature or a invalid value. Found {0:x?}")]
    CIASignatureTypeInvalidValue(u32),
    #[error("CIA Content Index contains invalid value. Found {0:x?}")]
    CIAContentIndexInvalidValue(u16),
    #[error("Error getting Executable Content (CXI) partition!")]
    CCIErrorGettingExecutableContentPartition,
    #[error("CXI file (usually internal to a CCI or CIA) is encrypted, consider using decrypted files instead.")]
    CXIFileEncrypted,
    #[error("Error finding icon file inside ExeFS!")]
    CXIExeFSIconFileNotFound,
    #[error("IO error: {0}")]
    IOError(#[from] std::io::Error),
}
