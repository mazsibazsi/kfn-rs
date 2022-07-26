use std::fmt::Write;

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

#[derive(Debug, Clone)]
pub struct Entry {
    pub file_type: FileType,
    pub filename: String,
    pub len1: usize,
    pub offset: usize,
    pub len2: usize,
    pub flags: usize,
}

#[derive(Debug)]
pub struct Header {
    pub title: String,
    pub artist: String,
    pub karafunizer: String,
}

impl Header {
    pub fn new() -> Self {
        Self { title: "".to_string(), artist: "".to_string(), karafunizer: "".to_string() }
    }
}