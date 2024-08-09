#[derive(Debug, Clone)]
pub struct ExeFSFileHeader {
    file_name: [u8; 8],
    file_offset: u32,
    _file_size: u32,
}

impl ExeFSFileHeader {
    pub fn from_bytes(file_headers_bytes: &[u8; 16]) -> Option<Self> {
        // Each header is composed of 16 bytes, if the header is empty it will be filled with zeroes
        // Therefore we can read it as a u128 and check if it's results in a zero as a small optimization

        let is_empty = u128::from_ne_bytes(*file_headers_bytes);
        let is_empty = is_empty == 0;
        if is_empty {
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
