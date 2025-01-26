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
