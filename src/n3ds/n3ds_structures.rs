use gdk_pixbuf::Pixbuf;
use std::io::{Read, Seek, SeekFrom};

use crate::n3ds::n3ds_parsing_errors::*;
use crate::utils::Rgb888;

/*
 * Intially SMDH, 3DSX and CIA files were supported.
 * Later on, support for CCI and CXI (including parsing contents of ExeFS) was added.
 *
 * Consider the following links for more info about the CIA, SMDH and 3DSX structure:
 *
 * On GBATEK:
 * SMDH: https://problemkaputt.de/gbatek.htm#3dsfilesvideoiconssmdh
 * 3DSX: https://problemkaputt.de/gbatek.htm#3dsfilestitlehomebrewexecutables3dsx
 * CIA: https://problemkaputt.de/gbatek.htm#3dsfilestitleinstallationarchivecia
 * CCI (scpecialization of NCSD): https://problemkaputt.de/gbatek.htm#3dsfilesncsdformat
 * CXI (specialization of NCCH):https://problemkaputt.de/gbatek.htm#3dsfilesncchformat
 * ExeFS (internal to CXI): https://problemkaputt.de/gbatek.htm#3dsfilesncchexefs
 *
 * On 3dbrew:
 * SMDH: https://www.3dbrew.org/wiki/SMDH
 * 3DSX: https://www.3dbrew.org/wiki/3DSX_Format
 * CIA: https://www.3dbrew.org/wiki/CIA
 * CCI: https://www.3dbrew.org/wiki/CCI
 * CXI: https://www.3dbrew.org/wiki/NCCH#CXI
 * ExeFS: https://www.3dbrew.org/wiki/ExeFS
 *
 * Do note that the Meta section conatining a SMHD might or might not be present on .cia files.
 * There's a proper way to get icons ffrom a CIA, getting it from the Meta section is currently a shortcut.
 *
 * Do also note that the extended header with a SMHD is optional for .3dsx
*/

#[derive(Debug, Clone)]
pub struct SMDHIcon {
    large_icon: Pixbuf,
}

impl SMDHIcon {
    pub fn get_large_icon(&self) -> Pixbuf {
        self.large_icon.clone()
    }

    fn generate_pixbuf_from_bytes(large_icon_bytes: &[u8; 0x1200]) -> Option<Pixbuf> {
        let large_icon_data: Vec<Rgb888> = large_icon_bytes
            .chunks_exact(2)
            .map(|chunk| Rgb888::from_rgb565_bytes(chunk.try_into().unwrap()))
            .collect();

        let pixbuf = Pixbuf::new(gdk_pixbuf::Colorspace::Rgb, true, 8, 48, 48)?;

        /*
         * The large 3DS icon is 48x48 px and divided in tiles according to Morton order
         * Each color will usually be RGB565 although it's not the only supported color enconding
         */

        /*
         * Due to the Morton order, the code for the coordinates of the pixels is oxided from
         * https://github.com/GEMISIS/SMDH-Creator/blob/master/SMDH-Creator/SMDH.cs#L255
         */

        let tile_order = [
            0, 1, 8, 9, 2, 3, 10, 11, 16, 17, 24, 25, 18, 19, 26, 27, 4, 5, 12, 13, 6, 7, 14, 15,
            20, 21, 28, 29, 22, 23, 30, 31, 32, 33, 40, 41, 34, 35, 42, 43, 48, 49, 56, 57, 50, 51,
            58, 59, 36, 37, 44, 45, 38, 39, 46, 47, 52, 53, 60, 61, 54, 55, 62, 63,
        ];

        let mut pos = 0;
        for tile_y in 0..6 {
            for tile_x in 0..6 {
                for k in 0..64 {
                    let x = tile_order[k] & 0x7;
                    let y = tile_order[k] >> 3;
                    let coords = (x + (tile_x * 8), y + (tile_y * 8));

                    let rgb = &large_icon_data[pos];
                    pixbuf.put_pixel(coords.0, coords.1, rgb.r(), rgb.g(), rgb.b(), 0xFF);

                    pos += 1;
                }
            }
        }

        Some(pixbuf)
    }
}

