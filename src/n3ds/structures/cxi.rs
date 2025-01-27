use std::io::{Read, Seek};

use crate::n3ds::{
    errors::{CXIParsingError, ParsingError},
    structures::SMDHIcon,
};

#[derive(Debug)]
pub struct ExeFSFileHeader {
    file_name: [u8; 8],
    file_offset: u32,
    _file_size: u32,
}

impl ExeFSFileHeader {
    pub fn from_bytes(file_headers_bytes: &[u8; 16]) -> Option<Self> {
        // Each header is composed of 16 bytes, if the header is empty it will be filled with zeroes
        if *file_headers_bytes == [0u8; 16] {
            return None;
        }

        let file_name: [u8; 8] = file_headers_bytes[..8].try_into().unwrap();
        let file_offset = u32::from_le_bytes(file_headers_bytes[8..8 + 4].try_into().unwrap());
        let file_size = u32::from_le_bytes(file_headers_bytes[8 + 4..].try_into().unwrap());

        let exefs_file_header = ExeFSFileHeader {
            file_name,
            file_offset,
            _file_size: file_size,
        };
        Some(exefs_file_header)
    }

    pub fn file_name(&self) -> &[u8] {
        let len = self.file_name.iter().position(|p| *p == b'\0').unwrap_or(8);
        &self.file_name[..len]
    }

    pub fn file_offset(&self) -> u32 {
        self.file_offset
    }
}

impl SMDHIcon {
    pub fn from_cxi<T: Read + Seek>(f: &mut T) -> Result<Self, ParsingError> {
        const CXI_HEADER_MAGIC_OFFSET: i64 = 0x100;
        const CXI_HEADER_FLAGS_OFFSET: i64 = 0x188;
        const CXI_HEADER_EXEFS_OFFSET_VALUE: i64 = 0x1A0;
        const CXI_MEDIA_UNIT_SIZE: i64 = 0x200;

        f.seek_relative(CXI_HEADER_MAGIC_OFFSET)?;
        let mut cxi_magic = [0u8; 4];
        f.read_exact(&mut cxi_magic)?;
        if b"NCCH" != &cxi_magic {
            return Err(ParsingError::FileMagicNotFound("NCCH", cxi_magic));
        }

        f.seek_relative(CXI_HEADER_FLAGS_OFFSET - (CXI_HEADER_MAGIC_OFFSET + 4))?;
        let mut flags = [0u8; 8];
        f.read_exact(&mut flags)?;
        let flags_index_7 = flags[7];
        if (flags_index_7 & 0x4) != 0x4 {
            return Err(CXIParsingError::FileEncrypted.into());
        }

        f.seek_relative(CXI_HEADER_EXEFS_OFFSET_VALUE - (CXI_HEADER_FLAGS_OFFSET + 8))?;

        let mut exefs_offset = [0u8; 4];
        f.read_exact(&mut exefs_offset)?;
        let exefs_offset: i64 = u32::from_le_bytes(exefs_offset).try_into().unwrap(); // in media units
        let exefs_offset = exefs_offset * CXI_MEDIA_UNIT_SIZE;

        let mut exefs_size = [0u8; 4];
        f.read_exact(&mut exefs_size)?;

        f.seek_relative(exefs_offset - (CXI_HEADER_EXEFS_OFFSET_VALUE + 4 + 4))?;
        let smdh_icon = SMDHIcon::from_exefs(f)?;
        Ok(smdh_icon)
    }

    pub fn from_exefs<T: Read + Seek>(f: &mut T) -> Result<Self, ParsingError> {
        const EXEFS_HEADER_FILE_HEADERS_SIZE: usize = 0xA0;
        const EXEFS_HEADER_SIZE: i64 = 0x200;

        let mut file_headers = [0u8; EXEFS_HEADER_FILE_HEADERS_SIZE];
        f.read_exact(&mut file_headers)?;
        let mut file_headers = file_headers
            .chunks_exact(16)
            .filter_map(|chunk| ExeFSFileHeader::from_bytes(chunk.try_into().unwrap()));
        let Some(icon_file) = file_headers.find(|item| item.file_name() == b"icon") else {
            return Err(CXIParsingError::ExeFSIconFileNotFound.into());
        };

        f.seek_relative(
            EXEFS_HEADER_SIZE + i64::from(icon_file.file_offset())
                - i64::try_from(EXEFS_HEADER_FILE_HEADERS_SIZE).unwrap(),
        )?;
        let smdh_icon = SMDHIcon::from_smdh(f)?;
        Ok(smdh_icon)
    }
}
