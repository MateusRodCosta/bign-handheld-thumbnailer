#[derive(Debug, Clone, Copy)]
pub struct Bgr555(pub u16);

impl From<[u8; 2]> for Bgr555 {
    fn from(value: [u8; 2]) -> Self {
        Self {
            0: u16::from_le_bytes(value),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Rgb565(pub u16);

impl From<[u8; 2]> for Rgb565 {
    fn from(value: [u8; 2]) -> Self {
        Self {
            0: u16::from_le_bytes(value),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Rgb888 {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl From<Bgr555> for Rgb888 {
    fn from(value: Bgr555) -> Self {
        /*
         * The NDS palette uses BGR555 for color encoding but we need RGB888
         * So, each individual color must be isolated and converted to RGB888
         */

        let color_bytes = value.0;

        // Conversion code borrowed from
        // https://learn.microsoft.com/en-us/windows/win32/directshow/working-with-16-bit-rgb
        // with added swapping between Blue and Red

        let blue_value = u8::try_from((color_bytes & 0x7C00) >> 10).unwrap();
        let green_value = u8::try_from((color_bytes & 0x03E0) >> 5).unwrap();
        let red_value = u8::try_from(color_bytes & 0x001F).unwrap();

        let b = blue_value << 3;
        let g = green_value << 3;
        let r = red_value << 3;

        Self { r, g, b }
    }
}

impl From<Rgb565> for Rgb888 {
    fn from(value: Rgb565) -> Self {
        /*
         * The 3DS icon usually uses RGB565 for color encoding, although others are also supported,
         * but we need RGB888
         * So, each individual color must be isolated and converted to RGB888
         */

        let color_bytes = value.0;

        // Conversion code borrowed from
        // https://learn.microsoft.com/en-us/windows/win32/directshow/working-with-16-bit-rgb

        let red_value = u8::try_from((color_bytes & 0xF800) >> 11).unwrap();
        let green_value = u8::try_from((color_bytes & 0x07E0) >> 5).unwrap();
        let blue_value = u8::try_from(color_bytes & 0x001F).unwrap();

        let r = red_value << 3;
        let g = green_value << 2;
        let b = blue_value << 3;

        Self { r, g, b }
    }
}
