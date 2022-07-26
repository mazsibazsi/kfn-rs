use std::fmt::Write;

pub fn dump_hex(array: Vec<u8>) -> String {
    let mut output = String::new();
    for i in 0..array.len() {
        if i > 0 {
            output.push(' ');
        }
        let mut tmp = String::new();
        write!(&mut tmp, "{:X} ", array[i] & 0xFF);
        output.push_str(&tmp);
    }
    output
}

#[derive(Debug)]
pub enum FileType {
    TYPE_SONGTEXT,
    TYPE_MUSIC,
    TYPE_IMAGE,
    TYPE_FONT,
    TYPE_VIDEO,
}

#[derive(Debug)]
pub struct Entry {
    pub f_type: FileType,
    pub filename: String,
    pub len1: u8,
    pub len2: u8,
    pub offset: u8,
    pub flags: u8,
}