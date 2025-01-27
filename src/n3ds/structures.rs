mod cci;
mod cia;
mod cxi;

use image::{ImageBuffer, Rgba, RgbaImage};
use std::io::{Read, Seek, SeekFrom};

use crate::n3ds::errors::N3DSParsingError;
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
 * Do note that the Meta section containing a SMHD might or might not be present on .cia files.
 * If the Meta section isn't present in a CIA, the tool tries to extract the icon from the CXI
 * located inside the CIA.
 *
 * Do also note that the extended header with a SMHD is optional for the .3dsx file format.
*/

#[derive(Debug)]
pub struct SMDHIcon {
    large_icon: ImageBuffer<Rgba<u8>, Vec<u8>>,
}

impl SMDHIcon {
    pub fn get_large_icon(&self) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
        self.large_icon.clone()
    }

    fn generate_icon_from_bytes(large_icon_bytes: &[u8; 0x1200]) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
        /*
         * The large 3DS icon is 48x48 px and divided in tiles according to Morton order
         * Each color will usually be RGB565 although it's not the only supported color enconding
         */

        const LARGE_ICON_SIZE: usize = 48;
        const LARGE_ICON_WIDTH: usize = LARGE_ICON_SIZE;
        const LARGE_ICON_HEIGHT: usize = LARGE_ICON_SIZE;

        let large_icon_data: [Rgb888; 0x1200 / 2] = large_icon_bytes
            .chunks_exact(2)
            .map(|chunk| Rgb888::from_rgb565_bytes(chunk.try_into().unwrap()))
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();

        /*
         * Due to the Morton order, the code for the coordinates of the pixels is oxided from
         * https://github.com/ihaveamac/pyctr/blob/master/pyctr/type/smdh.py
         * Many thanks to ihaveamac from the Nintendo Homebrew Discord for the help
         */

        let mut img = RgbaImage::new(LARGE_ICON_WIDTH as u32, LARGE_ICON_HEIGHT as u32);

        for y in 0..LARGE_ICON_HEIGHT {
            for x in 0..LARGE_ICON_WIDTH {
                let pixel_offset = (((y >> 3) * (LARGE_ICON_WIDTH >> 3) + (x >> 3)) << 6)
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
    pub fn from_smdh<T: Read + Seek>(f: &mut T) -> Result<Self, N3DSParsingError> {
        const SMDH_LARGE_ICON_OFFSET: i64 = 0x24C0;
        const SMDH_LARGE_ICON_SIZE: usize = 0x1200;

        let mut smdh_magic = [0u8; 4];
        f.read_exact(&mut smdh_magic)?;
        if b"SMDH" != &smdh_magic {
            return Err(N3DSParsingError::FileMagicNotFound("SMDH", smdh_magic));
        }

        f.seek_relative(SMDH_LARGE_ICON_OFFSET - 4)?;
        let mut large_icon_bytes = [0u8; SMDH_LARGE_ICON_SIZE];
        f.read_exact(&mut large_icon_bytes)?;
        Ok(SMDHIcon {
            large_icon: SMDHIcon::generate_icon_from_bytes(&large_icon_bytes),
        })
    }

    pub fn from_n3dsx<T: Read + Seek>(f: &mut T) -> Result<Self, N3DSParsingError> {
        const N3DSX_EXTENDED_HEADER_OFFSET: u64 = 0x20;

        let mut n3dsx_magic = [0u8; 4];
        f.read_exact(&mut n3dsx_magic)?;
        if b"3DSX" != &n3dsx_magic {
            return Err(N3DSParsingError::FileMagicNotFound("3DSX", n3dsx_magic));
        }

        let mut header_size = [0u8; 2];
        f.read_exact(&mut header_size)?;
        let header_size = u16::from_le_bytes(header_size);
        if header_size <= 32 {
            return Err(N3DSParsingError::N3DSXParsingError3DSXNoExtendedHeader(
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
}
