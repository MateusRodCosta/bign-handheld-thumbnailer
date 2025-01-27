use std::path::{Path, PathBuf};

use pico_args::Arguments;

use crate::error::ThumbnailerError;

#[derive(Debug)]
pub struct ThumbnailerArgs {
    show_version: bool,
    file_params: Option<ThumbnailerArgsFileParams>,
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

impl ThumbnailerArgs {
    pub fn show_version(&self) -> bool {
        self.show_version
    }
    pub fn file_params(&self) -> Option<&ThumbnailerArgsFileParams> {
        self.file_params.as_ref()
    }
}

#[derive(Debug)]
pub struct ThumbnailerArgsFileParams {
    is_dry_run: bool,
    size: Option<u32>,
    input_file: PathBuf,
    output_file: Option<PathBuf>,
}

impl TryFrom<&Arguments> for ThumbnailerArgsFileParams {
    type Error = ThumbnailerError;

    fn try_from(arguments: &Arguments) -> Result<Self, Self::Error> {
        let mut args = arguments.clone();

        let is_dry_run = args.contains("-n");

        let size = args.opt_value_from_str("-s")?;

        let input_file = args.free_from_str()?;

        let output_file = if !is_dry_run {
            Some(args.free_from_str()?)
        } else {
            None
        };

        Ok(ThumbnailerArgsFileParams {
            is_dry_run,
            size,
            input_file,
            output_file,
        })
    }
}

impl ThumbnailerArgsFileParams {
    pub fn is_dry_run(&self) -> bool {
        self.is_dry_run
    }
    pub fn size(&self) -> Option<u32> {
        self.size
    }
    pub fn input_file(&self) -> &Path {
        &self.input_file
    }
    pub fn output_file(&self) -> Option<&Path> {
        self.output_file.as_deref()
    }
}
