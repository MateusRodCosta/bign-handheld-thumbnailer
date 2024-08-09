#[derive(Debug, Clone)]
pub struct CCIPartition {
    offset: u32,
    _length: u32,
}

impl CCIPartition {
    pub fn from_bytes(partition_bytes: [u8; 8]) -> Self {
        let offset = u32::from_le_bytes(partition_bytes[..4].try_into().unwrap()); //in media units
        let offset = offset * 0x200;

        let length = u32::from_le_bytes(partition_bytes[4..].try_into().unwrap()); //in media units
        let length = length * 0x200;

        CCIPartition {
            offset,
            _length: length,
        }
    }

    pub fn offset(&self) -> u32 {
        self.offset
    }
}
