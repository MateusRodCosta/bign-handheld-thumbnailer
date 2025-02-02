use thiserror::Error;

#[derive(Error, Debug)]
pub enum N3DSParsingError {
    #[error("{0} magic not found! Found: {1:#04X?}")]
    FileMagicNotFound(&'static str, [u8; 4]),
    #[error("No extended header on 3DSX file. Found header size is {0}")]
    N3DSXParsingError3DSXNoExtendedHeader(u16),
    #[error(transparent)]
    CXIParsingError(#[from] CXIParsingError),
    #[error(transparent)]
    CIAParsingError(#[from] CIAParsingError),
    #[error("Error getting Executable Content (CXI) partition!")]
    CCIErrorGettingExecutableContentPartition,
    #[error(transparent)]
    IoError(#[from] std::io::Error),
}

#[derive(Error, Debug)]
pub enum CIAParsingError {
    #[error("CIA Meta block size is invalid. Found {0}")]
    MetaInvalidSize(u32),
    #[error("CIA has no icon available either on Meta section or on CXI: {0}")]
    NoIconAvailable(#[from] CXIParsingError),
    #[error("CIA Title Metadata contains no signature or a invalid value. Found {0:x?}")]
    SignatureTypeInvalidValue(u32),
    #[error("CIA Content Index contains invalid value. Found {0:x?}")]
    ContentIndexInvalidValue(u16),
}

#[derive(Error, Debug)]
pub enum CXIParsingError {
    #[error("No CXI found.")]
    NoCXIContent,
    #[error("CXI file (usually internal to a CCI or CIA) is encrypted, consider using decrypted files instead.")]
    FileEncrypted,
    #[error("Error finding icon file inside ExeFS!")]
    ExeFSIconFileNotFound,
}
