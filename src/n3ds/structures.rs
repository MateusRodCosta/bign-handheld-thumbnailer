mod cci;
mod cia;
mod cxi;

use image::{ImageBuffer, Rgba, RgbaImage};
use memmap2::Mmap;

use crate::n3ds::errors::{CIAParsingError, CXIParsingError, ParsingError};
use crate::utils::Rgb888;
use cci::CCIPartition;
use cia::{CIAContentIndex, CIAMetaSize, CIATitleMetadata};
use cxi::ExeFSFileHeader;

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

        const LARGE_ICON_SIZE: usize = 48;
        const LARGE_ICON_WIDTH: usize = LARGE_ICON_SIZE;
        const LARGE_ICON_HEIGHT: usize = LARGE_ICON_SIZE;
        /*
         * The large 3DS icon is 48x48 px and divided in tiles according to Morton order
         * Each color will usually be RGB565 although it's not the only supported color enconding
         */

        /*
         * Due to the Morton order, the code for the coordinates of the pixels is oxided from
         * https://github.com/ihaveamac/pyctr/blob/master/pyctr/type/smdh.py
         * Many thanks to ihaveamac from the Nintendo Homebrew Discord for the help
         */

        let mut img = RgbaImage::new(LARGE_ICON_WIDTH as u32, LARGE_ICON_HEIGHT as u32);

        for y in 0..LARGE_ICON_HEIGHT {
            for x in 0..LARGE_ICON_WIDTH {
                let pixel_offset: usize = (((y >> 3) * (LARGE_ICON_WIDTH >> 3) + (x >> 3)) << 6)
                    + ((x & 1)
                        | ((y & 1) << 1)
                        | ((x & 2) << 1)
                        | ((y & 2) << 2)
                        | ((x & 4) << 2)
                        | ((y & 4) << 3));

                let pixel = &large_icon_data[pixel_offset];
                img.put_pixel(
                    x as u32,
                    y as u32,
                    Rgba([pixel.r(), pixel.g(), pixel.b(), 0xFF]),
                );
            }
        }

        img
    }
}

impl SMDHIcon {
    pub fn from_smdh(input: &Mmap, offset: Option<usize>) -> Result<Self, ParsingError> {
        const SMDH_LARGE_ICON_OFFSET: usize = 0x24C0;
        const SMDH_LARGE_ICON_SIZE: usize = 0x1200;

        let offset: usize = offset.unwrap_or(0);

        let smdh_magic: [u8; 4] = input[offset..offset + 4].try_into().unwrap();
        if b"SMDH" != &smdh_magic {
            return Err(ParsingError::FileMagicNotFound("SMDH", smdh_magic));
        }

        let large_icon_bytes: [u8; SMDH_LARGE_ICON_SIZE] = input[offset + SMDH_LARGE_ICON_OFFSET
            ..offset + SMDH_LARGE_ICON_OFFSET + SMDH_LARGE_ICON_SIZE]
            .try_into()
            .unwrap();
        Ok(SMDHIcon {
            large_icon: SMDHIcon::generate_icon_from_bytes(&large_icon_bytes),
        })
    }

    pub fn from_n3dsx(input: &Mmap) -> Result<Self, ParsingError> {
        const N3DSX_EXTENDED_HEADER_OFFSET: usize = 0x20;

        let n3dsx_magic: [u8; 4] = input[..4].try_into().unwrap();
        if b"3DSX" != &n3dsx_magic {
            return Err(ParsingError::FileMagicNotFound("3DSX", n3dsx_magic));
        }

        let header_size: [u8; 2] = input[4..4 + 2].try_into().unwrap();
        let header_size = u16::from_le_bytes(header_size);
        if header_size <= 32 {
            return Err(ParsingError::N3DSXParsingError3DSXNoExtendedHeader(
                header_size,
            ));
        }

        let smdh_offset: [u8; 4] = input
            [N3DSX_EXTENDED_HEADER_OFFSET..N3DSX_EXTENDED_HEADER_OFFSET + 4]
            .try_into()
            .unwrap();
        let smdh_offset: usize = u32::from_le_bytes(smdh_offset).try_into().unwrap();
        let smdh_icon = SMDHIcon::from_smdh(input, Some(smdh_offset))?;
        Ok(smdh_icon)
    }

