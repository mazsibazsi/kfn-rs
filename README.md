# kfn-rs

## kfn-rs project
### Karaoke .kfn file IO library written in Rust.
### The goal of the project, is to have a working KFN I/O library, which can be use to build a player and editor for these kind of karaoke file formats.

## kfn-io library
### Based on Ulduzsoft's KFN Dumper.

- https://www.ulduzsoft.com/2012/10/reverse-engineering-the-karafun-file-format-part-1-the-header/
- https://www.ulduzsoft.com/2012/10/reverse-engineering-the-karafun-file-format-part-2-the-directory/
- https://www.ulduzsoft.com/2012/10/reverse-engineering-the-karafun-file-format-part-3-the-song-ini-file/
- https://www.ulduzsoft.com/2012/10/reverse-engineering-the-karafun-file-format-part-4-the-encryption/

### Features
- [x] Reading header
- [x] Modifying header
- [X] Replicating header
- [x] Extracting files
- [x] Repackaging files
- [x] Extracting .ini (songtext, animations, sync timestamps)
- [x] Modifying .ini (songtext, animations, sync timestamps)
- [x] Repackaging .ini (songtext, animations, sync timestamps)