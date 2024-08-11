use pico_args::Arguments;

use crate::error::Error;

#[derive(Debug, Clone)]
pub struct ThumbnailerArgs {
    show_version: bool,
    file_params: Option<ThumbnailerArgsFileParams>,
}

impl TryFrom<&Arguments> for ThumbnailerArgs {
    type Error = Error;

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
    pub fn file_params(&self) -> &Option<ThumbnailerArgsFileParams> {
        &self.file_params
    }
}

#[derive(Debug, Clone)]
pub struct ThumbnailerArgsFileParams {
    input_file: std::path::PathBuf,
    is_dry_run: bool,
    output_file: Option<std::path::PathBuf>,
    size: Option<u32>,
}

impl TryFrom<&Arguments> for ThumbnailerArgsFileParams {
    type Error = Error;

    fn try_from(arguments: &Arguments) -> Result<Self, Self::Error> {
        let mut args = arguments.clone();

        let input_file = args.value_from_str("-i")?;

        let is_dry_run = args.contains("-n");

        let output_file = args.opt_value_from_str("-o")?;

        let size = args.opt_value_from_str("-s")?;

        Ok(ThumbnailerArgsFileParams {
            input_file,
            is_dry_run,
            output_file,
            size,
        })
    }
}

impl ThumbnailerArgsFileParams {
    pub fn input_file(&self) -> &std::path::PathBuf {
        &self.input_file
    }
    pub fn is_dry_run(&self) -> bool {
        self.is_dry_run
    }
    pub fn output_file(&self) -> &Option<std::path::PathBuf> {
        &self.output_file
    }
    pub fn size(&self) -> Option<u32> {
        self.size
    }
}
