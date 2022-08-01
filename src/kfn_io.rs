/// Helpers, containing methods for handling the data and filetypes.
pub mod helpers;
/// Module for handling the main data of the KFN file, like songtexts and sync times.
pub mod kfn_data;
/// The header of the KFN file, containing non-essential data for playing, like the artist or title.
pub mod kfn_header;

use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::string::FromUtf8Error;

use helpers::dump_hex;
use helpers::FileType;
use helpers::Entry;


use regex::Regex;

use kfn_data::KfnData;

use kfn_header::KfnHeader;

use dbg_hex::dbg_hex;



#[derive(Debug)]
/// Struct representing a KFN file and it's components.
pub struct KfnFile {
    
    /// The binary data in a vector of bytes.
    file_data: Vec<u8>,

    /// The read head, used in calculating the offset from the directory end.
    read_head: usize,
    
    /// The header data of the file.
    pub header: KfnHeader,
    /// The data container for the file.
    pub data: KfnData,
}



impl KfnFile {

    /// Constructor for creating a KfnFile struct.
    /// Takes the filename as parameter.
    pub fn read(filename: &str) -> Self {
        Self { 
            file_data: match fs::read(filename) {
                Ok(file) => file,
                Err(e) => panic!("File not found! {}", e),
            },
            read_head: usize::default(),
            header: KfnHeader::default(),
            data: KfnData::new(),
         }
    }

    /// Method for parsing the file itself.
    pub fn dump(&mut self) -> Result<bool, FromUtf8Error> {
        // read file signature
        let signature = String::from_utf8(self.read_bytes(4))?;
        // if file signature is not KFNB, end parsing
        if signature != "KFNB" {
            panic!("Bad signature error");
        }
        
        // reading the header
        loop {
            // get signature
            let signature = String::from_utf8(self.read_bytes(4))?;
            // get type of the line
            let l_type = self.read_byte();
            let len_or_value = self.read_dword();

            // match for line type > if type 1, it's a value, if type 2 -> it contains header information
            match l_type {
                1 => {
                    match signature.as_str() {
                        "DIFM" => {
                            self.header.diff_men = len_or_value;
                        },
                        "DIFW" => {
                            self.header.diff_women = len_or_value;
                        },
                        "GNRE" => {
                            self.header.genre = len_or_value;
                        },
                        "SFTV" => {
                            self.header.sftv = len_or_value;
                        },
                        "MUSL" => {
                            self.header.musl = len_or_value;
                        },
                        "ANME" => {
                            self.header.anme = len_or_value;
                        },
                        "TYPE" => {
                            self.header.kfn_type = len_or_value;
                        },
                        "RGHT" => {
                            self.header.rght = len_or_value;
                        },
                        "PROV" => {
                            self.header.prov = len_or_value;
                        },
                        _ => println!("{}, type 1, value {:x}", signature, len_or_value),
                    }
                    //println!("{}, type 1, value {:x}", signature, len_or_value);
                },
                2 => {
                    // get data into buffer
                    let buffer = self.read_bytes(len_or_value);
                    let buffer_str = String::from_utf8(buffer.clone()).unwrap_or("Unknown".to_string());
                    // match header info and insert into header
                    match signature.as_str() {
                        "FLID" => {
                            self.header.flid = buffer_str;
                        },
                        "LANG" => {
                            self.header.language = buffer_str
                        },
                        "TITL" => {
                            self.header.title = buffer_str
                        },
                        "ALBM" => {
                            self.header.album = buffer_str
                        },
                        "ARTS" => {
                            self.header.artist = buffer_str
                        },
                        "COMP" => {
                            self.header.composer = buffer_str
                        },
                        "COPY" => {
                            self.header.copyright = buffer_str
                        },
                        "SORC" => {
                            self.header.source_file = buffer_str
                        },
                        "YEAR" => {
                            self.header.year = buffer_str
                        },
                        "TRAK" => {
                            self.header.trak = buffer_str
                        },
                        "KFNZ" => {
                            self.header.karafunizer = buffer_str
                        },
                        "IDUS" => {
                            self.header.idus = buffer_str
                        },
                        _ => ()
                        // TODO implement all header types
                    }
                    println!("{}, type 2, length {:#?}, hex: {}, string: {:?}", signature, len_or_value, dump_hex(&buffer), String::from_utf8(buffer));
                },
                _ => {

                },
            }

            if signature == "ENDH" {
                break;
            }
        }
        println!("header end: {}", self.read_head);
        
        dbg_hex!(&self.header);
        // reading the directory
        let num_files = self.read_dword();
        println!("# of files: {}", num_files);

        for _ in 0..num_files {

            let filename_len = self.read_dword();
            let filename = match String::from_utf8(self.read_bytes(filename_len)) {
                Ok(s) => s,
                Err(e) => return Err(e),
            };
            let file_type = FileType::from(self.read_dword());
            let len1 = self.read_dword() as usize;
            let offset = self.read_dword() as usize;
            let len2 = self.read_dword() as usize;
            let flags = self.read_dword() as usize;

            let buf: Vec<u8> = Vec::default();

            self.data.entries.push(Entry {
                filename, file_type, len1, offset, len2, flags, file_bin: buf,
            });
        }

        self.data.offset_dir_end = self.read_head;
        // readjust offset
        for i in 0..self.data.entries.len() {
            self.data.entries[i].offset += self.read_head;
            self.data.entries[i].file_bin = 
                        Vec::from(
                            &self.file_data[
                                self.data.entries[i].offset
                                ..
                                self.data.entries[i].offset +   self.data.entries[i].len1
                            ]
                        );
        }
        
        println!("Directory ends at offset {}", self.read_head);


        Ok(true)
    }

