use gdk_pixbuf::Pixbuf;
use std::{
    io::{Read, Seek, SeekFrom},
    str,
};

use crate::n3ds::n3ds_parsing_errors::*;
use crate::utils::rgb565::Rgb565;

#[derive(Debug, Clone)]
pub struct SMDH {
    icon: SMDHIcon,
}

impl SMDH {
    pub fn from_data<T: Read + Seek>(
        file_data: &mut T,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let mut smdh_magic = [0u8; 4];
        file_data.read_exact(&mut smdh_magic)?;
        if "SMDH".as_bytes() != smdh_magic {
            return Err(Box::new(N3DSParsingErrorSMDHMagicNotFound));
        }

        file_data.seek(SeekFrom::Current(0x24C0 - 0x04))?;
        let mut large_icon_bytes = [0u8; 0x1200];
        file_data.read_exact(&mut large_icon_bytes)?;
        let icon = SMDHIcon::try_from(&large_icon_bytes)?;

        let smdh = SMDH { icon };
        Ok(smdh)
    }

    pub fn get_icon(&self) -> SMDHIcon {
        self.icon.clone()
    }
}

#[derive(Debug, Clone)]
pub struct SMDHIcon {
    large_icon: Pixbuf,
}

impl TryFrom<&[u8; 0x1200]> for SMDHIcon {
    type Error = Box<dyn std::error::Error>;

    fn try_from(large_icon_bytes: &[u8; 0x1200]) -> Result<Self, Self::Error> {
        let large_icon_data: Vec<Rgb565> = large_icon_bytes
            .chunks_exact(2)
            .map(|chunk| u16::from_le_bytes(chunk.try_into().unwrap()))
            .map(|color| Rgb565::try_from(color))
            .collect::<Result<Vec<_>, _>>()?;

        let icon = match SMDHIcon::generate_n3ds_pixbuf(&large_icon_data) {
            Some(large_icon) => SMDHIcon { large_icon },
            None => return Err(Box::new(UnableToExtractN3DSIcon)),
        };
        Ok(icon)
    }
}

impl SMDHIcon {
    fn generate_n3ds_pixbuf(large_icon_data: &[Rgb565]) -> Option<Pixbuf> {
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

    pub fn get_large_icon(&self) -> Pixbuf {
        self.large_icon.clone()
    }
}

#[derive(Debug, Clone)]
pub struct N3DSX {
    smdh: SMDH,
}

impl N3DSX {
    pub fn from_data<T: Read + Seek>(
        file_data: &mut T,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let mut n3dsx_magic = [0u8; 4];
        file_data.read_exact(&mut n3dsx_magic)?;
        if "3DSX".as_bytes() != n3dsx_magic {
            return Err(Box::new(N3DSParsingError3DSXMagicNotFound));
        }

        let mut header_size = [0u8; 2];
        file_data.read_exact(&mut header_size)?;
        let header_size = u16::from_le_bytes(header_size);
        if !(header_size > 32) {
            return Err(Box::new(N3DSParsingError3DSXNoExtendedHeader {
                0: header_size,
            }));
        }

        file_data.seek(SeekFrom::Start(0x20))?;
        let mut smdh_offset = [0u8; 4];
        file_data.read_exact(&mut smdh_offset)?;
        let smdh_offset = u32::from_le_bytes(smdh_offset);

        let mut smdh_size = [0u8; 4];
        file_data.read_exact(&mut smdh_size)?;
        let _smdh_size = u32::from_le_bytes(smdh_size);

        file_data.seek(SeekFrom::Start(smdh_offset.into()))?;
        let smdh = SMDH::from_data(file_data)?;
        let n3dsx = N3DSX { smdh };
        Ok(n3dsx)
    }

