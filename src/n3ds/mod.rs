mod n3ds_parsing_errors;
mod n3ds_structures;

use gdk_pixbuf::Pixbuf;
use gio::{prelude::FileExt, Cancellable, File};
use n3ds_parsing_errors::*;
use n3ds_structures::*;
use rgb565::Rgb565;
use std::path::Path;

/*
 * Currently only .cia files are supported.
 *
 * Consider the following links for more info about the CIA and SMDH structure:
 *
 * On GBATEK:
 * CIA: https://problemkaputt.de/gbatek.htm#3dsfilestitleinstallationarchivecia
 * SMDH: https://problemkaputt.de/gbatek.htm#3dsfilesvideoiconssmdh
 *
 * On 3dbrew:
 * CIA: https://www.3dbrew.org/wiki/CIA
 * SMDH: https://www.3dbrew.org/wiki/SMDH
 *
 * Do note that the Meta section might or might not be present on .cia files.
*/

pub fn extract_3ds_data(file_path: &Path) -> Result<CIAMetaContent, Box<dyn std::error::Error>> {
    let f = File::for_path(file_path);

    let content = f.load_bytes(None::<&Cancellable>)?;
    let content = content.0;

    let meta = fetch_meta_section(&content)?;

    Ok(meta)
}

fn fetch_meta_section(content: &[u8]) -> Result<CIAMetaContent, Box<dyn std::error::Error>> {

    /*
     * The meta section isn't in a fixed place and is located after a bunch of sections whose
     * size can vary, therefore it's needed to at the very last fetch the other sizes and
     * take the padding into account
     */

    let certificate_chain_size = match content.get(0x08..0x08 + 4) {
        Some(c) => c,
        None => {
            return Err(Box::new(N3DSParsingErrorByteOutOfRange {
                attempted: 0x08 + 4,
                maximum_size: content.len(),
                step: String::from("Get certificate chain size"),
            }))
        }
    };
    let certificate_chain_size = u32::from_le_bytes(certificate_chain_size[..].try_into()?);

    let ticket_size = match content.get(0x0C..0x0C + 4) {
        Some(c) => c,
        None => {
            return Err(Box::new(N3DSParsingErrorByteOutOfRange {
                attempted: 0x0C + 4,
                maximum_size: content.len(),
                step: String::from("Get ticket size"),
            }))
        }
    };
    let ticket_size = u32::from_le_bytes(ticket_size[..].try_into()?);

    let tmd_size = match content.get(0x10..0x10 + 4) {
        Some(c) => c,
        None => {
            return Err(Box::new(N3DSParsingErrorByteOutOfRange {
                attempted: 0x10 + 4,
                maximum_size: content.len(),
                step: String::from("Get TMD size"),
            }))
        }
    };
    let tmd_size = u32::from_le_bytes(tmd_size[..].try_into()?);

    let meta_size = match content.get(0x14..0x14 + 4) {
        Some(c) => c,
        None => {
            return Err(Box::new(N3DSParsingErrorByteOutOfRange {
                attempted: 0x14 + 4,
                maximum_size: content.len(),
                step: String::from("Get Meta size"),
            }))
        }
    };
    let meta_size = match u32::from_le_bytes(meta_size[..].try_into()?) {
        0 => N3DSCIAMeta::MetaNone,
        8 => N3DSCIAMeta::MetaCVerUSA,
        0x200 => N3DSCIAMeta::MetaDummy,
        0x3AC0 => N3DSCIAMeta::MetaPresent,
        _ => return Err(Box::new(N3DSParsingErrorMetaNotPresentOrInvalidSize)),
    };

    let content_size = match content.get(0x18..0x18 + 8) {
        Some(c) => c,
        None => {
            return Err(Box::new(N3DSParsingErrorByteOutOfRange {
                attempted: 0x18 + 8,
                maximum_size: content.len(),
                step: String::from("Get content size"),
            }))
        }
    };
    let content_size = u64::from_le_bytes(content_size[..].try_into()?);

    let meta_size: u32 = match meta_size {
        N3DSCIAMeta::MetaPresent => 0x3AC0,
        _ => return Err(Box::new(N3DSParsingErrorMetaNotPresentOrInvalidSize)),
    };

    let certificate_chain_size_with_padding = certificate_chain_size.div_ceil(0x40) * 0x40;
    let ticket_size_with_padding = ticket_size.div_ceil(0x40) * 0x40;
    let tmd_size_with_padding = tmd_size.div_ceil(0x40) * 0x40;
    let meta_size_with_padding = meta_size.div_ceil(0x40) * 0x40;
    let content_size_with_padding = content_size.div_ceil(0x40) * 0x40;

    let content_size_with_padding: u32 = u32::try_from(content_size_with_padding)?;
    let offset = certificate_chain_size_with_padding
        + ticket_size_with_padding
        + tmd_size_with_padding
        + content_size_with_padding;
    let offset = offset as usize;
    let meta = match content.get(0x2040 + offset..0x2040 + offset + 0x3AC0) {
        Some(c) => c,
        None => {
            return Err(Box::new(
                n3ds_parsing_errors::N3DSParsingErrorByteOutOfRange {
                    attempted: 0x18 + 8,
                    maximum_size: content.len(),
                    step: String::from("Extracting meta section"),
                },
            ))
        }
    };

    let smdh_bytes = match meta.get(0x0400..0x0400 + 0x36c0) {
        Some(c) => c,
        None => {
            return Err(Box::new(N3DSParsingErrorByteOutOfRange {
                attempted: 0x0400 + 0x36c0,
                maximum_size: meta.len(),
                step: String::from("Extract SMDH"),
            }))
        }
    };

    let smdh_content = extract_smdh(smdh_bytes)?;
    let meta_content = CIAMetaContent::new(smdh_content);

    Ok(meta_content)
}