    /// Extracting songtexts into a vector.
    pub fn get_syncs(&mut self) -> Vec<usize> {

        let mut syncs: Vec<usize> = Vec::new();

        let contents_raw = fs::read_to_string(&self.data.path_song_ini).unwrap();
        let contents: Vec<&str> = contents_raw.split("\n").collect();
        for line in contents {
            let re = Regex::new(r"^Sync\d+=(.*)$").unwrap();
            
            if re.is_match(line) {
                let syncline_str = re.captures(line).unwrap().get(1).map_or("", |m| m.as_str());
                let mut syncline_split: Vec<usize> = syncline_str.split(",").map(|n| usize::from_str_radix(n, 10).unwrap()).collect();
                syncs.append(&mut syncline_split);
            }
        }
        syncs
    }

    /// Extracting sync points into a vector.
    pub fn get_text(&mut self) -> Vec<String> {

        let mut text: Vec<String> = Vec::new();

        let contents_raw = fs::read_to_string(&self.data.path_song_ini).unwrap();
        let contents: Vec<&str> = contents_raw.split("\n").collect();
        for line in contents {
            let re = Regex::new(r"^Text\d+=(.*)$").unwrap();
            if re.is_match(line) {
                let textline_str = re.captures(line).unwrap().get(1).map_or("", |m| m.as_str());
                let mut textline_split: Vec<String> = textline_str.split(&['/', ' ', '\n']).map(|s| s.to_string()).collect();
                if textline_str != "" {
                    text.append(&mut textline_split);
                }
            }
        }
        text
    }

    /// Extracting all files.
    pub fn extract_all(&mut self) {
        for i in 0..self.data.entries.len() {
            self.extract(self.data.entries[i].clone(), self.data.entries[i].clone().filename);
        }
    }

    /// Extracting a single file from the entry to a deisgnated output.
    fn extract(&mut self, entry: Entry, output_filename: String) {
        let mut path_str = self.header.title.clone();
        path_str.push('/');
        path_str.push_str(output_filename.as_str());
        self.data.path_song_ini = path_str.clone();
        let path = Path::new(&path_str);
        let prefix = path.parent().unwrap();
        fs::create_dir_all(prefix).unwrap();
        // create output file
        let mut output = File::create(path).unwrap();
        // init buffer
        let buf: Vec<u8> = Vec::from(&self.file_data[entry.offset..entry.offset+entry.len1]);
        output.write_all(&buf).unwrap();
        
    }

    /// Helper IO function for reading a byte
    fn read_byte(&mut self) -> u8 {
        let result = self.file_data[self.read_head as usize];
        self.read_head += 1;
        (result & 0xFF).into()
    }

    /// Helper IO function for reading a word
    fn _read_word(&mut self) -> u16 {
        let b1 = self.read_byte() as u16;
        let b2 = self.read_byte() as u16;

        b2 << 8 | b1
    }

    /// Helper IO function for reading a dword
    fn read_dword(&mut self) -> u32 {
        let b1 = self.read_byte() as u32;
        let b2 = self.read_byte() as u32;
        let b3 = self.read_byte() as u32;
        let b4 = self.read_byte() as u32;

        b4 << 24 | b3 << 16 | b2 << 8 | b1
    }

    /// Helper IO function for reading a specified amount fo bytes
    fn read_bytes(&mut self, length: u32) -> Vec<u8> {
        let mut array: Vec<u8> = Vec::with_capacity(length as usize);
        for _ in 0..length {
            array.push(self.read_byte());
        }
        array
    }

    
}
