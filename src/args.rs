use pico_args::Arguments;

use crate::main_errors::MainError;

#[derive(Debug, Clone)]
pub struct ThumbnailerArgs {
    pub show_version: bool,
    pub file_params: Option<ThumbnailerArgsFileParams>,
}

impl TryFrom<&Arguments> for ThumbnailerArgs {
    type Error = MainError;

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
    pub fn file_params(&self) -> Option<ThumbnailerArgsFileParams> {
        self.file_params.clone()
    }
}

#[derive(Debug, Clone)]
pub struct ThumbnailerArgsFileParams {
    pub input_file: std::path::PathBuf,
    pub is_dry_run: bool,
    pub output_file: Option<std::path::PathBuf>,
    pub size: Option<u32>,
}

impl TryFrom<&Arguments> for ThumbnailerArgsFileParams {
    type Error = MainError;

    fn try_from(arguments: &Arguments) -> Result<Self, Self::Error> {
        let mut args = arguments.clone();

        let input_file = args.value_from_str("-i")?;

        let is_dry_run = args.contains("-n");

        let output_file = args.opt_value_from_str("-o")?;

        let size = args.opt_value_from_str("-s")?;

        Ok(ThumbnailerArgsFileParams {
            size,
            input_file,
            is_dry_run,
            output_file,
        })
    }
}
