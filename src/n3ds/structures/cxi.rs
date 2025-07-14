mod ncch_flags;

use std::io::{Read, Seek, SeekFrom};

use crate::n3ds::{
    errors::{CXIParsingError, N3DSParsingError},
    structures::{cxi::ncch_flags::NCCHFlags, SMDHIcon},
};

#[derive(Debug)]
pub struct ExeFSFileHeader {
    file_name: [u8; 8],
    pub file_offset: u32,
    pub file_size: u32,
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
            file_size,
        };
        Some(exefs_file_header)
    }

    pub fn file_name(&self) -> &[u8] {
        let len = self.file_name.iter().position(|p| *p == b'\0').unwrap_or(8);
        &self.file_name[..len]
    }
}

impl SMDHIcon {
    pub fn from_cxi<T: Read + Seek>(f: &mut T) -> Result<Self, N3DSParsingError> {
        const CXI_HEADER_MAGIC_OFFSET: u64 = 0x100;
        const CXI_HEADER_FLAGS_OFFSET: u64 = 0x188;
        const CXI_HEADER_EXEFS_OFFSET_VALUE: u64 = 0x1A0;
        const CXI_MEDIA_UNIT_SIZE: u64 = 0x200;
        const CXI_MAGIC: &[u8] = b"NCCH";

        let cxi_start_pos = f.stream_position()?;

        f.seek(SeekFrom::Start(cxi_start_pos + CXI_HEADER_MAGIC_OFFSET))?;
        let mut cxi_magic = [0u8; 4];
        f.read_exact(&mut cxi_magic)?;
        if CXI_MAGIC != &cxi_magic {
            return Err(N3DSParsingError::FileMagicNotFound("NCCH", cxi_magic));
        }

        f.seek(SeekFrom::Start(cxi_start_pos + CXI_HEADER_FLAGS_OFFSET))?;
        let mut flags = [0u8; 8];
        f.read_exact(&mut flags)?;
        let flags = NCCHFlags::from(flags);
        let security_flags = flags.security;
        if !security_flags.is_not_encrypted() {
            return Err(CXIParsingError::FileEncrypted.into());
        }

        f.seek(SeekFrom::Start(
            cxi_start_pos + CXI_HEADER_EXEFS_OFFSET_VALUE,
        ))?;

        let mut exefs_offset = [0u8; 4];
        f.read_exact(&mut exefs_offset)?;
        let exefs_offset: u64 = u32::from_le_bytes(exefs_offset).into(); // in media units
        let exefs_offset = exefs_offset * CXI_MEDIA_UNIT_SIZE;

        let mut _exefs_size = [0u8; 4];
        f.read_exact(&mut _exefs_size)?;

        f.seek(SeekFrom::Start(cxi_start_pos + exefs_offset))?;
        let smdh_icon = Self::from_exefs(f)?;
        Ok(smdh_icon)
    }

    pub fn from_exefs<T: Read + Seek>(f: &mut T) -> Result<Self, N3DSParsingError> {
        const EXEFS_FILE_HEADERS_BLOCK_SIZE: usize = 0xA0;
        const EXEFS_HEADER_TOTAL_SIZE: u64 = 0x200;
        const ICON_FILENAME: &[u8] = b"icon";

        let exefs_start_pos = f.stream_position()?;

        let mut file_headers = [0u8; EXEFS_FILE_HEADERS_BLOCK_SIZE];
        f.read_exact(&mut file_headers)?;

        let icon_file = file_headers
            .chunks_exact(16)
            .filter_map(|chunk| ExeFSFileHeader::from_bytes(chunk.try_into().unwrap()))
            .find(|item| item.file_name() == ICON_FILENAME)
            .ok_or(CXIParsingError::ExeFSIconFileNotFound)?;

        let icon_pos = exefs_start_pos + EXEFS_HEADER_TOTAL_SIZE + u64::from(icon_file.file_offset);

        f.seek(SeekFrom::Start(icon_pos))?;
        let smdh_icon = Self::from_smdh(f)?;
        Ok(smdh_icon)
    }
}
