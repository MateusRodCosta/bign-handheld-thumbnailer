mod cci;
mod cia;
mod cxi;

use image::{ImageBuffer, Rgba, RgbaImage};
use std::io::{Read, Seek, SeekFrom};

use crate::n3ds::errors::ParsingError;
use crate::utils::Rgb888;

use cci::*;
use cia::*;
use cxi::*;

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
 * Do note that the Meta section containing a SMHD might or might not be present on .cia files.
 * If the Meta section isn't present in a CIA, the tool tries to extract the icon from the CXI
 * located inside the CIA.
 *
 * Do also note that the extended header with a SMHD is optional for the .3dsx file format.
*/

#[derive(Debug, Clone)]
pub struct SMDHIcon {
    large_icon: ImageBuffer<Rgba<u8>, Vec<u8>>,
}

impl SMDHIcon {
    pub fn get_large_icon(&self) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
        self.large_icon.clone()
    }

    fn generate_icon_from_bytes(large_icon_bytes: &[u8; 0x1200]) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
        let large_icon_data: [Rgb888; 0x1200 / 2] = large_icon_bytes
            .chunks_exact(2)
            .map(|chunk| Rgb888::from_rgb565_bytes(chunk.try_into().unwrap()))
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();

        /*
         * The large 3DS icon is 48x48 px and divided in tiles according to Morton order
         * Each color will usually be RGB565 although it's not the only supported color enconding
         */

        /*
         * Due to the Morton order, the code for the coordinates of the pixels is oxided from
         * https://github.com/GEMISIS/SMDH-Creator/blob/master/SMDH-Creator/SMDH.cs#L255
         */

        let mut img = RgbaImage::new(48, 48);

        let tile_order = [
            0, 1, 8, 9, 2, 3, 10, 11, 16, 17, 24, 25, 18, 19, 26, 27, 4, 5, 12, 13, 6, 7, 14, 15,
            20, 21, 28, 29, 22, 23, 30, 31, 32, 33, 40, 41, 34, 35, 42, 43, 48, 49, 56, 57, 50, 51,
            58, 59, 36, 37, 44, 45, 38, 39, 46, 47, 52, 53, 60, 61, 54, 55, 62, 63,
        ];

        let mut pos = 0;
        for tile_y in 0..6 {
            let tile_y_offset = tile_y * 8;
            for tile_x in 0..6 {
                let tile_x_offset = tile_x * 8;
                for tile_pos in tile_order {
                    let x = tile_pos & 0x7;
                    let y = tile_pos >> 3;
                    let coords = (x + tile_x_offset, y + tile_y_offset);

                    let rgb = &large_icon_data[pos];
                    img.put_pixel(coords.0, coords.1, Rgba([rgb.r(), rgb.g(), rgb.b(), 0xFF]));

                    pos += 1;
                }
            }
        }

        img
    }
}

impl SMDHIcon {
    pub fn from_smdh<T: Read + Seek>(f: &mut T) -> Result<Self, ParsingError> {
        const SMDH_LARGE_ICON_OFFSET: i64 = 0x24C0;
        const SMDH_LARGE_ICON_SIZE: usize = 0x1200;

        let mut smdh_magic = [0u8; 4];
        f.read_exact(&mut smdh_magic)?;
        if b"SMDH" != &smdh_magic {
            return Err(ParsingError::FileMagicNotFound("SMDH", smdh_magic));
        }

        f.seek(SeekFrom::Current(SMDH_LARGE_ICON_OFFSET - 4))?;
        let mut large_icon_bytes = [0u8; SMDH_LARGE_ICON_SIZE];
        f.read_exact(&mut large_icon_bytes)?;
        Ok(SMDHIcon {
            large_icon: SMDHIcon::generate_icon_from_bytes(&large_icon_bytes),
        })
    }

