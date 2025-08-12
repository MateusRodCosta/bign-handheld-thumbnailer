mod args;
mod error;
mod n3ds;
mod nds;
mod utils;

use image::DynamicImage;
use n3ds::structures::SMDHIcon;
use nds::extract_nds_banner;
use pico_args::Arguments;
use std::fs::File;
use std::process::ExitCode;
use utils::get_mime_type;

use crate::{
    args::{ThumbnailerCommand, ThumbnailerFileParams},
    error::ThumbnailerError,
};

fn main() -> ExitCode {
    let mut args = Arguments::from_env();

    if let Err(e) =
        ThumbnailerCommand::try_from(&mut args).and_then(|cmd| bign_handheld_thumbnailer(cmd))
    {
        eprintln!("{e}");
        return ExitCode::FAILURE;
    }

    ExitCode::SUCCESS
}

fn bign_handheld_thumbnailer(cmd: ThumbnailerCommand) -> Result<(), ThumbnailerError> {
    match cmd {
        ThumbnailerCommand::ShowVersion => show_version(),
        ThumbnailerCommand::GenerateThumbnail(file_params) => generate_thumbnail(file_params),
    }
}

fn show_version() -> Result<(), ThumbnailerError> {
    const NAME: &str = env!("CARGO_PKG_NAME");
    const VERSION: &str = env!("CARGO_PKG_VERSION");

    println!("{NAME} v{VERSION}");
    Ok(())
}

fn generate_thumbnail(file_params: ThumbnailerFileParams) -> Result<(), ThumbnailerError> {
    if file_params.is_dry_run {
        eprintln!("Dry run mode, extracted icon will not be saved to a file!");
    }

    let path = file_params.input_file.as_path();

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

    let mime_type = get_mime_type(path)?;
    let mut input = File::open(path)?;

    const MIME_TYPE_NDS: &str = "application/x-nintendo-ds-rom";
    const MIME_TYPE_N3DS_CIA: &str = "application/x-ctr-cia";
    const MIME_TYPE_N3DS_SMDH: &str = "application/x-ctr-smdh";
    const MIME_TYPE_N3DS_3DSX: &str = "application/x-ctr-3dsx";
    const MIME_TYPE_N3DS_3DSX_GENERIC: &str = "application/x-nintendo-3ds-executable";
    const MIME_TYPE_N3DS_CXI: &str = "application/x-ctr-cxi";
    const MIME_TYPE_N3DS_CCI: &str = "application/x-ctr-cci";
    const MIME_TYPE_N3DS_CCI_GENERIC: &str = "application/x-nintendo-3ds-rom";

    let img = match &mime_type[..] {
        MIME_TYPE_NDS => extract_nds_banner(&mut input)?.icon,
        MIME_TYPE_N3DS_CIA => SMDHIcon::from_cia(&mut input)?.large_icon,
        MIME_TYPE_N3DS_SMDH => SMDHIcon::from_smdh(&mut input)?.large_icon,
        MIME_TYPE_N3DS_3DSX | MIME_TYPE_N3DS_3DSX_GENERIC => {
            SMDHIcon::from_n3dsx(&mut input)?.large_icon
        }
        MIME_TYPE_N3DS_CXI => SMDHIcon::from_cxi(&mut input)?.large_icon,
        MIME_TYPE_N3DS_CCI | MIME_TYPE_N3DS_CCI_GENERIC => {
            SMDHIcon::from_cci(&mut input)?.large_icon
        }
        _ => return Err(ThumbnailerError::IncompatibleMimeType(mime_type)),
    };

    // Whether to skip saving file
    if file_params.is_dry_run {
        return Ok(());
    }
    let Some(output) = file_params.output_file.as_deref() else {
        eprintln!("No output path, not saving any icon.");
        return Ok(());
    };

    // Whether to do optional scaling or save as-is
    let img = if let Some(size) = file_params.size {
        DynamicImage::ImageRgba8(img).resize(size, size, image::imageops::FilterType::Lanczos3)
    } else {
        DynamicImage::ImageRgba8(img)
    };

    img.save_with_format(output, image::ImageFormat::Png)?;
    Ok(())
}
