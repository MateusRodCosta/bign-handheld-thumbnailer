use std::error::Error;
use std::fmt::{Display, Formatter, Result};

#[derive(Debug, Clone)]
pub struct InvalidContentType {
    pub content_type: String,
}

impl Error for InvalidContentType {}

impl Display for InvalidContentType {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(
            f,
            concat!(
                "File is not a valid Nitendo DS .nds or Nintendo 3DS .cia/.smdh/.3dsx file\n",
                "Found content type: {}",
            ),
            self.content_type,
        )
    }
}
