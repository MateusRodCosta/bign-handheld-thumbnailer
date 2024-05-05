mod nds_banner_structure;
pub mod nds_parsing_errors;

use self::nds_parsing_errors::NDSParsingError;
use super::utils::bgr555::Bgr555;
use gdk_pixbuf::{Colorspace, Pixbuf};
use nds_banner_structure::*;
use std::io::{Read, Seek, SeekFrom};

/*
 * Consider the following links for more info about the .nds file structure:
 *
 * NDS header: https://problemkaputt.de/gbatek.htm#dscartridgeheader
 * NDS banner: https://problemkaputt.de/gbatek.htm#dscartridgeicontitle
 *
 * Do note that while animated icons might be available if the version of the icon
 * matches the NDSIconVersion::DSi version, the static icon will be used instead
 * as the thumbnailer specification doesn't support animations.
*/

pub fn extract_nds_banner<T: Read + Seek>(f: &mut T) -> Result<NDSBannerDetails, NDSParsingError> {
    f.seek(SeekFrom::Start(0x068))?;

    let mut banner_offset = [0u8; 4];
    f.read_exact(&mut banner_offset)?;
    let banner_offset = u32::from_le_bytes(banner_offset);

    f.seek(SeekFrom::Start(banner_offset as u64))?;
    const BANNER_SIZE: usize = 0x240;
    let mut banner_bytes = [0u8; BANNER_SIZE];
    f.read_exact(&mut banner_bytes)?;

    let icon_version = u16::from_le_bytes(banner_bytes[..2].try_into().unwrap());
    let icon_version = NDSIconVersion::try_from(icon_version)?;

    let logo_bytes: &[u8; 0x200] = &banner_bytes[0x020..0x220].try_into().unwrap();
    let palette_bytes: &[u8; 0x20] = &banner_bytes[0x220..0x240].try_into().unwrap();
    let palette = extract_palette_colors(palette_bytes);

    let Some(pixbuf) = generate_nds_pixbuf(logo_bytes, &palette) else {
        return Err(NDSParsingError::UnableToExtractNDSIcon);
    };

    let banner_details = NDSBannerDetails::new(icon_version, pixbuf);

    Ok(banner_details)
}

fn extract_palette_colors(palette_raw: &[u8; 0x20]) -> Vec<PaletteColor> {
    // this unwrap will never fail: there's even length input.
    let colors_555 = palette_raw
        .chunks_exact(2)
        .map(|chunk| u16::from_le_bytes(chunk.try_into().unwrap()));

    // this unwrap will never fail:
    let colors_converted = colors_555.map(|color| Bgr555::try_from(color).unwrap());

    let mut palette_colors: Vec<PaletteColor> = colors_converted
        .map(|palette_color| {
            PaletteColor::new(
                palette_color.r(),
                palette_color.g(),
                palette_color.b(),
                0xFF,
            )
        })
        .collect();
    palette_colors[0] = PaletteColor {
        a: 0x00,
        ..palette_colors[0]
    };

    palette_colors
}

fn generate_nds_pixbuf(logo_data: &[u8; 0x200], palette: &[PaletteColor]) -> Option<Pixbuf> {
    let pixbuf = Pixbuf::new(Colorspace::Rgb, true, 8, 32, 32)?;

    /*
     * The NDS icon is 32x32 px divided into 8x8 tiles
     * Each byte in the logo data represents the color data for 2 pixels:
     * The lower 4 bits represent the pallete index for one pixel,
     * the higher 4 bits the same but for a second pixel
     */

    /*
     * The following code is oxided from the existing gnome-nds-thumbnailer at
     * https://gitlab.gnome.org/GNOME/gnome-nds-thumbnailer/-/blob/master/gnome-nds-thumbnailer.c?ref_type=heads#L73
     */

    let mut pos = 0;
    for j in 0..4 {
        for i in 0..4 {
            for y in 0..8 {
                for x in 0..4 {
                    let lower_index = usize::from(logo_data[pos] & 0x0F);
                    let lower = &palette[lower_index];
                    pixbuf.put_pixel(x * 2 + 8 * i, y + 8 * j, lower.r, lower.g, lower.b, lower.a);

                    let upper_index = usize::from((logo_data[pos] & 0xF0) >> 4);
                    let upper = &palette[upper_index];
                    pixbuf.put_pixel(
                        x * 2 + 1 + 8 * i,
                        y + 8 * j,
                        upper.r,
                        upper.g,
                        upper.b,
                        upper.a,
                    );

                    pos += 1;
                }
            }
        }
    }

    Some(pixbuf)
}
