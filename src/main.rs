mod args;
mod main_errors;
mod n3ds;
mod nds;
mod utils;

use args::ThumbnailerArgs;
use image::DynamicImage;
use main_errors::MainError;
use n3ds::structures::SMDHIcon;
use nds::extract_nds_banner;
use pico_args::Arguments;
use std::fs::File;
use std::{path::Path, process::ExitCode};

fn main() -> ExitCode {
    let args = Arguments::from_env();

    let args = match ThumbnailerArgs::try_from(&args) {
        Err(e) => {
            eprintln!("{e}");
            return ExitCode::FAILURE;
        }
        Ok(args) => args,
    };
    if let Err(e) = bign_handheld_thumbnailer(&args) {
        eprintln!("{e}");
        return ExitCode::FAILURE;
    }

    ExitCode::SUCCESS
}

fn bign_handheld_thumbnailer(args: &ThumbnailerArgs) -> Result<(), MainError> {
    if args.show_version {
        const NAME: &str = env!("CARGO_PKG_NAME");
        const VERSION: &str = env!("CARGO_PKG_VERSION");

        println!("{NAME} v{VERSION}");

        return Ok(());
    }

    // if it's not a `--version` command, then just extract the file params directly
    let file_params = args.file_params().unwrap();

    if file_params.is_dry_run {
        println!("Dry run mode, extracted icon will not be saved to a file!")
    }

    let input = Path::new(&file_params.input_file);

    let content_type = utils::content_type_guess(&Some(input), None);
    let content_type = content_type.0.as_str();

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
    let img = match content_type {
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
        _ => return Err(MainError::InvalidContentType(content_type.to_string())),
    };

    // Whether to do optional scaling

    if file_params.is_dry_run {
        return Ok(());
    }

    let output = match &file_params.output_file {
        Some(data) => Path::new(data),
        None => {
            println!("No output path, not saving any icon.");
            return Ok(());
        }
    };
    match file_params.size {
        None => {
            img.save_with_format(output, image::ImageFormat::Png)?;
            Ok(())
        }
        Some(size) => {
            let img = DynamicImage::ImageRgba8(img).resize(
                size,
                size,
                image::imageops::FilterType::CatmullRom,
            );
            img.save_with_format(output, image::ImageFormat::Png)?;
            Ok(())
        }
    }
}
