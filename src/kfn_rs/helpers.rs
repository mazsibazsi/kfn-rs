use std::{fmt::Write};

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

/// File types, that indicate what kind of files can occur in a KFN file.
#[derive(Debug, Copy, Clone)]
pub enum FileType {
    SongIni,
    Music,
    Image,
    Font,
    Video,
    INVALID,
}

pub trait ToBinary {
    fn to_binary(&mut self) -> Vec<u8>;
}

/// Helper to read the file type in the directory.
impl From<u32> for FileType {
    fn from(numeric: u32) -> Self {
        match numeric {
            1 => FileType::SongIni,
            2 => FileType::Music,
            3 => FileType::Image,
            4 => FileType::Font,
            5 => FileType::Video,
            _ => FileType::INVALID,
        }
    }
}

impl Into<u32> for FileType {
    fn into(self) -> u32 {
        match self {
            FileType::SongIni => 1,
            FileType::Music => 2,
            FileType::Image => 3,
            FileType::Font => 4,
            FileType::Video => 5,
            FileType::INVALID => 0,
        }
    }
}

impl Default for FileType {
    fn default() -> Self {
        FileType::INVALID
    }
}

/// Representing a file entry in the KFN file.
#[derive(Debug, Clone, Default)]
pub struct Entry {
    pub file_type: FileType,
    pub filename: String,
    pub len1: usize,
    pub offset: usize,
    pub len2: usize,
    pub flags: usize,
    pub file_bin: Vec<u8>,
}