impl SMDHIcon {
    pub fn from_smdh<T: Read + Seek>(f: &mut T) -> Result<Self, N3DSParsingError> {
        let mut smdh_magic = [0u8; 4];
        f.read_exact(&mut smdh_magic)?;
        if b"SMDH" != &smdh_magic {
            return Err(N3DSParsingError::FileMagicNotFound(
                FileMagicNotFound::SMDHMagicNotFound(smdh_magic),
            ));
        }

        const SMDH_LARGE_ICON_OFFSET: i64 = 0x24C0;
        f.seek(SeekFrom::Current(SMDH_LARGE_ICON_OFFSET - 4))?;
        const SMDH_LARGE_ICON_SIZE: usize = 0x1200;
        let mut large_icon_bytes = [0u8; SMDH_LARGE_ICON_SIZE];
        f.read_exact(&mut large_icon_bytes)?;
        let Some(large_icon) = SMDHIcon::generate_pixbuf_from_bytes(&large_icon_bytes) else {
            return Err(N3DSParsingError::UnableToExtractN3DSIcon);
        };
        Ok(SMDHIcon { large_icon })
    }

    pub fn from_n3dsx<T: Read + Seek>(f: &mut T) -> Result<Self, N3DSParsingError> {
        let mut n3dsx_magic = [0u8; 4];
        f.read_exact(&mut n3dsx_magic)?;
        if b"3DSX" != &n3dsx_magic {
            return Err(N3DSParsingError::FileMagicNotFound(
                FileMagicNotFound::N3DSXMagicNotFound(n3dsx_magic),
            ));
        }

        let mut header_size = [0u8; 2];
        f.read_exact(&mut header_size)?;
        let header_size = u16::from_le_bytes(header_size);
        if !(header_size > 32) {
            return Err(N3DSParsingError::N3DSXParsingError3DSXNoExtendedHeader { 0: header_size });
        }

        const N3DSX_EXTENDED_HEADER_OFFSET: u64 = 0x20;
        f.seek(SeekFrom::Start(N3DSX_EXTENDED_HEADER_OFFSET))?;

        let mut smdh_offset = [0u8; 4];
        f.read_exact(&mut smdh_offset)?;
        let smdh_offset = u32::from_le_bytes(smdh_offset);

        let mut smdh_size = [0u8; 4];
        f.read_exact(&mut smdh_size)?;
        let _smdh_size = u32::from_le_bytes(smdh_size);

        f.seek(SeekFrom::Start(smdh_offset.into()))?;
        let smdh_icon = SMDHIcon::from_smdh(f)?;
        Ok(smdh_icon)
    }

    pub fn from_cia<T: Read + Seek>(f: &mut T) -> Result<Self, N3DSParsingError> {
        /*
         * The meta section isn't in a fixed place and is located after a bunch of sections whose
         * size can vary, therefore it's needed to at the very last fetch the other sizes and
         * take the padding into account
         */

        const CIA_HEADER_CERTIFICATE_CHAIN_SIZE_OFFSET: u64 = 0x08;
        f.seek(SeekFrom::Start(CIA_HEADER_CERTIFICATE_CHAIN_SIZE_OFFSET))?;
        let mut certificate_chain_size = [0u8; 4];
        f.read_exact(&mut certificate_chain_size)?;
        let certificate_chain_size = u32::from_le_bytes(certificate_chain_size);

        let mut ticket_size = [0u8; 4];
        f.read_exact(&mut ticket_size)?;
        let ticket_size = u32::from_le_bytes(ticket_size);

        let mut tmd_size = [0u8; 4];
        f.read_exact(&mut tmd_size)?;
        let tmd_size = u32::from_le_bytes(tmd_size);

        let mut meta_size = [0u8; 4];
        f.read_exact(&mut meta_size)?;
        let meta_size = u32::from_le_bytes(meta_size);

        let meta_size = CIAMetaSize::try_from(meta_size)?;
        let meta_size: u32 = match meta_size {
            CIAMetaSize::Present => 0x3AC0,
            _ => {
                return Err(N3DSParsingError::CIAParsingError(
                    CIAParsingError::MetaNotExpectedValue { 0: meta_size },
                ))
            }
        };

        let mut content_size = [0u8; 8];
        f.read_exact(&mut content_size)?;
        let content_size = u64::from_le_bytes(content_size);

        let certificate_chain_size_with_padding = certificate_chain_size.div_ceil(0x40) * 0x40;
        let ticket_size_with_padding = ticket_size.div_ceil(0x40) * 0x40;
        let tmd_size_with_padding = tmd_size.div_ceil(0x40) * 0x40;
        let _meta_size_with_padding = meta_size.div_ceil(0x40) * 0x40;
        let content_size_with_padding = content_size.div_ceil(0x40) * 0x40;

        const CIA_HEADER_SIZE: u64 = 0x2040;
        let offset: u64 = CIA_HEADER_SIZE
            + u64::from(certificate_chain_size_with_padding)
            + u64::from(ticket_size_with_padding)
            + u64::from(tmd_size_with_padding)
            + content_size_with_padding;
        f.seek(SeekFrom::Start(offset))?;
        let smdh_icon = SMDHIcon::from_cia_meta(f)?;
        Ok(smdh_icon)
    }

