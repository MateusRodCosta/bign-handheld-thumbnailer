use std::error::Error;
use std::fmt;

#[derive(Debug, Clone)]
pub struct InvalidContentType {
    content_type: String,
}

impl InvalidContentType {
    pub fn new(content_type: String) -> InvalidContentType {
        InvalidContentType { content_type }
    }
}

impl fmt::Display for InvalidContentType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Found {}, which is not a valid Nintendo DS .nds or Nintendo 3DS .cia/.smdh/.3dsx file",
            self.content_type,
        )
    }
}

impl Error for InvalidContentType {}

#[derive(Debug, Clone)]
pub struct ParsingErrorByteOutOfRange {
    step: String,
    attempted: usize,
    maximum_size: usize,
}

impl ParsingErrorByteOutOfRange {
    pub fn new(step: String, attempted: usize, maximum_size: usize) -> ParsingErrorByteOutOfRange {
        ParsingErrorByteOutOfRange {
            step,
            attempted,
            maximum_size,
        }
    }
}

impl fmt::Display for ParsingErrorByteOutOfRange {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            concat!(
                "Parsing failed due to byte out of range, check if it's a valid Nintendo DS (.nds) or 3DS (.cia/.smhd/.3dsx) file.\n",
                "Failed at step: {}, Attempted index: {} but size of byte array is {}",
            ),
            self.step, self.attempted, self.maximum_size
        )
    }
}

impl Error for ParsingErrorByteOutOfRange {}