fn extract_smdh(smdh_bytes: &[u8]) -> Result<SMDHContent, Box<dyn std::error::Error>> {
    let sdmh_magic = &smdh_bytes[..4];
    let sdmh_magic_str = String::from_utf8(sdmh_magic.to_vec())?;

    if sdmh_magic_str != "SMDH" {
        return Err(Box::new(N3DSParsingErrorSMDHMagicNotFound));
    }

    let large_icon_bytes = &smdh_bytes[0x24c0..0x24c0 + 0x1200];
    let large_icon = extract_large_icon(large_icon_bytes)?;

    let smdh = SMDHContent::new(large_icon);

    Ok(smdh)
}

fn extract_large_icon(large_icon_bytes: &[u8]) -> Result<Pixbuf, Box<dyn std::error::Error>> {
    let large_icon_colors: Vec<[u8; 2]> = large_icon_bytes
        .chunks_exact(2)
        .map(|chunk| <[u8; 2]>::try_from(chunk))
        .collect::<Result<Vec<_>, _>>()?;

    let large_icon_data: Vec<Rgb565> = large_icon_colors
        .iter()
        .map(|color| Rgb565::from_rgb565_le(color.to_owned()))
        .collect();

    let large_icon = match generate_n3ds_pixbuf(&large_icon_data) {
        Some(p) => p,
        None => return Err(Box::new(UnableToExtractN3DSIcon)),
    };

    Ok(large_icon)
}

fn generate_n3ds_pixbuf(large_icon_data: &[Rgb565]) -> Option<Pixbuf> {
    let pixbuf = Pixbuf::new(gdk_pixbuf::Colorspace::Rgb, true, 8, 48, 48)?;

    /*
     * The large 3DS icon is 48x48 px and divided in tiles according to Morton order
     * Each color will usually be RGB565 although it's not the only supported color enconding
     */

    /*
     * Due to the Morton order, the code for the coordinates of the pixels is oxided from
     * https://github.com/GEMISIS/SMDH-Creator/blob/master/SMDH-Creator/SMDH.cs#L255
     */

    let tile_order = [
        0, 1, 8, 9, 2, 3, 10, 11, 16, 17, 24, 25, 18, 19, 26, 27, 4, 5, 12, 13, 6, 7, 14, 15, 20,
        21, 28, 29, 22, 23, 30, 31, 32, 33, 40, 41, 34, 35, 42, 43, 48, 49, 56, 57, 50, 51, 58, 59,
        36, 37, 44, 45, 38, 39, 46, 47, 52, 53, 60, 61, 54, 55, 62, 63,
    ];

    let mut pos = 0;
    for tile_y in 0..6 {
        for tile_x in 0..6 {
            for k in 0..64 {
                let x = tile_order[k] & 0x7;
                let y = tile_order[k] >> 3;
                let coords = (x + (tile_x * 8), y + (tile_y * 8));

                let rgb = large_icon_data[pos].to_rgb888_components();
                let (r, g, b) = (rgb[0], rgb[1], rgb[2]);

                pixbuf.put_pixel(coords.0, coords.1, r, g, b, 0xFF);

                pos += 1;
            }
        }
    }

    Some(pixbuf)
}
