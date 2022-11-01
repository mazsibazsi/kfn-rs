
pub mod file_type;
pub mod event;

use std::{fmt::Write};
use file_type::FileType;

/// Converting a Vector of u8s HEX to String HEX
pub fn dump_hex(array: &Vec<u8>) -> String {
    let mut output = String::new();
    for i in 0..array.len() {
        if i > 0 {
            output.push(' ');
        }
        let mut tmp = String::new();
        write!(&mut tmp, "{:X} ", array[i] & 0xFF).unwrap();
        output.push_str(&tmp);
    }
    output
}

/// Helper function to slice up an u32 to a vector of u8s.
pub fn u32_to_u8_arr(x:u32) -> Vec<u8> {
    let b1 : u8 = ((x >> 24) & 0xff) as u8;
    let b2 : u8 = ((x >> 16) & 0xff) as u8;
    let b3 : u8 = ((x >> 8) & 0xff) as u8;
    let b4 : u8 = (x & 0xff) as u8;
    return vec![b4, b3, b2, b1]
}

/// Representing a file entry in the KFN file.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Entry {
    pub file_type: FileType,
    pub filename: String,
    pub len1: usize,
    pub offset: usize,
    pub len2: usize,
    pub flags: usize,
    pub file_bin: Vec<u8>,
}


