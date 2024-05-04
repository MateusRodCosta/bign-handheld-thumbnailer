mod generic_errors;
mod n3ds;
mod nds;
mod utils;

use gdk_pixbuf::InterpType;
use generic_errors::*;
use n3ds::n3ds_structures::SMDHIcon;
use nds::extract_nds_banner;
use pico_args::Arguments;
use std::fs::File;
use std::{path::Path, process::ExitCode};

#[derive(Debug)]
struct ThumbnailerArgs {
    size: i32,
    input_file: std::path::PathBuf,
    output_file: std::path::PathBuf,
}

fn main() -> ExitCode {
    let args = Arguments::from_env();

    let args = match get_thumbnailer_args(&args) {
        Ok(a) => a,
        Err(e) => {
            eprintln!(
                concat!("bign-handheld-thumbnailer: {}\n", "Error: {}"),
                ErrorParsingThumbnailerArguments { parsed_args: args },
                e
            );
            return ExitCode::FAILURE;
        }
    };

    if let Err(e) = bign_handheld_thumbnailer(&args) {
        eprintln!("bign-handheld-thumbnailer: {}", e);
        return ExitCode::FAILURE;
    };

    ExitCode::SUCCESS
}

fn get_thumbnailer_args(
    arguments: &Arguments,
) -> Result<ThumbnailerArgs, Box<dyn std::error::Error>> {
    let mut args = arguments.clone();

    let size = args.value_from_str("-s")?;
    let input_file = args.free_from_str()?;
    let output_file = args.free_from_str()?;

    Ok(ThumbnailerArgs {
        size,
        input_file,
        output_file,
    })
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

    let mut input = File::open(input)?;
    let pixbuf = match &content_type[..] {
        "application/x-nintendo-ds-rom" => extract_nds_banner(&mut input)?.get_icon(),
        "application/x-ctr-cia" => SMDHIcon::from_cia(&mut input)?.get_large_icon(),
        "application/x-ctr-smdh" => SMDHIcon::from_smdh(&mut input)?.get_large_icon(),
        "application/x-ctr-3dsx" | "application/x-nintendo-3ds-executable" => {
            SMDHIcon::from_n3dsx(&mut input)?.get_large_icon()
        }
        "application/x-ctr-cxi" => SMDHIcon::from_cxi(&mut input)?.get_large_icon(),
        "application/x-ctr-cci" | "application/x-nintendo-3ds-rom" => {
            SMDHIcon::from_cci(&mut input)?.get_large_icon()
        }
        _ => return Err(Box::new(InvalidContentType { content_type })),
    };

    let pixbuf = match pixbuf.scale_simple(size, size, InterpType::Bilinear) {
        Some(p) => p,
        None => pixbuf,
    };

    pixbuf.savev(output, "png", &[])?;

    Ok(())
}