    pub fn get_smdh(&self) -> SMDH {
        self.smdh.clone()
    }
}

pub struct CIA {
    meta: CIAMeta,
}

impl CIA {
    pub fn from_data<T: Read + Seek>(
        file_data: &mut T,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        /*
         * The meta section isn't in a fixed place and is located after a bunch of sections whose
         * size can vary, therefore it's needed to at the very last fetch the other sizes and
         * take the padding into account
         */

        file_data.seek(SeekFrom::Start(0x08))?;
        let mut certificate_chain_size = [0u8; 4];
        file_data.read_exact(&mut certificate_chain_size)?;
        let certificate_chain_size = u32::from_le_bytes(certificate_chain_size);

        let mut ticket_size = [0u8; 4];
        file_data.read_exact(&mut ticket_size)?;
        let ticket_size = u32::from_le_bytes(ticket_size);

        let mut tmd_size = [0u8; 4];
        file_data.read_exact(&mut tmd_size)?;
        let tmd_size = u32::from_le_bytes(tmd_size);

        let mut meta_size = [0u8; 4];
        file_data.read_exact(&mut meta_size)?;
        let meta_size = u32::from_le_bytes(meta_size);

        let meta_size = CIAMetaSize::try_from(meta_size)?;
        let meta_size: u32 = match meta_size {
            CIAMetaSize::Present => 0x3AC0,
            _ => {
                return Err(Box::new(N3DSCIAParsingErrorMetaNotExpectedValue {
                    0: meta_size,
                }))
            }
        };

        let mut content_size = [0u8; 8];
        file_data.read_exact(&mut content_size)?;
        let content_size = u64::from_le_bytes(content_size);

        let certificate_chain_size_with_padding = certificate_chain_size.div_ceil(0x40) * 0x40;
        let ticket_size_with_padding = ticket_size.div_ceil(0x40) * 0x40;
        let tmd_size_with_padding = tmd_size.div_ceil(0x40) * 0x40;
        let _meta_size_with_padding = meta_size.div_ceil(0x40) * 0x40;
        let content_size_with_padding = content_size.div_ceil(0x40) * 0x40;

        let sections_offset: u64 = u64::from(certificate_chain_size_with_padding)
            + u64::from(ticket_size_with_padding)
            + u64::from(tmd_size_with_padding)
            + content_size_with_padding;
        file_data.seek(SeekFrom::Start(0x2040 + sections_offset))?;
        let meta = CIAMeta::from_data(file_data)?;
        let cia = CIA { meta };
        Ok(cia)
    }

    pub fn get_meta(&self) -> CIAMeta {
        self.meta.clone()
    }
}
#[derive(Debug, Clone)]
pub struct CIAMeta {
    icon_data: SMDH,
}

impl CIAMeta {
    pub fn from_data<T: Read + Seek>(
        file_data: &mut T,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        file_data.seek(SeekFrom::Current(0x400))?;
        let icon_data = SMDH::from_data(file_data)?;
        let cia_meta = CIAMeta { icon_data };
        Ok(cia_meta)
    }

    pub fn get_icon_data(&self) -> SMDH {
        self.icon_data.clone()
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
    type Error = N3DSCIAParsingErrorMetaInvalidSize;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(CIAMetaSize::None),
            8 => Ok(CIAMetaSize::CVerUSA),
            0x200 => Ok(CIAMetaSize::Dummy),
            0x3AC0 => Ok(CIAMetaSize::Present),
            _ => Err(Self::Error { 0: value }),
        }
    }
}

#[derive(Debug, Clone)]
pub struct CCI {
    cxi: CXI,
}

impl CCI {
    pub fn from_data<T: Read + Seek>(
        file_data: &mut T,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        file_data.seek(SeekFrom::Start(0x100))?;
        let mut cci_magic = [0u8; 4];
        file_data.read_exact(&mut cci_magic)?;
        if "NCSD".as_bytes() != cci_magic {
            return Err(Box::new(N3DSParsingErrorCCIMagicNotFound));
        }

        file_data.seek(SeekFrom::Start(0x120))?;
        let partition_table = (0..8)
            .map(|_| CCIPartition::from_data(file_data))
            .collect::<Result<Vec<_>, _>>()?;

        let first_partition = match partition_table.first() {
            Some(x) => x,
            None => {
                return Err(Box::new(
                    N3DSParsingErrorCCIErrorGettingExecutableContentPartition,
                ))
            }
        };

        file_data.seek(SeekFrom::Start(first_partition.offset().into()))?;
        let cxi = CXI::from_data(file_data)?;
        let cci = CCI { cxi };
        Ok(cci)
    }

    pub fn get_cxi(&self) -> CXI {
        self.cxi.clone()
    }
}

#[derive(Debug, Clone)]
pub struct CCIPartition {
    offset: u32,
    _length: u32,
}

impl CCIPartition {
    pub fn from_data<T: Read + Seek>(
        file_data: &mut T,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let mut offset = [0u8; 0x4];
        file_data.read_exact(&mut offset)?;
        let offset = u32::from_le_bytes(offset); //in media units
        let offset = offset * 0x200;

        let mut length = [0u8; 0x4];
        file_data.read_exact(&mut length)?;
        let length = u32::from_le_bytes(length); //in media units
        let length = length * 0x200;

        let cci_partition = CCIPartition {
            offset,
            _length: length,
        };
        Ok(cci_partition)
    }

    pub fn offset(&self) -> u32 {
        self.offset
    }

