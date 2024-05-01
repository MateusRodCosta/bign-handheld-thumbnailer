use gdk_pixbuf::Pixbuf;

use super::N3DSCIAParsingErrorMetaInvalidSize;

#[derive(Debug, Clone)]
pub enum N3DSCIAMetaSize {
    MetaNone,
    MetaCVerUSA,
    MetaDummy,
    MetaPresent,
}

impl TryFrom<u32> for N3DSCIAMetaSize {
    type Error = N3DSCIAParsingErrorMetaInvalidSize;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(N3DSCIAMetaSize::MetaNone),
            8 => Ok(N3DSCIAMetaSize::MetaCVerUSA),
            0x200 => Ok(N3DSCIAMetaSize::MetaDummy),
            0x3AC0 => Ok(N3DSCIAMetaSize::MetaPresent),
            _ => Err(Self::Error { 0: value }),
        }
    }
}

#[derive(Debug, Clone)]
pub struct CIAMetaContent {
    smdh_content: SMDHContent,
}

impl CIAMetaContent {
    pub fn new(smdh_content: SMDHContent) -> CIAMetaContent {
        CIAMetaContent { smdh_content }
    }

    pub fn get_smdh_content(&self) -> &SMDHContent {
        &self.smdh_content
    }
}

#[derive(Debug, Clone)]
pub struct SMDHContent {
    large_icon: Pixbuf,
}

impl SMDHContent {
    pub fn new(large_icon: Pixbuf) -> SMDHContent {
        SMDHContent { large_icon }
    }

    pub fn get_large_icon(&self) -> &Pixbuf {
        &self.large_icon
    }
}

#[derive(Debug, Clone)]
pub struct N3DSXContent {
    smdh_content: SMDHContent,
}

impl N3DSXContent {
    pub fn new(smdh_content: SMDHContent) -> N3DSXContent {
        N3DSXContent { smdh_content }
    }

    pub fn get_smdh_content(&self) -> &SMDHContent {
        &self.smdh_content
    }
}

#[derive(Debug, Clone)]
pub struct ExeFSContent {
    icon: SMDHContent,
}

impl ExeFSContent {
    pub fn new(icon: SMDHContent) -> ExeFSContent {
        ExeFSContent { icon }
    }

    pub fn get_icon(&self) -> &SMDHContent {
        &self.icon
    }
}

#[derive(Debug, Clone)]
pub struct ExeFSFileHeader {
    file_name: String,
    file_offset: u32,
    file_size: u32,
}

impl ExeFSFileHeader {
    pub fn new(file_name: String, file_offset: u32, file_size: u32) -> ExeFSFileHeader {
        ExeFSFileHeader {
            file_name,
            file_offset,
            file_size,
        }
    }

    pub fn file_name(&self) -> &str {
        &self.file_name
    }

    pub fn file_offset(&self) -> u32 {
        self.file_offset
    }

    pub fn file_size(&self) -> u32 {
        self.file_size
    }
}

#[derive(Debug, Clone)]
pub struct CXIContent {
    exefs: ExeFSContent,
}

impl CXIContent {
    pub fn new(exefs: ExeFSContent) -> CXIContent {
        CXIContent { exefs }
    }

    pub fn get_exefs(&self) -> &ExeFSContent {
        &self.exefs
    }
}

#[derive(Debug, Clone)]
pub struct CCIPartition {
    _index: u8,
    offset: u32,
    length: u32,
}

impl CCIPartition {
    pub fn new(index: u8, offset: u32, length: u32) -> CCIPartition {
        CCIPartition {
            _index: index,
            offset,
            length,
        }
    }

    pub fn _index(&self) -> u8 {
        self._index
    }

    pub fn offset(&self) -> u32 {
        self.offset
    }

    pub fn lenght(&self) -> u32 {
        self.length
    }
}

#[derive(Debug, Clone)]
pub struct CCIContent {
    cxi: CXIContent,
}

impl CCIContent {
    pub fn new(cxi: CXIContent) -> CCIContent {
        CCIContent { cxi }
    }

    pub fn get_cxi(&self) -> &CXIContent {
        &self.cxi
    }
}
