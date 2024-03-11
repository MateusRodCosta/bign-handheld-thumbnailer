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
