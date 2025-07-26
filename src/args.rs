use std::path::{Path, PathBuf};

use pico_args::Arguments;

use crate::error::ThumbnailerError;

#[derive(Debug)]
pub struct ThumbnailerArgs {
    pub show_version: bool,
    pub file_params: Option<ThumbnailerArgsFileParams>,
}

impl TryFrom<&Arguments> for ThumbnailerArgs {
    type Error = ThumbnailerError;

    fn try_from(arguments: &Arguments) -> Result<Self, Self::Error> {
        let mut args = arguments.clone();

        let show_version = args.contains("--version");
        let file_params = if show_version {
            None
        } else {
            Some(ThumbnailerArgsFileParams::try_from(&args)?)
        };

        Ok(ThumbnailerArgs {
            show_version,
            file_params,
        })
    }
}

#[derive(Debug)]
pub struct ThumbnailerArgsFileParams {
    pub is_dry_run: bool,
    pub size: Option<u32>,
    pub input_file: PathBuf,
    pub output_file: Option<PathBuf>,
}

impl TryFrom<&Arguments> for ThumbnailerArgsFileParams {
    type Error = ThumbnailerError;

    fn try_from(arguments: &Arguments) -> Result<Self, Self::Error> {
        let mut args = arguments.clone();

        let is_dry_run = args.contains("-n");

        let size = args.opt_value_from_str("-s")?;

        let input_file = args.free_from_str()?;

        let output_file = if is_dry_run {
            None
        } else {
            Some(args.free_from_str()?)
        };

        Ok(ThumbnailerArgsFileParams {
            is_dry_run,
            size,
            input_file,
            output_file,
        })
    }
}
