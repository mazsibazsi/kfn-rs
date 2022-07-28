use std::fmt::Write;

/// Converting u8 HEX to String HEX
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
    Songtext,
    Music,
    Image,
    Font,
    Video,
    INVALID,
}

impl From<u32> for FileType {
    fn from(numeric: u32) -> Self {
        match numeric {
            1 => FileType::Songtext,
            2 => FileType::Music,
            3 => FileType::Image,
            4 => FileType::Font,
            5 => FileType::Video,
            _ => FileType::INVALID,
        }
    }
}

/// Representing a file entry in the KFN file.
#[derive(Debug, Clone)]
pub struct Entry {
    pub file_type: FileType,
    pub filename: String,
    pub len1: usize,
    pub offset: usize,
    pub len2: usize,
    pub flags: usize,
}

