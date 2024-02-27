# bign-handheld-thumbnailer

A thumbnailer for Nintendo handheld systems (Nintendo DS and 3DS) roms and files.

This project adheres to the [Freedesktop Thumbnail Managing Standard](https://specifications.freedesktop.org/thumbnail-spec/thumbnail-spec-latest.html).

## Supported files and limitations

* Nintendo DS:
  * .nds roms (DSi animated icon not supported)
* Nintendo 3DS:
  * .cia files (only if Meta section is present and contains a SMDH with a valid icon)
  * .smdh metadata files
  * other file types might be supported in the future

## How to install

There are three pieces to get the thumbnailer running: a compiled `bign-handheld-thumbnailer` binary, the .thumbnailer file and the definition of the .cia mime type (needed for .cia thumbnails).

Due to nautilus using a sandbox to run thumbnailers it's needed to copy it from a place where the sandbox can find it, such as `/usr/bin`.

1. Compile the project with `cargo build --release`
2. Copy the compiled binary to /usr/bin (e.g. `sudo cp target/release/bign-handheld-thumbnailer /usr/bin/`)
3. Copy `bign-handheld-thumbnailer-3ds.xml` to `/usr/share/mime/packages/` to install the .cia mime type definition system-wide
4. Run `sudo update-mime-database /usr/share/mime` so the system-wide mime type database is updated based on the newly-added configuration
5. Copy the `bign-handheld-thumbnailer.thumbnailer` file to `~/.local/share/thumbnailers` (user-install) or `/usr/share/thumbnailers` (system-install)
6. At this point thumbnails should be working
