pub mod n3ds_parsing_errors;
pub mod n3ds_structures;

/*
 * Currently .cia, .smhd and .3dsx files are supported.
 *
 * Consider the following links for more info about the CIA, SMDH and 3DSX structure:
 *
 * On GBATEK:
 * CIA: https://problemkaputt.de/gbatek.htm#3dsfilestitleinstallationarchivecia
 * SMDH: https://problemkaputt.de/gbatek.htm#3dsfilesvideoiconssmdh
 * 3DSx: https://problemkaputt.de/gbatek.htm#3dsfilestitlehomebrewexecutables3dsx
 *
 * On 3dbrew:
 * CIA: https://www.3dbrew.org/wiki/CIA
 * SMDH: https://www.3dbrew.org/wiki/SMDH
 * 3DSX: https://www.3dbrew.org/wiki/3DSX_Format
 *
 * Do note that the Meta section conatining a SMHD might or might not be present on .cia files.
 * Do also note that the extended header with a SMHD is ptional for .3dsx
*/
