use std::path::PathBuf;

use pico_args::Arguments;

use crate::error::ThumbnailerError;

#[derive(Debug)]
pub enum ThumbnailerCommand {
    ShowVersion,
    GenerateThumbnail(ThumbnailerFileParams),
}

impl TryFrom<&mut Arguments> for ThumbnailerCommand {
    type Error = ThumbnailerError;

    fn try_from(args: &mut Arguments) -> Result<Self, Self::Error> {
        if args.contains("--version") {
            return Ok(Self::ShowVersion);
        }

        Ok(Self::GenerateThumbnail(ThumbnailerFileParams::try_from(
            args,
        )?))
    }
}

#[derive(Debug)]
pub struct ThumbnailerFileParams {
    pub is_dry_run: bool,
    pub size: Option<u32>,
    pub input_file: PathBuf,
    pub output_file: Option<PathBuf>,
}

impl TryFrom<&mut Arguments> for ThumbnailerFileParams {
    type Error = ThumbnailerError;

    fn try_from(args: &mut Arguments) -> Result<Self, Self::Error> {
        let is_dry_run = args.contains("-n");

        let size = args.opt_value_from_str("-s")?;

        let input_file = args.free_from_str()?;

        let output_file = if is_dry_run {
            None
        } else {
            Some(args.free_from_str()?)
        };

        Ok(Self {
            is_dry_run,
            size,
            input_file,
            output_file,
        })
    }
}
