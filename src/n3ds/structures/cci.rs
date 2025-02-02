use std::io::{Read, Seek, SeekFrom};

use crate::n3ds::{errors::N3DSParsingError, structures::SMDHIcon};

#[derive(Debug)]
pub struct CCIPartition {
    offset: u32,
    _length: u32,
}

impl CCIPartition {
    pub fn from_bytes(partition_bytes: [u8; 8]) -> Self {
        const MEDIA_UNIT_SIZE: u32 = 0x200;

        let offset = u32::from_le_bytes(partition_bytes[..4].try_into().unwrap()); //in media units
        let offset = offset * MEDIA_UNIT_SIZE;

        let length = u32::from_le_bytes(partition_bytes[4..].try_into().unwrap()); //in media units
        let length = length * MEDIA_UNIT_SIZE;

        CCIPartition {
            offset,
            _length: length,
        }
    }

    pub fn offset(&self) -> u32 {
        self.offset
    }
}

impl SMDHIcon {
    pub fn from_cci<T: Read + Seek>(f: &mut T) -> Result<Self, N3DSParsingError> {
        const CCI_HEADER_MAGIC_OFFSET: u64 = 0x100;
        const CCI_HEADER_PARTITION_TABLE_OFFSET: u64 = 0x120;
        const CCI_HEADER_PARTITION_TABLE_SIZE: usize = 0x40;

        f.seek(SeekFrom::Start(CCI_HEADER_MAGIC_OFFSET))?;
        let mut cci_magic = [0u8; 4];
        f.read_exact(&mut cci_magic)?;
        if b"NCSD" != &cci_magic {
            return Err(N3DSParsingError::FileMagicNotFound("NCSD", cci_magic));
        }
        f.seek(SeekFrom::Start(CCI_HEADER_PARTITION_TABLE_OFFSET))?;
        let mut partition_table = [0u8; CCI_HEADER_PARTITION_TABLE_SIZE];
        f.read_exact(&mut partition_table)?;

        let partition_table: [CCIPartition; CCI_HEADER_PARTITION_TABLE_SIZE / 8] = partition_table
            .chunks_exact(8)
            .map(|chunk| CCIPartition::from_bytes(chunk.try_into().unwrap()))
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();
        let Some(first_partition) = partition_table.first() else {
            return Err(N3DSParsingError::CCIErrorGettingExecutableContentPartition);
        };

        f.seek(SeekFrom::Start(first_partition.offset().into()))?;
        let smdh_icon = SMDHIcon::from_cxi(f)?;
        Ok(smdh_icon)
    }
}