    pub fn from_cia_meta<T: Read + Seek>(f: &mut T) -> Result<Self, N3DSParsingError> {
        const CIA_META_SMDH_OFFSET: i64 = 0x400;
        f.seek(SeekFrom::Current(CIA_META_SMDH_OFFSET))?;
        let smdh_icon = SMDHIcon::from_smdh(f)?;
        Ok(smdh_icon)
    }

    pub fn from_cci<T: Read + Seek>(f: &mut T) -> Result<Self, N3DSParsingError> {
        const CCI_HEADER_MAGIC_OFFSET: u64 = 0x100;
        f.seek(SeekFrom::Start(CCI_HEADER_MAGIC_OFFSET))?;
        let mut cci_magic = [0u8; 4];
        f.read_exact(&mut cci_magic)?;
        if b"NCSD" != &cci_magic {
            return Err(N3DSParsingError::FileMagicNotFound(
                FileMagicNotFound::NCSDMagicNotFound(cci_magic),
            ));
        }

        const CCI_HEADER_PARTITION_TABLE_OFFSET: u64 = 0x120;
        const CCI_HEADER_PARTITION_TABLE_SIZE: usize = 0x40;
        f.seek(SeekFrom::Start(CCI_HEADER_PARTITION_TABLE_OFFSET))?;
        let mut partiton_table = [0u8; CCI_HEADER_PARTITION_TABLE_SIZE];
        f.read_exact(&mut partiton_table)?;

        let partition_table: Vec<CCIPartition> = partiton_table
            .chunks_exact(8)
            .map(|chunk| CCIPartition::from_bytes(chunk.try_into().unwrap()))
            .collect();
        let Some(first_partition) = partition_table.first() else {
            return Err(N3DSParsingError::CCIErrorGettingExecutableContentPartition);
        };

        f.seek(SeekFrom::Start(first_partition.offset().into()))?;
        let smdh_icon = SMDHIcon::from_cxi(f)?;
        Ok(smdh_icon)
    }

    pub fn from_cxi<T: Read + Seek>(f: &mut T) -> Result<Self, N3DSParsingError> {
        const CXI_HEADER_MAGIC_OFFSET: i64 = 0x100;
        f.seek(SeekFrom::Current(CXI_HEADER_MAGIC_OFFSET))?;
        let mut cxi_magic = [0u8; 4];
        f.read_exact(&mut cxi_magic)?;
        if b"NCCH" != &cxi_magic {
            return Err(N3DSParsingError::FileMagicNotFound(
                FileMagicNotFound::NCCHMagicNotFound(cxi_magic),
            ));
        }

        const CXI_HEADER_FLAGS_OFFSET: i64 = 0x188;
        f.seek(SeekFrom::Current(
            CXI_HEADER_FLAGS_OFFSET - (CXI_HEADER_MAGIC_OFFSET + 4),
        ))?;
        let mut flags = [0u8; 8];
        f.read_exact(&mut flags)?;
        let flags_index_7 = flags[7];
        let is_no_crypto = (flags_index_7 & 0x4) == 0x4;
        if !is_no_crypto {
            return Err(N3DSParsingError::CXIParsingError(
                CXIParsingError::FileEncrypted,
            ));
        }

        const CXI_HEADER_EXEFS_OFFSET_VALUE: i64 = 0x1A0;
        f.seek(SeekFrom::Current(
            CXI_HEADER_EXEFS_OFFSET_VALUE - (CXI_HEADER_FLAGS_OFFSET + 8),
        ))?;

        let mut exefs_offset = [0u8; 4];
        f.read_exact(&mut exefs_offset)?;
        let exefs_offset = u32::from_le_bytes(exefs_offset); // in media units
        let exefs_offset = exefs_offset * 0x200;

        let mut exefs_size = [0u8; 4];
        f.read_exact(&mut exefs_size)?;
        let exefs_size = u32::from_le_bytes(exefs_size); // in media units
        let _exefs_size = exefs_size * 0x200;

        f.seek(SeekFrom::Current(
            exefs_offset as i64 - (CXI_HEADER_EXEFS_OFFSET_VALUE + 4 + 4),
        ))?;
        let smdh_icon = SMDHIcon::from_exefs(f)?;
        Ok(smdh_icon)
    }