    pub fn from_cia(input: &Mmap) -> Result<Self, ParsingError> {
        /*
         * The meta section isn't in a fixed place and is located after a bunch of sections whose
         * size can vary, therefore it's needed to at the very last fetch the other sizes and
         * take the padding into account
         */
        const CIA_HEADER_CERTIFICATE_CHAIN_SIZE_OFFSET: usize = 0x08;
        const CIA_HEADER_TICKET_SIZE_OFFSET: usize = 0x0C;
        const CIA_HEADER_TMD_SIZE_OFFSET: usize = 0x10;
        const CIA_HEADER_META_SIZE_OFFSET: usize = 0x14;
        const CIA_HEADER_CONTENT_SIZE_OFFSET: usize = 0x18;
        const CIA_HEADER_SIZE: usize = 0x2040;
        const CIA_PADDING_SIZE: usize = 0x40;

        let certificate_chain_size: [u8; 4] = input[CIA_HEADER_CERTIFICATE_CHAIN_SIZE_OFFSET
            ..CIA_HEADER_CERTIFICATE_CHAIN_SIZE_OFFSET + 4]
            .try_into()
            .unwrap();
        let certificate_chain_size: usize = u32::from_le_bytes(certificate_chain_size)
            .try_into()
            .unwrap();

        let ticket_size: [u8; 4] = input
            [CIA_HEADER_TICKET_SIZE_OFFSET..CIA_HEADER_TICKET_SIZE_OFFSET + 4]
            .try_into()
            .unwrap();
        let ticket_size: usize = u32::from_le_bytes(ticket_size).try_into().unwrap();

        let tmd_size: [u8; 4] = input[CIA_HEADER_TMD_SIZE_OFFSET..CIA_HEADER_TMD_SIZE_OFFSET + 4]
            .try_into()
            .unwrap();
        let tmd_size: usize = u32::from_le_bytes(tmd_size).try_into().unwrap();

        let meta_size: [u8; 4] = input
            [CIA_HEADER_META_SIZE_OFFSET..CIA_HEADER_META_SIZE_OFFSET + 4]
            .try_into()
            .unwrap();
        let meta_size = u32::from_le_bytes(meta_size);
        let meta_size = CIAMetaSize::try_from(meta_size)?;

        let content_size: [u8; 8] = input
            [CIA_HEADER_CONTENT_SIZE_OFFSET..CIA_HEADER_CONTENT_SIZE_OFFSET + 8]
            .try_into()
            .unwrap();
        let content_size: usize = u64::from_le_bytes(content_size).try_into().unwrap();

        let certificate_chain_size_with_padding =
            certificate_chain_size.next_multiple_of(CIA_PADDING_SIZE);
        let ticket_size_with_padding = ticket_size.next_multiple_of(CIA_PADDING_SIZE);
        let tmd_size_with_padding = tmd_size.next_multiple_of(CIA_PADDING_SIZE);
        let content_size_with_padding = content_size.next_multiple_of(CIA_PADDING_SIZE);

        eprintln!("Trying to parse icon from CIA Meta section...");
        if meta_size == CIAMetaSize::Present {
            let offset_meta: usize = CIA_HEADER_SIZE
                + certificate_chain_size_with_padding
                + ticket_size_with_padding
                + tmd_size_with_padding
                + content_size_with_padding;

            let meta_smdh_icon = SMDHIcon::from_cia_meta(input, offset_meta)?;
            return Ok(meta_smdh_icon);
        }
        eprintln!("Meta section not present, skipping");

        let offset_tmd: usize =
            CIA_HEADER_SIZE + certificate_chain_size_with_padding + ticket_size_with_padding;
        let offset_content: usize = CIA_HEADER_SIZE
            + certificate_chain_size_with_padding
            + ticket_size_with_padding
            + tmd_size_with_padding;

        eprintln!("Trying to parse SMDH from CIA's CXI");
        match SMDHIcon::from_cia_tmd(input, offset_tmd, offset_content) {
            Ok(icon) => Ok(icon),
            Err(error) => {
                eprintln!("Failed to parse SMDH from CIA's CXI");
                Err(error)
            }
        }
    }

    pub fn from_cia_meta(input: &Mmap, offset: usize) -> Result<Self, ParsingError> {
        const CIA_META_SMDH_OFFSET: usize = 0x400;

        let smdh_icon = SMDHIcon::from_smdh(input, Some(offset + CIA_META_SMDH_OFFSET))?;
        Ok(smdh_icon)
    }

