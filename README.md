# bign-handheld-thumbnailer

A thumbnailer for Nintendo handheld systems (Nintendo DS and 3DS) roms and files.

This project adheres to the [Freedesktop Thumbnail Managing Standard](https://specifications.freedesktop.org/thumbnail-spec/thumbnail-spec-latest.html).

## Supported files and limitations

* Nintendo DS:
  * NDS roms (.nds extension) - note that DSi animated icons are not supported, the standard DS icon is used instead
* Nintendo 3DS:
  * CIA installer files (.cia extension) - only if Meta section is present and contains a valid SMDH with a valid large icon
  * SMDH metadata files (.smdh extension) - used to be shipped as a separate file for older homebrew but is usually contained inside the more common 3DS file formats and also on newer homebrew
  * 3DSX homebrew files (.3dsx extension) - only if extended header is present and contains a valid SMDH with valid large icon
  * CXI executable files (.cxi extension) - as long as the file is decrypted and it's possible to extract the icon file from the ExeFS
  * CCI cartridge dumps files (the usual roms, .cci extension but more commonly .3ds) - as long it's possible to access the CXI (usually on partition 0) and extract the icon from there (see above, requires a decrypted rom)

## How to install

There are three pieces to get the thumbnailer running: a compiled `bign-handheld-thumbnailer` binary, the `bign-handheld-thumbnailer.thumbnailer` file and the needed mime type definitions from `bign-handheld-thumbnailer-3ds.xml`.

Due to nautilus using a sandbox to run thumbnailers it's needed to install the binary in a place where the sandbox can access it, such as `/usr/bin`.

1. Compile the project with `cargo build --release`
2. Copy the compiled binary to /usr/bin (e.g. `sudo cp target/release/bign-handheld-thumbnailer /usr/bin/`)
3. Copy `bign-handheld-thumbnailer-3ds.xml` to `/usr/share/mime/packages/` to install the needed mime type definition system-wide
4. Run `sudo update-mime-database /usr/share/mime` so the system-wide mime type database is updated based on the newly-added configuration
5. Copy the `bign-handheld-thumbnailer.thumbnailer` file to `~/.local/share/thumbnailers` (user-install) or `/usr/share/thumbnailers` (system-install)
6. At this point thumbnails should be working, you likely will want to restart the file explorer or clear the cached thumbnails
