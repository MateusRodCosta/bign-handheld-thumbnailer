mod cci;
mod cia;
mod cxi;

use image::{ImageBuffer, Rgba, RgbaImage};
use std::io::{Read, Seek, SeekFrom};

use crate::n3ds::errors::N3DSParsingError;
use crate::utils::rgb888::{Rgb565, Rgb888};

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
    pub large_icon: ImageBuffer<Rgba<u8>, Vec<u8>>,
}

impl SMDHIcon {
    fn generate_icon_from_bytes(large_icon_bytes: &[u8; 0x1200]) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
        /*
         * The large 3DS icon is 48x48 px and divided in tiles according to Morton order
         * Each color will usually be RGB565 although it's not the only supported color enconding
         */

        const LARGE_ICON_SIZE: u32 = 48;
        let mut img = RgbaImage::new(LARGE_ICON_SIZE, LARGE_ICON_SIZE);

        /*
         * Due to the Morton order, the code for the coordinates of the pixels is oxided from
         * https://github.com/ihaveamac/pyctr/blob/master/pyctr/type/smdh.py
         * Many thanks to ihaveamac from the Nintendo Homebrew Discord for the help
         */

        for y in 0..LARGE_ICON_SIZE as usize {
            for x in 0..LARGE_ICON_SIZE as usize {
                let morton_index = (((y >> 3) * ((LARGE_ICON_SIZE as usize) >> 3) + (x >> 3)) << 6)
                    + ((x & 1)
                        | ((y & 1) << 1)
                        | ((x & 2) << 1)
                        | ((y & 2) << 2)
                        | ((x & 4) << 2)
                        | ((y & 4) << 3));

                let byte_offset = morton_index * 2;
                let raw_pixel = [large_icon_bytes[byte_offset], large_icon_bytes[byte_offset + 1]];
                let pixel = Rgb888::from(Rgb565::from(raw_pixel));

                img.put_pixel(x as u32, y as u32, Rgba([pixel.r, pixel.g, pixel.b, 0xFF]));
            }
        }

        img
    }
}

impl SMDHIcon {
    pub fn from_smdh<T: Read + Seek>(f: &mut T) -> Result<Self, N3DSParsingError> {
        const SMDH_LARGE_ICON_OFFSET: u64 = 0x24C0;
        const SMDH_LARGE_ICON_SIZE: usize = 0x1200;

        let smdh_start_pos = f.stream_position()?;

        let mut smdh_magic = [0u8; 4];
        f.read_exact(&mut smdh_magic)?;
        if b"SMDH" != &smdh_magic {
            return Err(N3DSParsingError::FileMagicNotFound("SMDH", smdh_magic));
        }

        f.seek(SeekFrom::Start(smdh_start_pos + SMDH_LARGE_ICON_OFFSET))?;
        let mut large_icon_bytes = [0u8; SMDH_LARGE_ICON_SIZE];
        f.read_exact(&mut large_icon_bytes)?;
        Ok(Self {
            large_icon: Self::generate_icon_from_bytes(&large_icon_bytes),
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
        Self::from_smdh(f)
    }
}