    pub fn from_cci(input: &Mmap) -> Result<Self, ParsingError> {
        const CCI_HEADER_MAGIC_OFFSET: usize = 0x100;
        const CCI_HEADER_PARTITION_TABLE_OFFSET: usize = 0x120;
        const CCI_HEADER_PARTITION_TABLE_SIZE: usize = 0x40;

        let cci_magic: [u8; 4] = input[CCI_HEADER_MAGIC_OFFSET..CCI_HEADER_MAGIC_OFFSET + 4]
            .try_into()
            .unwrap();
        if b"NCSD" != &cci_magic {
            return Err(ParsingError::FileMagicNotFound("NCSD", cci_magic));
        }

        let partition_table: [u8; CCI_HEADER_PARTITION_TABLE_SIZE] = input
            [CCI_HEADER_PARTITION_TABLE_OFFSET
                ..CCI_HEADER_PARTITION_TABLE_OFFSET + CCI_HEADER_PARTITION_TABLE_SIZE]
            .try_into()
            .unwrap();
        let partition_table: [CCIPartition; CCI_HEADER_PARTITION_TABLE_SIZE / 8] = partition_table
            .chunks_exact(8)
            .map(|chunk| CCIPartition::from_bytes(chunk.try_into().unwrap()))
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();
        let Some(first_partition) = partition_table.first() else {
            return Err(ParsingError::CCIErrorGettingExecutableContentPartition);
        };

        let first_partition_offset: usize = first_partition.offset().try_into().unwrap();
        let smdh_icon = SMDHIcon::from_cxi(input, Some(first_partition_offset))?;
        Ok(smdh_icon)
    }

    pub fn from_cia_tmd(
        input: &Mmap,
        offset_tmd: usize,
        offset_content: usize,
    ) -> Result<Self, ParsingError> {
        let title_metadata = CIATitleMetadata::from_input(input, offset_tmd)?;

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

        SMDHIcon::from_cxi(input, Some(offset_content))
    }

    pub fn from_cxi(input: &Mmap, offset: Option<usize>) -> Result<Self, ParsingError> {
        const CXI_HEADER_MAGIC_OFFSET: usize = 0x100;
        const CXI_HEADER_FLAGS_OFFSET: usize = 0x188;
        const CXI_HEADER_EXEFS_OFFSET_VALUE: usize = 0x1A0;
        const CXI_MEDIA_UNIT_SIZE: usize = 0x200;

        let offset: usize = offset.unwrap_or(0);

        let cxi_magic: [u8; 4] = input
            [offset + CXI_HEADER_MAGIC_OFFSET..offset + CXI_HEADER_MAGIC_OFFSET + 4]
            .try_into()
            .unwrap();
        if b"NCCH" != &cxi_magic {
            return Err(ParsingError::FileMagicNotFound("NCCH", cxi_magic));
        }

        let flags: [u8; 8] = input
            [offset + CXI_HEADER_FLAGS_OFFSET..offset + CXI_HEADER_FLAGS_OFFSET + 8]
            .try_into()
            .unwrap();
        let flags_index_7 = flags[7];
        if (flags_index_7 & 0x4) != 0x4 {
            return Err(CXIParsingError::FileEncrypted.into());
        }

        let exefs_offset: [u8; 4] = input
            [offset + CXI_HEADER_EXEFS_OFFSET_VALUE..offset + CXI_HEADER_EXEFS_OFFSET_VALUE + 4]
            .try_into()
            .unwrap();
        let exefs_offset: usize = u32::from_le_bytes(exefs_offset).try_into().unwrap(); // in media units
        let exefs_offset: usize = exefs_offset * CXI_MEDIA_UNIT_SIZE;

        let smdh_icon = SMDHIcon::from_exefs(input, offset + exefs_offset)?;
        Ok(smdh_icon)
    }

    pub fn from_exefs(input: &Mmap, offset: usize) -> Result<Self, ParsingError> {
        const EXEFS_HEADER_FILE_HEADERS_SIZE: usize = 0xA0;
        const EXEFS_HEADER_SIZE: usize = 0x200;

        let file_headers: [u8; EXEFS_HEADER_FILE_HEADERS_SIZE] = input
            [offset..offset + EXEFS_HEADER_FILE_HEADERS_SIZE]
            .try_into()
            .unwrap();
        let mut file_headers = file_headers
            .chunks_exact(16)
            .filter_map(|chunk| ExeFSFileHeader::from_bytes(chunk.try_into().unwrap()));
        let Some(icon_file) = file_headers.find(|item| item.file_name() == b"icon") else {
            return Err(CXIParsingError::ExeFSIconFileNotFound.into());
        };

        let icon_file_offset: usize = icon_file.file_offset().try_into().unwrap();
        let icon_offset: usize = offset + EXEFS_HEADER_SIZE + icon_file_offset;
        let smdh_icon = SMDHIcon::from_smdh(input, Some(icon_offset))?;
        Ok(smdh_icon)
    }
}
