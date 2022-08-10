

use std::fs;
use std::path::Path;

use derivative::Derivative;
use ini::Ini;

use super::helpers::{Entry, u32_to_u8_arr};
use super::helpers::file_type::{FileType, ToBinary};

use super::kfn_ini::KfnIni;


/// KfnHeader depicting the header contents of a KFN file
#[derive(Derivative)]
#[derivative(Debug)]
pub struct KfnData {
    /// The location of the Songs.ini file.
    pub path_song_ini: String,
    /// Files in the directory/library
    pub entries: Vec<Entry>,
    /// End of the directory header
    pub offset_dir_end: usize,
    /// Representation of the last file of the directory, the Song.ini.
    #[derivative(Debug="ignore")]
    pub song: KfnIni,
}

impl KfnData {
    /// Creating a new KfnData with default values.
    pub fn new() -> Self {

        let dir_songs_ini = String::new();
        let entries = Vec::new();
        let offset_dir_end = 0;
        let mut kfn_ini = KfnIni::new();

        kfn_ini.populate();
        
        Self {
            path_song_ini: dir_songs_ini, entries, offset_dir_end, song: kfn_ini
        }
    }

    /// Get the Songs.ini file from the entries.
    pub fn get_songs_ini(&self) -> Option<Entry> {

        let mut song_ini = None;
        
        for entry in self.entries.clone() {

            if entry.filename == "Song.ini" {
                
                song_ini = Some(entry);
                break;
            }
        }
        
        song_ini
    }

    /// Reads the INI file into the struct.
    pub fn read_ini(&mut self) {

        self.song.ini = Ini::load_from_str(String::from_utf8(self.get_songs_ini().unwrap().file_bin).unwrap().as_str()).unwrap();
    
    }

    /// Updates the ini file. Removes the Song.ini entry, then recreates the INI file from the struct.
    pub fn update_ini(&mut self) {

        // remove the entry
        self.remove_entry_by_name("Song.ini");


        // updating the ini
        self.song.set_materials(self.entries.clone());

        // creating a destination vector for the data
        let mut writer = Vec::new();

        // write the data into the vector
        self.song.ini.write_to(&mut writer).unwrap();
        let data = writer.to_owned();

        //  create a new entry
        let new_entry = Entry {
            file_type: FileType::SongIni,
            filename: "Song.ini".to_string(),
            len1: data.len(),
            offset: 0,
            len2: data.len(),
            flags: usize::default(),
            file_bin: data,
        };
        // and add the entry
        self.add_entry(new_entry);
        

    }

    /// Used internally before writing to binary, to readjust the offsets that became misaligned.
    fn adjust_dir_offset(&mut self) {

        self.entries[0].offset = 0;
        
        for i in 1..self.entries.len() {
        
            self.entries[i].offset = self.entries[i-1].offset + self.entries[i-1].len1;
            
        }

    }

    /// Adds an entry to the directory
    pub fn add_entry(&mut self, new_entry: Entry) {
        /* let last_entry = self.entries[self.entries.len()].clone();
        let new_offset: usize = last_entry.offset + new_entry.len1;
        new_entry.offset = new_offset; */
        self.entries.push(new_entry);
    
    }

    /// Adding a new entry from the data.
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

        let filename = Path::new(filename);
        let filename = filename.file_name().unwrap().to_str().unwrap();

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

        // update the ini, so that it contains the new file as well
        self.update_ini();

    }

    // Removing an entry from the data.
    pub fn remove_entry_by_id(&mut self, id: usize) {
        
        // Extract the entry and save it
        // to have it's length later.
        let removed_entry = self.entries.remove(id);
        // iterate over the entries...
        for i in id+1..self.entries.len()-1 {
            // ...and remove the removed entry's length from their offset.
            self.entries[i].offset -= removed_entry.len1;
        }
    }

    /// Removing an entry by name from the data. If it doesn't exist, it wont delete.
    pub fn remove_entry_by_name(&mut self, name: &str) {
        
        let mut id: isize = -1;
        
        for i in 0..self.entries.len() {

            if self.entries[i].filename == name {
                
                id = i as isize;
            }
        }
        if id == -1 {
            return;
        }

        // Extract the entry and save it to have it's length later.
        let removed_entry = self.entries.remove(id as usize);

        // iterate over the entries...
        for i in id as usize+1..self.entries.len()-1 {
            // ...and remove the removed entry's length from their offset.
            self.entries[i as usize].offset -= removed_entry.len1;
        }
    
    }

    /// Gets the next available offset for the new entry.
    pub fn get_next_offset(&self) -> usize {
        
        if self.entries.len() == 0 {
            return 0;
        }
        
        // get the id of the last entry
        let last_index = self.entries.len()-1;

        // return the last entry's offset plus its length, removed the end of the dir header to get the new offset
        self.entries[last_index].offset + self.entries[last_index].len1 - self.offset_dir_end
    
    }

}

impl ToBinary for KfnData {
    fn to_binary(&mut self) -> Vec<u8> {

        self.adjust_dir_offset();
        
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

        //data.append(&mut self.get_songs_ini().unwrap().file_bin);

        data
    }
}