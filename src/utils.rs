pub mod rgb888;

use std::path::Path;

use crate::error::ThumbnailerError;

#[derive(Debug)]
pub enum RomFormat {
    Nds,
    N3dsCia,
    N3dsSmdh,
    N3ds3dsx,
    N3dsCxi,
    N3dsCci
}

impl RomFormat {
    pub fn detect_from_header(data: &[u8; 512]) -> Option<Self> {
        match &data[0x100..0x104] {
            b"NCSD" => return Some(Self::N3dsCci),
            b"NCCH" => return Some(Self::N3dsCxi),
            _ => {}
        }

        match &data[0x000..0x004] {
            b"SMDH" => return Some(Self::N3dsSmdh),
            b"3DSX" => return Some(Self::N3ds3dsx),
            [0x20, 0x20, 0x00, 0x00] if data[0x004..0x006] == [0x00, 0x00] => {
                return Some(Self::N3dsCia);
            }
            _ => {}
        }

        if data[0x15C..0x15E] == [0x56, 0xCF] && data[0x0C0..0x0C4] == [0x24, 0xFF, 0xAE, 0x51] {
            return Some(Self::Nds);
        }

        None
    }

    pub fn detect_from_path(path: &Path) -> Option<Self> {
        let ext = path.extension()?.to_str()?;

        if ext.eq_ignore_ascii_case("nds")
            || ext.eq_ignore_ascii_case("srl")
            || ext.eq_ignore_ascii_case("dsi")
        {
            Some(Self::Nds)
        } else if ext.eq_ignore_ascii_case("cia") {
            Some(Self::N3dsCia)
        } else if ext.eq_ignore_ascii_case("smdh") {
            Some(Self::N3dsSmdh)
        } else if ext.eq_ignore_ascii_case("3dsx") {
            Some(Self::N3ds3dsx)
        } else if ext.eq_ignore_ascii_case("cxi")
            || ext.eq_ignore_ascii_case("cfa")
            || ext.eq_ignore_ascii_case("app")
        {
            Some(Self::N3dsCxi)
        } else if ext.eq_ignore_ascii_case("cci") || ext.eq_ignore_ascii_case("3ds") {
            Some(Self::N3dsCci)
        } else {
            None
        }
    }
}

pub fn get_rom_format(path: &Path, data: &[u8; 512]) -> Result<RomFormat, ThumbnailerError> {
    RomFormat::detect_from_header(data)
        .or_else(|| RomFormat::detect_from_path(path))
        .ok_or(ThumbnailerError::RomFormatDetectionFailure)
}
