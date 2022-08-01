use std::fs;

use super::helpers::{Entry, FileType};
use super::helpers::{u32_to_u8_arr, ToBinary};

/// KfnHeader depicting the header contents of a KFN file
#[derive(Debug)]
pub struct KfnData {
    /// The location of the Songs.ini file.
    pub path_songs_ini: String,
    /// Sync timestamps
    pub syncs: Vec<usize>,
    /// Lyrics
    pub text: Vec<String>,
    /// Files in the directory/library
    pub entries: Vec<Entry>,
    /// End of the directory header
    pub dir_end: usize,
}

impl KfnData {
    pub fn new() -> Self {
        let dir_songs_ini = String::new();
        let entries = Vec::new();
        let syncs = Vec::new();
        let text = Vec::new();
        let dir_end = 0;
        Self {
            path_songs_ini: dir_songs_ini, syncs, text, entries, dir_end,
        }
    }


    /// Adds an entry to the directory
    pub fn add_entry(&mut self, new_entry: Entry) {
        /* let last_entry = self.entries[self.entries.len()].clone();
        let new_offset: usize = last_entry.offset + new_entry.len1;
        new_entry.offset = new_offset; */
        self.entries.push(new_entry);
    }

    /// Adding a new entry from file.
    pub fn add_entry_from_file(&mut self, filename: &str) {

        // reading the file from the file system
        let new_file = fs::read(filename).unwrap();
        
        // splitting it at the point to get the extension
        let parts : Vec<&str> = filename.split('.').collect();

        // match the extension to the appropriate file type
        let extension = match parts.last() {
            Some(v) =>
                match *v {
                    "png" => FileType::Image,
                    "jpg" => FileType::Image,
                    "mp3" => FileType::Music,
                    "wav" => FileType::Music,
                    "ttf" => FileType::Font,
                    "otf" => FileType::Font,
                    &_ => FileType::INVALID,
                },
            None => FileType::INVALID,
        };

        // create an entry
        let new_entry = Entry {
            file_type: extension,
            filename: filename.to_string(),
            len1: new_file.len(),
            offset: self.get_next_offset(),
            len2: new_file.len(),
            flags: 0,
            file_bin: new_file,
        };

        // add the entry to the library
        self.add_entry(new_entry);

    }

    /// Gets the next available offset for the new entry.
    pub fn get_next_offset(&self) -> usize {
        
        // get the id of the last entry
        let last_index = self.entries.len()-1;

        // return the last entry's offset plus its length, removed the end of the dir header to get the new offset
        self.entries[last_index].offset + self.entries[last_index].len1 - self.dir_end
    }

}

impl ToBinary for KfnData {
    fn to_binary(&self) -> Vec<u8> {
        let mut data: Vec<u8> = Vec::new();
        data.append(&mut u32_to_u8_arr(self.entries.len() as u32));
        for entry in &self.entries {
            // append the filename length
            data.append(&mut u32_to_u8_arr(entry.filename.len() as u32));
            // a. filename
            data.append(&mut entry.filename.as_bytes().to_owned());
            // a. file type
            data.append(&mut u32_to_u8_arr(entry.file_type.into()));
            // a. length 1
            data.append(&mut u32_to_u8_arr(entry.len1 as u32));
            // a. offset
            data.append(&mut u32_to_u8_arr(entry.offset as u32));
            // a. length 2
            data.append(&mut u32_to_u8_arr(entry.len2 as u32));
            // a. flags
            data.append(&mut u32_to_u8_arr(entry.flags as u32));
        }

        // append the file data
        for entry in &self.entries {
            data.append(&mut entry.file_bin.to_owned());
        }

        data
    }
}