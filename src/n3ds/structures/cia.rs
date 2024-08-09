use std::io::{Read, Seek, SeekFrom};

use crate::n3ds::errors::ParsingError;

#[derive(Debug, Clone)]
pub enum CIAMetaSize {
    None,
    CVerUSA,
    Dummy,
    Present,
}

impl TryFrom<u32> for CIAMetaSize {
    type Error = ParsingError;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(CIAMetaSize::None),
            8 => Ok(CIAMetaSize::CVerUSA),
            0x200 => Ok(CIAMetaSize::Dummy),
            0x3AC0 => Ok(CIAMetaSize::Present),
            _ => Err(Self::Error::CIAMetaInvalidSize(value)),
        }
    }
}

impl CIAMetaSize {
    pub fn value(self) -> u32 {
        match self {
            CIAMetaSize::Present => 0x3AC0,
            _ => 0,
        }
    }
}

#[derive(Debug, Clone)]
enum CIASignatureType {
    Rsa4096Sha1,
    Rsa2048Sha1,
    EllipticCurveWithSHA1,
    Rsa4096Sha256,
    Rsa2048Sha256,
    EcdsaWithSha256,
}

impl TryFrom<u32> for CIASignatureType {
    type Error = ParsingError;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            0x010000 => Ok(CIASignatureType::Rsa4096Sha1),
            0x010001 => Ok(CIASignatureType::Rsa2048Sha1),
            0x010002 => Ok(CIASignatureType::EllipticCurveWithSHA1),
            0x010003 => Ok(CIASignatureType::Rsa4096Sha256),
            0x010004 => Ok(CIASignatureType::Rsa2048Sha256),
            0x010005 => Ok(CIASignatureType::EcdsaWithSha256),
            _ => Err(Self::Error::CIASignatureTypeInvalidValue(value)),
        }
    }
}

impl CIASignatureType {
    pub fn size(&self) -> usize {
        match self {
            CIASignatureType::Rsa4096Sha1 => 0x200,
            CIASignatureType::Rsa2048Sha1 => 0x100,
            CIASignatureType::EllipticCurveWithSHA1 => 0x3C,
            CIASignatureType::Rsa4096Sha256 => 0x200,
            CIASignatureType::Rsa2048Sha256 => 0x100,
            CIASignatureType::EcdsaWithSha256 => 0x3C,
        }
    }

    pub fn padding_size(&self) -> usize {
        match self {
            CIASignatureType::Rsa4096Sha1 => 0x3C,
            CIASignatureType::Rsa2048Sha1 => 0x3C,
            CIASignatureType::EllipticCurveWithSHA1 => 0x40,
            CIASignatureType::Rsa4096Sha256 => 0x3C,
            CIASignatureType::Rsa2048Sha256 => 0x3C,
            CIASignatureType::EcdsaWithSha256 => 0x40,
        }
    }
}

#[derive(Debug, Clone)]
pub struct CIATitleMetadata {
    content_chunk_records: Vec<CIAContentChunkRecord>,
}

impl CIATitleMetadata {
    pub fn from_file<T: Read + Seek>(f: &mut T) -> Result<Self, ParsingError> {
        let mut signature_type = [0u8; 4];
        f.read_exact(&mut signature_type)?;
        let signature_type = u32::from_be_bytes(signature_type);
        let signature_type = CIASignatureType::try_from(signature_type)?;

        let signature_full_size: i64 = (signature_type.size() + signature_type.padding_size())
            .try_into()
            .unwrap();
        let header_position = f.seek(SeekFrom::Current(signature_full_size))?;

        const TITLE_METADATA_HEADER_CONTENT_COUNT_OFFSET: i64 = 0x9E;
        f.seek(SeekFrom::Current(
            TITLE_METADATA_HEADER_CONTENT_COUNT_OFFSET,
        ))?;
        let mut content_count = [0u8; 2];
        f.read_exact(&mut content_count)?;
        let content_count = u16::from_be_bytes(content_count);

        const CONTENT_CHUNK_RECORDS_OFFSET: u64 = 0x9C4;
        f.seek(SeekFrom::Start(
            header_position + CONTENT_CHUNK_RECORDS_OFFSET,
        ))?;

        const CONTENT_CHUNK_RECORD_SIZE: usize = 0x30;
        let mut content_chunk_records: Vec<_> = vec![];

        for _ in 0..content_count {
            let mut content_chunk_record = [0u8; CONTENT_CHUNK_RECORD_SIZE];
            f.read_exact(&mut content_chunk_record)?;
            let content_chunk_record =
                CIAContentChunkRecord::from_bytes(&content_chunk_record.try_into().unwrap())?;
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

#[derive(Debug, Clone)]
pub enum CIAContentIndex {
    MainContent,
    HomeMenuManual,
    DlpChildContainer,
}

impl TryFrom<u16> for CIAContentIndex {
    type Error = ParsingError;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(CIAContentIndex::MainContent),
            1 => Ok(CIAContentIndex::HomeMenuManual),
            2 => Ok(CIAContentIndex::DlpChildContainer),
            _ => Err(Self::Error::CIAContentIndexInvalidValue(value)),
        }
    }
}

#[derive(Debug, Clone)]
pub struct CIAContentChunkRecord {
    content_id: u32,
    content_index: CIAContentIndex,
    content_type: u16,
    content_size: u64,
    _sha256_hash: [u8; 0x20],
}

impl CIAContentChunkRecord {
    pub fn from_bytes(content_chunk_record_bytes: &[u8; 0x30]) -> Result<Self, ParsingError> {
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
            content_id,
            content_index,
            content_type,
            content_size,
            _sha256_hash: sha256_hash,
        };
        Ok(content_info_record)
    }

    pub fn content_id(&self) -> u32 {
        self.content_id
    }

    pub fn content_index(&self) -> &CIAContentIndex {
        &self.content_index
    }

    pub fn content_type(&self) -> u16 {
        self.content_type
    }

    pub fn content_size(&self) -> u64 {
        self.content_size
    }
}