    pub fn from_n3dsx<T: Read + Seek>(f: &mut T) -> Result<Self, ParsingError> {
        const N3DSX_EXTENDED_HEADER_OFFSET: u64 = 0x20;

        let mut n3dsx_magic = [0u8; 4];
        f.read_exact(&mut n3dsx_magic)?;
        if b"3DSX" != &n3dsx_magic {
            return Err(ParsingError::FileMagicNotFound("3DSX", n3dsx_magic));
        }

        let mut header_size = [0u8; 2];
        f.read_exact(&mut header_size)?;
        let header_size = u16::from_le_bytes(header_size);
        if header_size <= 32 {
            return Err(ParsingError::N3DSXParsingError3DSXNoExtendedHeader(
                header_size,
            ));
        }

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

    pub fn from_cia<T: Read + Seek>(f: &mut T) -> Result<Self, ParsingError> {
        /*
         * The meta section isn't in a fixed place and is located after a bunch of sections whose
         * size can vary, therefore it's needed to at the very last fetch the other sizes and
         * take the padding into account
         */

        const CIA_HEADER_CERTIFICATE_CHAIN_SIZE_OFFSET: u64 = 0x08;
        const CIA_HEADER_SIZE: u64 = 0x2040;

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

        let mut content_size = [0u8; 8];
        f.read_exact(&mut content_size)?;
        let content_size = u64::from_le_bytes(content_size);

        let certificate_chain_size_with_padding = certificate_chain_size.div_ceil(0x40) * 0x40;
        let ticket_size_with_padding = ticket_size.div_ceil(0x40) * 0x40;
        let tmd_size_with_padding = tmd_size.div_ceil(0x40) * 0x40;
        let content_size_with_padding = content_size.div_ceil(0x40) * 0x40;

        println!("Trying to parse icon from CIA Meta section...");
        if meta_size != CIAMetaSize::Present {
            println!("Meta section not present, skipping");
        } else {
            let offset_meta: u64 = CIA_HEADER_SIZE
                + u64::from(certificate_chain_size_with_padding)
                + u64::from(ticket_size_with_padding)
                + u64::from(tmd_size_with_padding)
                + content_size_with_padding;

            f.seek(SeekFrom::Start(offset_meta))?;
            let meta_smdh_icon = SMDHIcon::from_cia_meta(f)?;
            return Ok(meta_smdh_icon);
        }

        let offset_tmd: u64 = CIA_HEADER_SIZE
            + u64::from(certificate_chain_size_with_padding)
            + u64::from(ticket_size_with_padding);
        f.seek(SeekFrom::Start(offset_tmd))?;

        let offset_content: u64 = CIA_HEADER_SIZE
            + u64::from(certificate_chain_size_with_padding)
            + u64::from(ticket_size_with_padding)
            + u64::from(tmd_size_with_padding);

        println!("Trying to parse SMDH from CIA's CXI");
        match SMDHIcon::from_cia_tmd(f, offset_content) {
            Ok(Some(icon)) => Ok(icon),
            Ok(None) => {
                println!("Failed to parse SMDH from CIA's CXI");
                return Err(ParsingError::CIAHasNoIconAvailable);
            },
            Err(error) => {
                println!("{}", error);
                return Err(ParsingError::CIAHasNoIconAvailable);
            }
        }
    }

    pub fn from_cia_meta<T: Read + Seek>(f: &mut T) -> Result<Self, ParsingError> {
        const CIA_META_SMDH_OFFSET: i64 = 0x400;
        f.seek(SeekFrom::Current(CIA_META_SMDH_OFFSET))?;
        let smdh_icon = SMDHIcon::from_smdh(f)?;
        Ok(smdh_icon)
    }

    pub fn from_cci<T: Read + Seek>(f: &mut T) -> Result<Self, ParsingError> {
        const CCI_HEADER_MAGIC_OFFSET: u64 = 0x100;
        const CCI_HEADER_PARTITION_TABLE_OFFSET: u64 = 0x120;
        const CCI_HEADER_PARTITION_TABLE_SIZE: usize = 0x40;

        f.seek(SeekFrom::Start(CCI_HEADER_MAGIC_OFFSET))?;
        let mut cci_magic = [0u8; 4];
        f.read_exact(&mut cci_magic)?;
        if b"NCSD" != &cci_magic {
            return Err(ParsingError::FileMagicNotFound("NCSD", cci_magic));
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
            return Err(ParsingError::CCIErrorGettingExecutableContentPartition);
        };

        f.seek(SeekFrom::Start(first_partition.offset().into()))?;
        let smdh_icon = SMDHIcon::from_cxi(f)?;
        Ok(smdh_icon)
    }

    pub fn from_cia_tmd<T: Read + Seek>(
        f: &mut T,
        content_offset: u64,
    ) -> Result<Option<Self>, ParsingError> {
        let title_metadata = CIATitleMetadata::from_file(f)?;

        f.seek(SeekFrom::Start(content_offset))?;
        let Some(first_content) = title_metadata.content_chunk_records().first() else {
            return Ok(None);
        };

        match first_content.content_index() {
            CIAContentIndex::MainContent => (),
            _ => {
                return Ok(None);
            }
        }

        match first_content.content_type() {
            1 => {
                return Err(ParsingError::CXIFileEncrypted);
            }
            _ => (),
        };

        Ok(Some(SMDHIcon::from_cxi(f)?))
    }

    pub fn from_cxi<T: Read + Seek>(f: &mut T) -> Result<Self, ParsingError> {
        const CXI_HEADER_MAGIC_OFFSET: i64 = 0x100;
        const CXI_HEADER_FLAGS_OFFSET: i64 = 0x188;
        const CXI_HEADER_EXEFS_OFFSET_VALUE: i64 = 0x1A0;

        f.seek(SeekFrom::Current(CXI_HEADER_MAGIC_OFFSET))?;
        let mut cxi_magic = [0u8; 4];
        f.read_exact(&mut cxi_magic)?;
        if b"NCCH" != &cxi_magic {
            return Err(ParsingError::FileMagicNotFound("NCCH", cxi_magic));
        }

        f.seek(SeekFrom::Current(
            CXI_HEADER_FLAGS_OFFSET - (CXI_HEADER_MAGIC_OFFSET + 4),
        ))?;
        let mut flags = [0u8; 8];
        f.read_exact(&mut flags)?;
        let flags_index_7 = flags[7];
        let is_no_crypto = (flags_index_7 & 0x4) == 0x4;
        if !is_no_crypto {
            return Err(ParsingError::CXIFileEncrypted);
        }

        f.seek(SeekFrom::Current(
            CXI_HEADER_EXEFS_OFFSET_VALUE - (CXI_HEADER_FLAGS_OFFSET + 8),
        ))?;

        let mut exefs_offset = [0u8; 4];
        f.read_exact(&mut exefs_offset)?;
        let exefs_offset = u32::from_le_bytes(exefs_offset); // in media units
        let exefs_offset = exefs_offset * 0x200;

        let mut exefs_size = [0u8; 4];
        f.read_exact(&mut exefs_size)?;

        f.seek(SeekFrom::Current(
            i64::from(exefs_offset) - (CXI_HEADER_EXEFS_OFFSET_VALUE + 4 + 4),
        ))?;
        let smdh_icon = SMDHIcon::from_exefs(f)?;
        Ok(smdh_icon)
    }

    pub fn from_exefs<T: Read + Seek>(f: &mut T) -> Result<Self, ParsingError> {
        const EXEFS_HEADER_FILE_HEADERS_SIZE: usize = 0xA0;
        const EXEFS_HEADER_SIZE: i64 = 0x200;

        let mut file_headers = [0u8; EXEFS_HEADER_FILE_HEADERS_SIZE];
        f.read_exact(&mut file_headers)?;
        let mut file_headers = file_headers
            .chunks_exact(16)
            .filter_map(|chunk| ExeFSFileHeader::from_bytes(chunk.try_into().unwrap()));
        let Some(icon_file) = file_headers.find(|item| item.file_name() == b"icon") else {
            return Err(ParsingError::CXIExeFSIconFileNotFound);
        };

        f.seek(SeekFrom::Current(
            EXEFS_HEADER_SIZE + i64::from(icon_file.file_offset())
                - i64::try_from(EXEFS_HEADER_FILE_HEADERS_SIZE).unwrap(),
        ))?;
        let smdh_icon = SMDHIcon::from_smdh(f)?;
        Ok(smdh_icon)
    }
}
