use std::fmt::Debug;

/// Header, containing information about the KFN file. WIP
#[derive(Debug, Default)]
pub struct KfnHeader {
    /// Difficulty for men. Value between 1 to 5.
    pub diff_men: u32,  
    /// Difficulty for women. Value between 1 to 5.
    pub diff_women: u32,
    pub genre: u32,
    pub sftv: u32,
    pub musl: u32,
    pub anme: u32,
    pub kfn_type: u32,
    pub flid: String,
    pub language: String,
    pub album: String,
    pub title: String,
    pub artist: String,
    pub composer: String,
    pub copyright: String,
    pub source_file: String,
    pub year: String,
    pub trak: String,
    pub rght: u32,
    pub prov: u32,
    pub karafunizer: String,
    pub idus: String,
    // TODO have all header entries represented
}


impl KfnHeader {
    // Creating a new empty header file without data.
    /* pub fn new() -> Self {
        Self { 
            
            title: "".to_string(), 
            artist: "".to_string(), 
            karafunizer: "".to_string() }
    }*/
    // TODO have header entries be reproducible/rewritable to file.
}
