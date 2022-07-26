mod helpers;

use core::{num, panic};
use std::fs;
use std::error::Error;
use helpers::dump_hex;

#[derive(Debug)]
pub struct KfnFile {
    file: Vec<u8>,
    read_head: usize,
}

impl KfnFile {
    pub fn read(filename: &str) -> Self {
        Self { 
            file: match fs::read(filename) {
                Ok(file) => file,
                Err(e) => panic!("File not found! {}", e),
            },
            read_head: 0
         }
    }

    pub fn parse(&mut self) -> Result<bool, Box<dyn Error>> {
        // read file signature
        let signature = String::from_utf8(self.read_bytes(4))?;
        // if file signature is not KFNB, end parsing
        if signature != "KFNB" {
            return Ok(false);
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
                    println!("{}, type 2, length {}, hex: {}", signature, len_or_value, dump_hex(buf));
                },
                _ => {

                },
            }

            if signature == "ENDH" {
                break;
            }
        }
        println!("Header ends at offset {}", self.read_head);

        // reading the directory
        let num_files = self.read_dword();
        println!("Number of files: {}", num_files);

        for i in 0..num_files {
            let filename_len = self.read_dword();
            let filename = match String::from_utf8(self.read_bytes(filename_len)) {
                Ok(s) => s,
                Err(_) => panic!("Invalid filename in KFN file.")
            };
            let file_type = self.read_dword();
            let file_len1 = self.read_dword();
            let file_offset = self.read_dword();
            let file_len2 = self.read_dword();
            let file_flags = self.read_dword();

            println!("File {}, type: {}, len1: {}, len2: {}, offset: {}, flags: {}",
                                                                                filename,
                                                                                file_type,
                                                                                file_len1,
                                                                                file_len2,
                                                                                file_offset,
                                                                                file_flags);
        }
        println!("Directory ends at offset {}", self.read_head);

        Ok(true)
    }

    /// Helper IO function for reading a byte
    fn read_byte(&mut self) -> u8 {
        let result = self.file[self.read_head];
        self.read_head += 1;
        (result & 0xFF).into()
    }

    /// Helper IO function for reading a word
    fn read_word(&mut self) -> u8 {
        let b1 = self.read_byte();
        let b2 = self.read_byte();

        b2 << 8 | b1
    }

    /// Helper IO function for reading a dword
    fn read_dword(&mut self) -> u8 {
        let b1 = self.read_byte() as u32;
        let b2 = self.read_byte() as u32;
        let b3 = self.read_byte() as u32;
        let b4 = self.read_byte() as u32;

        (b4 << 24 | b3 << 16 | b2 << 8 | b1) as u8
    }

    /// Helper IO function for reading a specified amount fo bytes
    fn read_bytes(&mut self, length: u8) -> Vec<u8> {
        let mut array: Vec<u8> = Vec::with_capacity(length as usize);
        for n in 0..length {
            array.push(self.read_byte());
        }
        array
    }

    
}