    pub fn _length(&self) -> u32 {
        self._length
    }
}

#[derive(Debug, Clone)]
pub struct CXI {
    _is_decrypted: bool,
    exefs: Option<ExeFS>,
}

impl CXI {
    pub fn from_data<T: Read + Seek>(
        file_data: &mut T,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        file_data.seek(SeekFrom::Current(0x100))?;
        let mut cxi_magic = [0u8; 4];
        file_data.read_exact(&mut cxi_magic)?;
        if "NCCH".as_bytes() != cxi_magic {
            return Err(Box::new(N3DSParsingErrorCXIMagicNotFound));
        }

        file_data.seek(SeekFrom::Current(0x188 - 0x104))?;
        let mut flags = [0u8; 8];
        file_data.read_exact(&mut flags)?;
        let flags_index_7 = flags[7];
        let is_no_crypto = (flags_index_7 & 0x4) == 0x4;

        if !is_no_crypto {
            return Ok(CXI {
                _is_decrypted: false,
                exefs: None,
            });
        }

        file_data.seek(SeekFrom::Current(0x1A0 - 0x190))?;
        let mut exefs_offset = [0u8; 4];
        file_data.read_exact(&mut exefs_offset)?;
        let exefs_offset = u32::from_le_bytes(exefs_offset); // in media units
        let exefs_offset = exefs_offset * 0x200;

        let mut exefs_size = [0u8; 4];
        file_data.read_exact(&mut exefs_size)?;
        let exefs_size = u32::from_le_bytes(exefs_size); // in media units
        let _exefs_size = exefs_size * 0x200;

        file_data.seek(SeekFrom::Current(exefs_offset as i64 - 0x1A8))?;
        let exefs = ExeFS::from_data(file_data)?;
        let cxi = CXI {
            _is_decrypted: true,
            exefs: Some(exefs),
        };
        Ok(cxi)
    }

    pub fn _is_decrypted(&self) -> bool {
        self._is_decrypted
    }

    pub fn get_exefs(&self) -> Option<ExeFS> {
        self.exefs.clone()
    }
}

#[derive(Debug, Clone)]
pub struct ExeFS {
    icon: SMDH,
}

impl ExeFS {
    pub fn from_data<T: Read + Seek>(
        file_data: &mut T,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let file_headers: Vec<ExeFSFileHeader> = (0..10)
            .map(|_| ExeFSFileHeader::from_data(file_data))
            .collect::<Result<Vec<_>, _>>()?
            .into_iter()
            .flatten()
            .collect();

        let icon_file = match file_headers.iter().find(|item| item.file_name() == "icon") {
            Some(x) => x,
            None => return Err(Box::new(N3DSParsingErrorExeFSIconFileNotFound)),
        };

        file_data.seek(SeekFrom::Current(
            0x200 + icon_file.file_offset() as i64 - 0xA0,
        ))?;
        let smdh = SMDH::from_data(file_data)?;
        let exefs = ExeFS { icon: smdh };
        Ok(exefs)
    }

    pub fn get_icon_file(&self) -> SMDH {
        self.icon.clone()
    }
}

#[derive(Debug, Clone)]
pub struct ExeFSFileHeader {
    file_name: String,
    file_offset: u32,
    _file_size: u32,
}

impl ExeFSFileHeader {
    pub fn from_data<T: Read + Seek>(
        file_data: &mut T,
    ) -> Result<Option<Self>, Box<dyn std::error::Error>> {
        // Each header is composed of 16 bytes, if the header is empty it will be filled with zeroes
        // Therefore we can read it as a u128 and check if it's results in a zero as a small optimization

        let mut file_header = [0u8; 16];
        file_data.read_exact(&mut file_header)?;

        let is_empty = u128::from_ne_bytes(file_header);
        let is_empty = is_empty == 0;
        if is_empty {
            return Ok(None);
        }

        let file_name = str::from_utf8(&file_header[..0x8])?
            .trim_matches(char::from(0))
            .to_owned();
        let file_offset = u32::from_le_bytes(file_header[0x8..0x8 + 4].try_into().unwrap());
        let file_size = u32::from_le_bytes(file_header[0x8 + 4..].try_into().unwrap());

        let exefs_file_header = ExeFSFileHeader {
            file_name,
            file_offset,
            _file_size: file_size,
        };
        Ok(Some(exefs_file_header))
    }

    pub fn file_name(&self) -> &str {
        &self.file_name
    }

    pub fn file_offset(&self) -> u32 {
        self.file_offset
    }

    pub fn _file_size(&self) -> u32 {
        self._file_size
    }
}
