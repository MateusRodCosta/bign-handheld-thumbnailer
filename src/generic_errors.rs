use std::error::Error;
use std::fmt;

#[derive(Debug, Clone)]
pub struct ErrorParsingThumbnailerArguments {
    pub parsed_args: pico_args::Arguments,
}

impl fmt::Display for ErrorParsingThumbnailerArguments {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Error parsing thumbnailer arguments. Parsed arguments: {:?}",
            self.parsed_args
        )
    }
}

impl Error for ErrorParsingThumbnailerArguments {}

#[derive(Debug, Clone)]
pub struct InvalidContentType {
    pub content_type: String,
}

impl fmt::Display for InvalidContentType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Found {}, which is not a supported Nintendo DS (.nds) or Nintendo 3DS (.cia/.smdh/.3dsx) file",
            self.content_type,
        )
    }
}

impl Error for InvalidContentType {}

#[derive(Debug, Clone)]
pub struct ParsingErrorByteOutOfRange {
    pub step: String,
    pub attempted: usize,
    pub maximum_size: usize,
}

impl fmt::Display for ParsingErrorByteOutOfRange {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            concat!(
                "Parsing failed due to byte out of range, check if it's a valid Nintendo DS (.nds) or 3DS (.cia/.smhd/.3dsx) file.\n",
                "Failed at '{}' on index {}, byte array size is {}",
            ),
            self.step, self.attempted, self.maximum_size
        )
    }
}

impl Error for ParsingErrorByteOutOfRange {}
