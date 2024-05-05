use gio::ffi;
use gio::glib::translate::{from_glib, from_glib_full, ToGlibPtr};

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

// Workaround to https://github.com/gtk-rs/gtk-rs-core/issues/1257
pub fn content_type_guess(
    filename: &Option<impl AsRef<std::path::Path>>,
    data: Option<&[u8]>,
) -> (gio::glib::GString, bool) {
    let data_size = data.map_or(0, <[u8]>::len);
    unsafe {
        let mut result_uncertain = std::mem::MaybeUninit::uninit();
        let ret = from_glib_full(ffi::g_content_type_guess(
            filename
                .as_ref()
                .map(std::convert::AsRef::as_ref)
                .to_glib_none()
                .0,
            data.to_glib_none().0,
            data_size as _,
            result_uncertain.as_mut_ptr(),
        ));
        (ret, from_glib(result_uncertain.assume_init()))
    }
}
