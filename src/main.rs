mod generic_errors;
mod n3ds;
mod nds;
mod utils;

use clap::Parser;
use gdk_pixbuf::InterpType;
use generic_errors::*;
use n3ds::{extract_n3ds_3dsx_data, extract_n3ds_cia_data, extract_n3ds_smdh_data};
use nds::extract_nds_data;
use std::{path::Path, process::ExitCode};

#[derive(Debug, Parser)]
struct ThumbnailerArgs {
    #[arg(short = 's')]
    size: i32,
    input_file: std::path::PathBuf,
    output_file: std::path::PathBuf,
}

fn main() -> ExitCode {
    let args = ThumbnailerArgs::parse();

    if let Err(e) = bign_handheld_thumbnailer(&args) {
        eprintln!("bign-handheld-thumbnailer error: {}", e);
        return ExitCode::FAILURE;
    };

    ExitCode::SUCCESS
}

fn bign_handheld_thumbnailer(args: &ThumbnailerArgs) -> Result<(), Box<dyn std::error::Error>> {
    let input = Path::new(&args.input_file);
    let output = Path::new(&args.output_file);
    let size = args.size;

    let content_type = utils::content_type_guess(Some(input), None);
    let content_type = content_type.0.to_string();

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

    let pixbuf = match &content_type[..] {
        "application/x-nintendo-ds-rom" => extract_nds_data(&input)?.get_icon().to_owned(),
        "application/x-ctr-cia" => extract_n3ds_cia_data(&input)?
            .get_smdh_content()
            .get_large_icon()
            .to_owned(),
        "application/x-ctr-smdh" => extract_n3ds_smdh_data(&input)?.get_large_icon().to_owned(),
        "application/x-ctr-3dsx" | "application/x-nintendo-3ds-executable" => {
            extract_n3ds_3dsx_data(&input)?
                .get_smdh_content()
                .get_large_icon()
                .to_owned()
        }
        _ => return Err(Box::new(InvalidContentType::new(content_type))),
    };

    let pixbuf = match pixbuf.scale_simple(size, size, InterpType::Bilinear) {
        Some(p) => p,
        None => pixbuf.to_owned(),
    };

    pixbuf.savev(output, "png", &[])?;

    Ok(())
}
