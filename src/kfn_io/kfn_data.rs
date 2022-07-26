/// KfnHeader depicting the header contents of a Kfn file
#[derive(Debug)]
pub struct KfnData {
    pub path_songs_ini: String,
    pub syncs: Vec<usize>,
    text: Vec<String>,
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