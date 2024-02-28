mod nds_banner_structure;
mod nds_parsing_errors;

use super::generic_errors::ParsingErrorByteOutOfRange;
use clap::error::Result;
use gdk_pixbuf::{Colorspace, Pixbuf};
use gio::{prelude::FileExt, Cancellable, File};
use nds_banner_structure::*;
use nds_parsing_errors::*;
use std::path::Path;

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

pub fn extract_nds_banner(
    file_path: &Path,
) -> Result<NDSBannerDetails, Box<dyn std::error::Error>> {
    let f = File::for_path(file_path);

    let content = f.load_bytes(Cancellable::NONE)?;
    let content = content.0;

    let banner_offset = &content.get(0x068..0x068 + 4);
    let banner_offset = match banner_offset {
        None => {
            return Err(Box::new(ParsingErrorByteOutOfRange::new(
                String::from("Get banner offset"),
                0x068 + 4,
                content.len(),
            )))
        }
        Some(x) => x.to_owned(),
    };

    let banner_offset = u32::from_le_bytes(banner_offset[..].try_into()?);
    let banner_offset = banner_offset as usize;
    let banner_size = 0xA00;

    let banner_bytes = &content.get(banner_offset..banner_offset + banner_size);
    let banner_bytes = match banner_bytes {
        None => {
            return Err(Box::new(ParsingErrorByteOutOfRange::new(
                String::from("Get banner data"),
                banner_offset + banner_size,
                content.len(),
            )))
        }
        Some(x) => x.to_owned(),
    };

    let icon_version_bytes = &banner_bytes[..2];
    let icon_version = extract_icon_version(icon_version_bytes)?;

    let logo_bytes = &banner_bytes[0x0020..0x0020 + 0x200];
    let palette_bytes = &banner_bytes[0x0220..0x0220 + 0x20];
    let palette = extract_palette_colors(&palette_bytes)?;

    let pixbuf = match generate_nds_pixbuf(&logo_bytes, &palette) {
        Some(p) => p,
        None => return Err(Box::new(UnableToExtractNDSIcon)),
    };

    let banner_details = NDSBannerDetails::new(icon_version, pixbuf);

    Ok(banner_details)
}

fn extract_icon_version(
    icon_version_bytes: &[u8],
) -> Result<NDSIconVersion, Box<dyn std::error::Error>> {
    let found_icon_version = u16::from_le_bytes(icon_version_bytes[..].try_into()?);

    /*
     * The NDS icon versions map to this:
     *
     * 0001h = Original
     * 0002h = With Chinese Title
     * 0003h = With Chinese+Korean Titles
     * 0103h = With Chinese+Korean Titles and animated DSi icon
     *
     * Do note that the animated DSi icon is not supported by this thumbnailer
     */

    match found_icon_version {
        0x0001 => Ok(NDSIconVersion::V1),
        0x0002 => Ok(NDSIconVersion::V2),
        0x0003 => Ok(NDSIconVersion::V3),
        0x0103 => Ok(NDSIconVersion::DSi),
        _ => Err(Box::new(UnknownOrInvalidNDSIconVersion::new(
            found_icon_version,
        ))),
    }
}

fn extract_palette_colors(
    palette_raw: &[u8],
) -> Result<Vec<PaletteColor>, Box<dyn std::error::Error>> {
    let colors_raw: Vec<[u8; 2]> = palette_raw
        .chunks_exact(2)
        .map(|chunk| <[u8; 2]>::try_from(chunk))
        .collect::<Result<Vec<_>, _>>()?;

    let colors_converted: Vec<(u8, u8, u8)> = colors_raw
        .iter()
        .map(convert_color)
        .collect::<Result<Vec<_>, _>>()?;

    let palette_colors: Vec<PaletteColor> = colors_converted
        .iter()
        .enumerate()
        .map(|(i, palette_color)| {
            PaletteColor::new(
                palette_color.0,
                palette_color.1,
                palette_color.2,
                if i == 0 { 0x00 } else { 0xFF },
            )
        })
        .collect();

    Ok(palette_colors)
}

fn convert_color(color_bytes: &[u8; 2]) -> Result<(u8, u8, u8), Box<dyn std::error::Error>> {
    /*
     * The NDS palette uses RGB555 for color encoding but we need RGB888
     * So, each individual color must be isolated and converted to RGB888
     */

    let converted_color = u16::from_le_bytes(color_bytes.to_owned());

    let r = u8::try_from((converted_color & 0x001F) << 3)?;
    let g = u8::try_from((converted_color & 0x03E0) >> 2)?;
    let b = u8::try_from((converted_color & 0x7C00) >> 7)?;

    Ok((r, g, b))
}

fn generate_nds_pixbuf(logo_data: &[u8], palette: &[PaletteColor]) -> Option<Pixbuf> {
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
                    pixbuf.put_pixel(
                        x * 2 + 8 * i,
                        y + 8 * j,
                        lower.get_r(),
                        lower.get_g(),
                        lower.get_b(),
                        lower.get_a(),
                    );

                    let upper_index = usize::from((logo_data[pos] & 0xF0) >> 4);
                    let upper = &palette[upper_index];
                    pixbuf.put_pixel(
                        x * 2 + 1 + 8 * i,
                        y + 8 * j,
                        upper.get_r(),
                        upper.get_g(),
                        upper.get_b(),
                        upper.get_a(),
                    );

                    pos += 1;
                }
            }
        }
    }

    Some(pixbuf)
}
