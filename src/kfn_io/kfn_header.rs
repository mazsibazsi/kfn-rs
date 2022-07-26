/// Header, containing information about the KFN file. WIP
#[derive(Debug)]
pub struct KfnHeader {
    pub title: String,
    pub artist: String,
    pub karafunizer: String,
    // TODO have all header entries represented
}

impl KfnHeader {
    pub fn new() -> Self {
        Self { title: "".to_string(), artist: "".to_string(), karafunizer: "".to_string() }
    }
    // TODO have header entries be reproducible/rewritable to file.
}