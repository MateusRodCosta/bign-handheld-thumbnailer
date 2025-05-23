#[derive(Debug)]
pub struct NCCHFlags {
    _crypto_method: NCCHCryptoMethodFlags,
    _content_type: NCCHContentTypeFlags,
    security: NCCHSecurityFlags,
}

impl From<[u8; 8]> for NCCHFlags {
    fn from(value: [u8; 8]) -> Self {
        NCCHFlags {
            _crypto_method: NCCHCryptoMethodFlags::from(value[3]),
            _content_type: NCCHContentTypeFlags::from(value[5]),
            security: NCCHSecurityFlags::from(value[7]),
        }
    }
}

impl NCCHFlags {
    pub fn _crypto_method(&self) -> &NCCHCryptoMethodFlags {
        &self._crypto_method
    }

    pub fn _content_type(&self) -> &NCCHContentTypeFlags {
        &self._content_type
    }

    pub fn security_flags(&self) -> &NCCHSecurityFlags {
        &self.security
    }
}

#[derive(Debug)]
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

#[derive(Debug)]
pub struct NCCHContentTypeFlags {
    _data: bool,
    _executable: bool,
    _system_update: bool,
    _manual: bool,
    _child: bool,
    _trial: bool,
}

impl From<u8> for NCCHContentTypeFlags {
    fn from(value: u8) -> Self {
        NCCHContentTypeFlags {
            _data: (value & 0x1) != 0,
            _executable: (value & 0x2) != 0,
            _system_update: (value & 0x4) != 0,
            _manual: (value & 0x8) != 0,
            _child: (value & (0x4 | 0x8)) != 0,
            _trial: (value & 0x10) != 0,
        }
    }
}

impl NCCHContentTypeFlags {
    fn _is_cfa(&self) -> bool {
        self._data && !self._executable
    }

    fn _is_cxi(&self) -> bool {
        self._data && self._executable
    }
}

#[derive(Debug)]
pub struct NCCHSecurityFlags {
    _fixed_crypto_key: bool,
    _no_mount_rom_fs: bool,
    no_crypto: bool,
    _new_key_y_generator: bool,
}

impl From<u8> for NCCHSecurityFlags {
    fn from(value: u8) -> Self {
        NCCHSecurityFlags {
            _fixed_crypto_key: (value & 0x1) != 0,
            _no_mount_rom_fs: (value & 0x2) != 0,
            no_crypto: (value & 0x4) != 0,
            _new_key_y_generator: (value & 0x20) != 0,
        }
    }
}

impl NCCHSecurityFlags {
    pub fn is_not_encrypted(&self) -> bool {
        self.no_crypto
    }
}
