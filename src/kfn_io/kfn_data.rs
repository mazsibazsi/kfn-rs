
/// KfnHeader depicting the header contents of a KFN file
#[derive(Debug)]
pub struct KfnData {
    /// The location of the Songs.ini file.
    pub path_songs_ini: String,
    /// Sync timestamps
    pub syncs: Vec<usize>,
    /// Lyrics
    pub text: Vec<String>,
}

impl KfnData {
    pub fn new() -> Self {
        let dir_songs_ini = String::new();
        let syncs = Vec::new();
        let text = Vec::new();
        Self {
            path_songs_ini: dir_songs_ini, syncs, text, 
        }
    }



}