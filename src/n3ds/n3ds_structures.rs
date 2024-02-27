use gdk_pixbuf::Pixbuf;

#[derive(Debug, Clone)]
pub enum N3DSCIAMetaSize {
    MetaNone,
    MetaCVerUSA,
    MetaDummy,
    MetaPresent,
    GarbageData(u32), // invalid value that should never happen for a valid .cia
}

impl N3DSCIAMetaSize {
    pub fn from(meta_size_value: u32) -> N3DSCIAMetaSize {
        match meta_size_value {
            0 => N3DSCIAMetaSize::MetaNone,
            8 => N3DSCIAMetaSize::MetaCVerUSA,
            0x200 => N3DSCIAMetaSize::MetaDummy,
            0x3AC0 => N3DSCIAMetaSize::MetaPresent,
            _ => N3DSCIAMetaSize::GarbageData(meta_size_value),
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
