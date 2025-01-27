use std::path::Path;

use gio::{prelude::FileExt, Cancellable};

use crate::error::MimeTypeDetectionError;

#[derive(Debug)]
pub struct Rgb888 {
    r: u8,
    g: u8,
    b: u8,
}

impl Rgb888 {
    pub fn r(&self) -> u8 {
        self.r
    }
    pub fn g(&self) -> u8 {
        self.g
    }
    pub fn b(&self) -> u8 {
        self.b
    }
}

impl Rgb888 {
    pub fn from_bgr555_bytes(color_bytes: [u8; 2]) -> Self {
        /*
         * The NDS palette uses BGR555 for color encoding but we need RGB888
         * So, each individual color must be isolated and converted to RGB888
         */

        let color_bytes = u16::from_le_bytes(color_bytes);

        // Conversion code borrowed from
        // https://learn.microsoft.com/en-us/windows/win32/directshow/working-with-16-bit-rgb
        // with added swapping between Blue and Red

        let blue_value = u8::try_from((color_bytes & 0x7C00) >> 10).unwrap();
        let green_value = u8::try_from((color_bytes & 0x03E0) >> 5).unwrap();
        let red_value = u8::try_from(color_bytes & 0x001F).unwrap();

        let b = blue_value << 3;
        let g = green_value << 3;
        let r = red_value << 3;

        Rgb888 { r, g, b }
    }

    pub fn from_rgb565_bytes(color_bytes: [u8; 2]) -> Self {
        /*
         * The 3DS icon usually uses BGR555 for color encoding, although others are also supported,
         * but we need RGB888
         * So, each individual color must be isolated and converted to RGB888
         */

        let color_bytes = u16::from_le_bytes(color_bytes);

        // Conversion code borrowed from
        // https://learn.microsoft.com/en-us/windows/win32/directshow/working-with-16-bit-rgb

        let red_value = u8::try_from((color_bytes & 0xF800) >> 11).unwrap();
        let green_value = u8::try_from((color_bytes & 0x07E0) >> 5).unwrap();
        let blue_value = u8::try_from(color_bytes & 0x001F).unwrap();

        let r = red_value << 3;
        let g = green_value << 2;
        let b = blue_value << 3;

        Rgb888 { r, g, b }
    }
}

pub fn get_mime_type(input: &Path) -> Result<String, MimeTypeDetectionError> {
    let file = gio::File::for_path(input);
    let attrs = format!("{}", gio::FILE_ATTRIBUTE_STANDARD_CONTENT_TYPE);
    let file_info = file
        .query_info(
            attrs.as_str(),
            gio::FileQueryInfoFlags::NONE,
            Cancellable::NONE,
        )?;

    let mime_type = file_info
        .content_type()
        .and_then(|c| gio::functions::content_type_get_mime_type(&c))
        .ok_or(MimeTypeDetectionError::MimeTypeDetectionFailure)?;

    Ok(mime_type.to_string())
}
