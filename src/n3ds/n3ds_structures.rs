use gdk_pixbuf::Pixbuf;

#[derive(Debug)]
pub enum N3DSCIAMeta {
    MetaNone,
    MetaCVerUSA,
    MetaDummy,
    MetaPresent,
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
