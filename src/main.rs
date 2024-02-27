mod generic_errors;
mod n3ds;
mod nds;

use clap::Parser;
use gdk_pixbuf::InterpType;
use generic_errors::*;
use n3ds::{extract_n3ds_cia_data, extract_n3ds_smdh_data};
use nds::extract_nds_data;
use std::path::Path;

#[derive(Debug, Parser)]
struct ThumbnailerArgs {
    #[arg(short = 's')]
    size: i32,
    input_file: std::path::PathBuf,
    output_file: std::path::PathBuf,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = ThumbnailerArgs::parse();

    let input = Path::new(&args.input_file);
    let output = Path::new(&args.output_file);

    let content_type = gio::functions::content_type_guess(Some(input), &[]);
    let mime_type = match gio::functions::content_type_get_mime_type(&content_type.0) {
        Some(x) => x,
        None => return Err(Box::new(InvalidMimeType)),
    };

    /* There are currently two supported file types:
     * .nds roms, indicated by the application/x-nintendo-ds-rom mime type
     * and .cia files, indicated by the application/x-ctr-cia mime type
     *
     * Note that application/x-ctr-cia is the same mime type used by Citra
     * and might not be defined on the user system
     * Therefore .cia support might require shipping .cia mime type support
     */

    // You might want to check https://github.com/citra-emu/citra/blob/master/dist/citra.xml
    // for the Nintendo 3DS-related mime types as defined by the Citra emulator

    let pixbuf = match &mime_type.to_string()[..] {
        "application/x-nintendo-ds-rom" => extract_nds_data(&input)?.get_icon().to_owned(),
        "application/x-ctr-cia" => extract_n3ds_cia_data(&input)?
            .get_smdh_content()
            .get_large_icon()
            .to_owned(),
        "application/x-ctr-smdh" => extract_n3ds_smdh_data(&input)?.get_large_icon().to_owned(),
        _ => return Err(Box::new(InvalidMimeType)),
    };

    let new_size = args.size;
    let pixbuf = match pixbuf.scale_simple(new_size, new_size, InterpType::Bilinear) {
        Some(p) => p,
        None => pixbuf.to_owned(),
    };

    pixbuf.savev(output, "png", &[])?;

    Ok(())
}
