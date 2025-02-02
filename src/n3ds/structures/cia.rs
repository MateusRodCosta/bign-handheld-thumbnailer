use std::io::{Read, Seek, SeekFrom};

use crate::n3ds::errors::{CIAParsingError, CXIParsingError, N3DSParsingError};

use super::SMDHIcon;

#[derive(Debug, PartialEq, Eq)]
pub enum CIAMetaSize {
    None,
    CVerUSA,
    Dummy,
    Present,
}

impl TryFrom<u32> for CIAMetaSize {
    type Error = CIAParsingError;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(CIAMetaSize::None),
            8 => Ok(CIAMetaSize::CVerUSA),
            0x200 => Ok(CIAMetaSize::Dummy),
            0x3AC0 => Ok(CIAMetaSize::Present),
            _ => Err(Self::Error::MetaInvalidSize(value)),
        }
    }
}

#[derive(Debug)]
enum CIASignatureType {
    Rsa4096Sha1,
    Rsa2048Sha1,
    EllipticCurveWithSHA1,
    Rsa4096Sha256,
    Rsa2048Sha256,
    EcdsaWithSha256,
}

impl TryFrom<u32> for CIASignatureType {
    type Error = CIAParsingError;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            0x0001_0000 => Ok(CIASignatureType::Rsa4096Sha1),
            0x0001_0001 => Ok(CIASignatureType::Rsa2048Sha1),
            0x0001_0002 => Ok(CIASignatureType::EllipticCurveWithSHA1),
            0x0001_0003 => Ok(CIASignatureType::Rsa4096Sha256),
            0x0001_0004 => Ok(CIASignatureType::Rsa2048Sha256),
            0x0001_0005 => Ok(CIASignatureType::EcdsaWithSha256),
            _ => Err(Self::Error::SignatureTypeInvalidValue(value)),
        }
    }
}

impl CIASignatureType {
    pub fn size(&self) -> usize {
        match self {
            CIASignatureType::Rsa4096Sha1 | CIASignatureType::Rsa4096Sha256 => 0x200,
            CIASignatureType::Rsa2048Sha1 | CIASignatureType::Rsa2048Sha256 => 0x100,
            CIASignatureType::EllipticCurveWithSHA1 | CIASignatureType::EcdsaWithSha256 => 0x3C,
        }
    }

    pub fn padding_size(&self) -> usize {
        match self {
            CIASignatureType::Rsa4096Sha1
            | CIASignatureType::Rsa4096Sha256
            | CIASignatureType::Rsa2048Sha1
            | CIASignatureType::Rsa2048Sha256 => 0x3C,
            CIASignatureType::EllipticCurveWithSHA1 | CIASignatureType::EcdsaWithSha256 => 0x40,
        }
    }
}

#[derive(Debug)]
pub struct CIATitleMetadata {
    content_chunk_records: Vec<CIAContentChunkRecord>,
}

impl CIATitleMetadata {
    pub fn from_file<T: Read + Seek>(f: &mut T) -> Result<Self, N3DSParsingError> {
        const TITLE_METADATA_HEADER_CONTENT_COUNT_OFFSET: i64 = 0x9E;
        const CONTENT_CHUNK_RECORDS_OFFSET: u64 = 0x9C4;
        const CONTENT_CHUNK_RECORD_SIZE: usize = 0x30;

        let mut signature_type = [0u8; 4];
        f.read_exact(&mut signature_type)?;
        let signature_type = u32::from_be_bytes(signature_type);
        let signature_type = CIASignatureType::try_from(signature_type)?;

        let signature_full_size: i64 = (signature_type.size() + signature_type.padding_size())
            .try_into()
            .unwrap();
        let header_position = f.seek(SeekFrom::Current(signature_full_size))?;

        f.seek_relative(TITLE_METADATA_HEADER_CONTENT_COUNT_OFFSET)?;
        let mut content_count = [0u8; 2];
        f.read_exact(&mut content_count)?;
        let content_count = u16::from_be_bytes(content_count);

        f.seek(SeekFrom::Start(
            header_position + CONTENT_CHUNK_RECORDS_OFFSET,
        ))?;

        let mut content_chunk_records: Vec<_> = vec![];

        for _ in 0..content_count {
            let mut content_chunk_record = [0u8; CONTENT_CHUNK_RECORD_SIZE];
            f.read_exact(&mut content_chunk_record)?;
            let content_chunk_record = CIAContentChunkRecord::from_bytes(&content_chunk_record)?;
            content_chunk_records.push(content_chunk_record);
        }

        let title_metadata = CIATitleMetadata {
            content_chunk_records,
        };

        Ok(title_metadata)
    }

    pub fn content_chunk_records(&self) -> &Vec<CIAContentChunkRecord> {
        &self.content_chunk_records
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum CIAContentIndex {
    MainContent,
    HomeMenuManual,
    DlpChildContainer,
}

impl TryFrom<u16> for CIAContentIndex {
    type Error = CIAParsingError;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(CIAContentIndex::MainContent),
            1 => Ok(CIAContentIndex::HomeMenuManual),
            2 => Ok(CIAContentIndex::DlpChildContainer),
            _ => Err(Self::Error::ContentIndexInvalidValue(value)),
        }
    }
}

#[derive(Debug)]
pub struct CIAContentChunkRecord {
    _content_id: u32,
    content_index: CIAContentIndex,
    content_type: u16,
    _content_size: u64,
    _sha256_hash: [u8; 0x20],
}

