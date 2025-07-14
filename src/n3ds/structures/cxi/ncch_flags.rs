use bitflags::bitflags;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NCCHFlags {
    pub crypto_method: NCCHCryptoMethodFlags,
    pub content_type: NCCHContentTypeFlags,
    pub security: NCCHSecurityFlags,
}

impl From<[u8; 8]> for NCCHFlags {
    fn from(value: [u8; 8]) -> Self {
        NCCHFlags {
            crypto_method: NCCHCryptoMethodFlags::from(value[3]),
            content_type: NCCHContentTypeFlags::from_bits_truncate(value[5]),
            security: NCCHSecurityFlags::from_bits_truncate(value[7]),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum NCCHCryptoMethodFlags {
    Invalid,
    Initial,
    KeyY,
    New3DSArm9Loader,
    New3DSArmLoaderChanged,
}

impl From<u8> for NCCHCryptoMethodFlags {
    fn from(value: u8) -> Self {
        match value {
            0x00 => NCCHCryptoMethodFlags::Initial,
            0x01 => NCCHCryptoMethodFlags::KeyY,
            0x0A => NCCHCryptoMethodFlags::New3DSArm9Loader,
            0x0B => NCCHCryptoMethodFlags::New3DSArmLoaderChanged,
            _ => NCCHCryptoMethodFlags::Invalid,
        }
    }
}

bitflags! {
    #[repr(transparent)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct NCCHContentTypeFlags: u8 {
        const DATA = 0x01;
        const EXECUTABLE = 0x02;
        const SYSTEM_UPDATE = 0x04;
        const MANUAL = 0x08;
        const TRIAL = 0x10;

        const CHILD = NCCHContentTypeFlags::SYSTEM_UPDATE.bits() | NCCHContentTypeFlags::MANUAL.bits();
    }
}

impl NCCHContentTypeFlags {
    pub fn is_cfa(&self) -> bool {
        self.contains(Self::DATA) && !self.contains(Self::EXECUTABLE)
    }

    pub fn is_cxi(&self) -> bool {
        self.contains(Self::EXECUTABLE)
    }
}

bitflags! {
    #[repr(transparent)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct NCCHSecurityFlags: u8 {
        const FIXED_CRYPTO_KEY = 0x1;
        const NO_MOUNT_ROM_FS = 0x2;
        const NO_CRYPTO = 0x4;
        const NEW_KEY_Y_GENERATOR = 0x20;
    }
}

impl NCCHSecurityFlags {
    pub fn is_not_encrypted(&self) -> bool {
        self.contains(Self::NO_CRYPTO)
    }
}
