pub mod rgb888;

use std::path::Path;

use gio::{prelude::FileExt, Cancellable};

use crate::error::ThumbnailerError;

pub fn get_mime_type(input: &Path) -> Result<String, ThumbnailerError> {
    let file = gio::File::for_path(input);
    let attrs = gio::FILE_ATTRIBUTE_STANDARD_CONTENT_TYPE;
    let file_info = file.query_info(attrs, gio::FileQueryInfoFlags::NONE, Cancellable::NONE)?;

    let mime_type = file_info
        .content_type()
        .ok_or(ThumbnailerError::MimeTypeDetectionFailure)?;

    Ok(mime_type.to_string())
}