impl CIAContentChunkRecord {
    pub fn from_bytes(content_chunk_record_bytes: &[u8; 0x30]) -> Result<Self, CIAParsingError> {
        let content_id = u32::from_be_bytes(content_chunk_record_bytes[..4].try_into().unwrap());
        let content_index =
            u16::from_be_bytes(content_chunk_record_bytes[0x4..0x4 + 2].try_into().unwrap());
        let content_type =
            u16::from_be_bytes(content_chunk_record_bytes[0x6..0x6 + 2].try_into().unwrap());
        let content_size =
            u64::from_be_bytes(content_chunk_record_bytes[0x8..0x8 + 8].try_into().unwrap());
        let sha256_hash: [u8; 0x20] = content_chunk_record_bytes[0x10..].try_into().unwrap();

        let content_index = CIAContentIndex::try_from(content_index)?;

        let content_info_record = CIAContentChunkRecord {
            _content_id: content_id,
            content_index,
            content_type,
            _content_size: content_size,
            _sha256_hash: sha256_hash,
        };
        Ok(content_info_record)
    }

    pub fn content_index(&self) -> &CIAContentIndex {
        &self.content_index
    }
    pub fn content_type(&self) -> u16 {
        self.content_type
    }
}

impl SMDHIcon {
    pub fn from_cia<T: Read + Seek>(f: &mut T) -> Result<Self, N3DSParsingError> {
        /*
         * The meta section isn't in a fixed place and is located after a bunch of sections whose
         * size can vary, therefore it's needed to at the very last fetch the other sizes and
         * take the padding into account
         */

        const CIA_HEADER_CERTIFICATE_CHAIN_SIZE_OFFSET: u64 = 0x08;
        const CIA_HEADER_SIZE: u64 = 0x2040;
        const CIA_PADDING_SIZE: u64 = 0x40;

        f.seek(SeekFrom::Start(CIA_HEADER_CERTIFICATE_CHAIN_SIZE_OFFSET))?;
        let mut certificate_chain_size = [0u8; 4];
        f.read_exact(&mut certificate_chain_size)?;
        let certificate_chain_size: u64 = u32::from_le_bytes(certificate_chain_size).into();

        let mut ticket_size = [0u8; 4];
        f.read_exact(&mut ticket_size)?;
        let ticket_size: u64 = u32::from_le_bytes(ticket_size).into();

        let mut tmd_size = [0u8; 4];
        f.read_exact(&mut tmd_size)?;
        let tmd_size: u64 = u32::from_le_bytes(tmd_size).into();

        let mut meta_size = [0u8; 4];
        f.read_exact(&mut meta_size)?;
        let meta_size = u32::from_le_bytes(meta_size);
        let meta_size = CIAMetaSize::try_from(meta_size)?;

        let mut content_size = [0u8; 8];
        f.read_exact(&mut content_size)?;
        let content_size: u64 = u64::from_le_bytes(content_size);

        let certificate_chain_size_with_padding =
            certificate_chain_size.next_multiple_of(CIA_PADDING_SIZE);
        let ticket_size_with_padding = ticket_size.next_multiple_of(CIA_PADDING_SIZE);
        let tmd_size_with_padding = tmd_size.next_multiple_of(CIA_PADDING_SIZE);
        let content_size_with_padding = content_size.next_multiple_of(CIA_PADDING_SIZE);

        if meta_size == CIAMetaSize::Present {
            let offset_meta: u64 = CIA_HEADER_SIZE
                + certificate_chain_size_with_padding
                + ticket_size_with_padding
                + tmd_size_with_padding
                + content_size_with_padding;

            f.seek(SeekFrom::Start(offset_meta))?;
            let meta_smdh_icon = SMDHIcon::from_cia_meta(f)?;
            return Ok(meta_smdh_icon);
        }
        eprintln!("CIA Meta section not present, attempting CIA's CXI..");

        let offset_tmd: u64 =
            CIA_HEADER_SIZE + certificate_chain_size_with_padding + ticket_size_with_padding;
        f.seek(SeekFrom::Start(offset_tmd))?;

        let offset_content: u64 = CIA_HEADER_SIZE
            + certificate_chain_size_with_padding
            + ticket_size_with_padding
            + tmd_size_with_padding;

        match SMDHIcon::from_cia_tmd(f, offset_content) {
            Ok(icon) => Ok(icon),
            Err(error) => {
                eprintln!("Failed to parse SMDH from CIA's CXI");
                Err(error)
            }
        }
    }

    pub fn from_cia_meta<T: Read + Seek>(f: &mut T) -> Result<Self, N3DSParsingError> {
        const CIA_META_SMDH_OFFSET: i64 = 0x400;
        f.seek_relative(CIA_META_SMDH_OFFSET)?;
        let smdh_icon = SMDHIcon::from_smdh(f)?;
        Ok(smdh_icon)
    }

    pub fn from_cia_tmd<T: Read + Seek>(
        f: &mut T,
        content_offset: u64,
    ) -> Result<Self, N3DSParsingError> {
        let title_metadata = CIATitleMetadata::from_file(f)?;

        f.seek(SeekFrom::Start(content_offset))?;
        let Some(cxi_content) = title_metadata
            .content_chunk_records()
            .iter()
            .find(|item| *item.content_index() == CIAContentIndex::MainContent)
        else {
            return Err(CIAParsingError::NoIconAvailable(CXIParsingError::NoCXIContent).into());
        };

        if (cxi_content.content_type() & 0x1) != 0 {
            return Err(CIAParsingError::NoIconAvailable(CXIParsingError::FileEncrypted).into());
        };

        SMDHIcon::from_cxi(f)
    }
}
