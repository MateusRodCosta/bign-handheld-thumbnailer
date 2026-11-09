mod args;
mod error;
mod n3ds;
mod nds;
mod utils;

use std::fs::File;
use std::io::{Read, Seek};
use std::process::ExitCode;

use pico_args::Arguments;
use image::{
    imageops::{self, FilterType},
    RgbaImage,
};

use crate::args::{ThumbnailerCommand, ThumbnailerFileParams};
use crate::error::ThumbnailerError;
use crate::utils::{RomFormat, get_rom_format};

use crate::nds::extract_nds_banner;
use crate::n3ds::structures::SMDHIcon;

fn main() -> ExitCode {
    let mut args = Arguments::from_env();

    if let Err(e) = ThumbnailerCommand::try_from(&mut args).and_then(bign_handheld_thumbnailer) {
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

// MRC: Please excuse the use of AI here, previously I was always using Lanczos3 when resizing the icons
// Since it didn't turn out as good and since it may have been using too musch CPU for not enough benefit,
// I asked Gemini to determine the best way to resize the game icons by making the best usage of the available filters
// Assume this would be better than anything I could so myself even if the AI output is flawed
fn resize_icon(img: RgbaImage, target_size: Option<u32>, format: RomFormat) -> RgbaImage {
    let Some(size) = target_size else {
        return img;
    };

    let width = img.width(); // Icons are always square (32x32 or 48x48)
    if size == width || size == 0 {
        return img;
    }

    let filter = match (format, size) {
        // NDS is 32x32 pixel art with 1-bit alpha: always use Nearest on integer multiples (64, 128, 256)
        (RomFormat::Nds, s) if s > width && s % width == 0 => FilterType::Nearest,
        // Any format at exact 2x size (e.g., 32->64 or 48->96): Nearest keeps crisp edges
        (_, s) if s == width * 2 => FilterType::Nearest,
        // Small non-integer scale (up to 2x) or downscaling: simple bilinear filter
        (_, s) if s <= width * 2 => FilterType::Triangle,
        // Large scale (> 2x, e.g., 48->128 or 48->256): Bicubic preserves quality without Lanczos ringing
        _ => FilterType::CatmullRom,
    };

    imageops::resize(&img, size, size, filter)
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

    let mut input = File::open(path)?;

    let mut header_bytes = [0u8; 512];
    input.read_exact(&mut header_bytes)?;

    input.rewind()?;

    let rom_format = get_rom_format(path, &header_bytes)?;

    let img = match rom_format {
        RomFormat::Nds => extract_nds_banner(&mut input)?.icon,
        RomFormat::N3dsCia => SMDHIcon::from_cia(&mut input)?.large_icon,
        RomFormat::N3dsSmdh => SMDHIcon::from_smdh(&mut input)?.large_icon,
        RomFormat::N3ds3dsx => SMDHIcon::from_n3dsx(&mut input)?.large_icon,
        RomFormat::N3dsCxi => SMDHIcon::from_cxi(&mut input)?.large_icon,
        RomFormat::N3dsCci => SMDHIcon::from_cci(&mut input)?.large_icon,
    };

    // Whether to skip saving file
    if file_params.is_dry_run {
        return Ok(());
    }
    let Some(output) = file_params.output_file.as_deref() else {
        eprintln!("No output path, not saving any icon.");
        return Ok(());
    };

    let img = resize_icon(img, file_params.size, rom_format);
    img.save_with_format(output, image::ImageFormat::Png)?;
    Ok(())
}