    pub fn from_exefs<T: Read + Seek>(f: &mut T) -> Result<Self, N3DSParsingError> {
        const EXEFS_HEADER_FILE_HEADERS_SIZE: usize = 0xA0;
        let mut file_headers = [0u8; EXEFS_HEADER_FILE_HEADERS_SIZE];
        f.read_exact(&mut file_headers)?;

        let file_headers: Vec<ExeFSFileHeader> = file_headers
            .chunks_exact(16)
            .map(|chunk| ExeFSFileHeader::from_bytes(chunk.try_into().unwrap()))
            .collect::<Result<Vec<_>, _>>()?
            .into_iter()
            .flatten()
            .collect();

        let Some(icon_file) = file_headers.iter().find(|item| item.file_name() == b"icon") else {
            return Err(N3DSParsingError::CXIParsingError(
                CXIParsingError::ExeFSIconFileNotFound,
            ));
        };

        const EXEFS_HEADER_SIZE: i64 = 0x200;
        f.seek(SeekFrom::Current(
            EXEFS_HEADER_SIZE + i64::try_from(icon_file.file_offset()).unwrap()
                - EXEFS_HEADER_FILE_HEADERS_SIZE as i64,
        ))?;
        let smdh_icon = SMDHIcon::from_smdh(f)?;
        Ok(smdh_icon)
    }
}

#[derive(Debug, Clone)]
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
            _ => Err(Self::Error::MetaInvalidSize { 0: value }),
        }
    }
}

#[derive(Debug, Clone)]
pub struct CCIPartition {
    offset: u32,
    _length: u32,
}

impl CCIPartition {
    pub fn from_bytes(partition_bytes: &[u8; 8]) -> Self {
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

    pub fn _length(&self) -> u32 {
        self._length
    }
}

#[derive(Debug, Clone)]
pub struct ExeFSFileHeader {
    file_name: [u8; 8],
    file_offset: u32,
    _file_size: u32,
}

impl ExeFSFileHeader {
    pub fn from_bytes(file_headers_bytes: &[u8; 16]) -> Result<Option<Self>, N3DSParsingError> {
        // Each header is composed of 16 bytes, if the header is empty it will be filled with zeroes
        // Therefore we can read it as a u128 and check if it's results in a zero as a small optimization

        let is_empty = u128::from_ne_bytes(*file_headers_bytes);
        let is_empty = is_empty == 0;
        if is_empty {
            return Ok(None);
        }

        let file_name: [u8; 8] = file_headers_bytes[..8].try_into().unwrap();
        let file_offset = u32::from_le_bytes(file_headers_bytes[8..8 + 4].try_into().unwrap());
        let file_size = u32::from_le_bytes(file_headers_bytes[8 + 4..].try_into().unwrap());

        let exefs_file_header = ExeFSFileHeader {
            file_name,
            file_offset,
            _file_size: file_size,
        };
        Ok(Some(exefs_file_header))
    }

    pub fn file_name(&self) -> &[u8] {
        let len = self.file_name.iter().position(|p| *p == b'\0').unwrap_or(8);
        &self.file_name[..len]
    }

    pub fn file_offset(&self) -> u32 {
        self.file_offset
    }

    pub fn _file_size(&self) -> u32 {
        self._file_size
    }
}
