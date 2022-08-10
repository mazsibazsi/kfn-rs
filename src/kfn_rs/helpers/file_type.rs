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