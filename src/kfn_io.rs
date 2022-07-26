pub mod helpers;

use core::{panic};
use std::fs;
use std::error::Error;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use helpers::dump_hex;
use helpers::FileType;
use helpers::Entry;

use crate::kfn_io::helpers::Header;

#[derive(Debug)]
/// Struct representing a KFN file and it's components.
pub struct KfnFile {
    header: Header,
    file: Vec<u8>,
    read_head: usize,
    entries: Vec<Entry>,
}

impl KfnFile {

    /// Constructor for creating a KfnFile struct.
    /// Takes the filename as parameter.
    pub fn new(filename: &str) -> Self {
        let entries = Vec::new();
        let header = Header::new();
        Self { 
            file: match fs::read(filename) {
                Ok(file) => file,
                Err(e) => panic!("File not found! {}", e),
            },
            read_head: 0,
            entries,
            header,
         }
    }

    /// Method for parsing the file itself.
    pub fn parse(&mut self) -> Result<bool, Box<dyn Error>> {
        // read file signature
        let signature = String::from_utf8(self.read_bytes(4))?;
        // if file signature is not KFNB, end parsing
        if signature != "KFNB" {
            panic!("Bad signature error");
        }

        // reading the header
        loop {
            let signature = String::from_utf8(self.read_bytes(4))?;
            let l_type = self.read_byte();
            let len_or_value = self.read_dword();

            match l_type {
                1 => {
                    println!("{}, type 1, value {:x}", signature, len_or_value);
                },
                2 => {
                    let buf = self.read_bytes(len_or_value);
                    dbg!(&signature);
                    match signature.as_str() {
                        "TITL" => self.header.title = String::from_utf8(buf.clone()).unwrap_or("Unknown".to_string()),
                        "ARTS" => self.header.artist = String::from_utf8(buf.clone()).unwrap_or("Unknown".to_string()),
                        "KFNZ" => self.header.karafunizer = String::from_utf8(buf.clone()).unwrap_or("Unknown".to_string()),
                        _ => ()
                    }
                    println!("{}, type 2, length {:#?}, hex: {}, string: {:?}", signature, len_or_value, dump_hex(&buf), String::from_utf8(buf));
                },
                _ => {

                },
            }

            if signature == "ENDH" {
                break;
            }
        }
        println!("header end: {}", self.read_head);

        // reading the directory
        let num_files = self.read_dword();
        println!("# of files: {}", num_files);

        for _ in 0..num_files {

            let filename_len = self.read_dword();
            let filename = match String::from_utf8(self.read_bytes(filename_len)) {
                Ok(s) => s,
                Err(_) => panic!("Invalid filename in KFN file.")
            };
            let file_type = FileType::from(self.read_dword());
            let len1 = self.read_dword() as usize;
            let offset = self.read_dword() as usize;
            let len2 = self.read_dword() as usize;
            let flags = self.read_dword() as usize;

            self.entries.push(Entry {
                filename, file_type, len1, offset, len2, flags
            });
        }

        // readjust offset
        for i in 0..self.entries.len() {
            self.entries[i].offset += self.read_head;
        }

        println!("Directory ends at offset {}", self.read_head);

        Ok(true)
    }

    /// Extracting all files.
    pub fn extract_all(&mut self) {
        for i in 0..self.entries.len() {
            self.extract(self.entries[i].clone(), self.entries[i].clone().filename);
        }
    }

    /// Extracting a single file from the entry to a deisgnated output.
    fn extract(&mut self, entry: Entry, output_filename: String) {
        // move read head to the beginning of the file
        let mut path_str = self.header.title.clone();
        path_str.push('/');
        path_str.push_str(output_filename.as_str());
        let path = Path::new(&path_str);
        let prefix = path.parent().unwrap();
        fs::create_dir_all(prefix).unwrap();
        // create output file
        dbg!(&self.header.title);
        let mut output = File::create(path).unwrap();
        // init buffer
        let buf: Vec<u8> = Vec::from(&self.file[entry.offset..entry.offset+entry.len1]);
        output.write_all(&buf).unwrap();
        
    }

    /// Helper IO function for reading a byte
    fn read_byte(&mut self) -> u8 {
        let result = self.file[self.read_head as usize];
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


