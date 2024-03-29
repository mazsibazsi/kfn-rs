mod tests;

/// Helpers, containing methods for handling the data and filetypes.
pub mod helpers;
/// Module for handling the main data of the KFN file, like songtexts and sync times.
pub mod kfn_data;
/// The header of the KFN file, containing non-essential data for playing, like the artist or title.
pub mod kfn_header;
/// The Song.ini file, containing essential information about the KFN.
pub mod kfn_ini;
/// Window for displaying the KFN file.
pub mod kfn_player;
/// Default fonts module
pub mod fonts;

pub mod kfn_thread;


// helpers
use crate::helpers::Entry;
use crate::helpers::file_type::FileType;
use crate::helpers::file_type::ToBinary;
use crate::helpers::event::{Event, EventType};

// header
use crate::kfn_header::KfnHeader;

use kfn_data::KfnData;

// player
use kfn_player::KfnPlayer;



#[derive(derivative::Derivative)]
#[derivative(Debug)]
/// Struct representing a .kfn file and it's components, like the header and data.
pub struct Kfn {
    
    /// The binary data in a vector of bytes.
    file_data: Vec<u8>,

    /// The read head, used in calculating the offset from the directory end.
    read_head: usize,
    
    /// The header data of the file.
    pub header: KfnHeader,

    /// The data container for the file.
    pub data: KfnData,

}

#[derive(Debug)]
pub enum KfnParseError {
    InvalidHeaderSignature(String),
    Utf8ConversionError,
}

impl Kfn {

    /// Constructor for creating a Kfn struct from an existing file.
    /// Takes the filename as parameter.
    pub fn open(filename: &str) -> Self {
        Self { 
            file_data: match std::fs::read(filename) {
                Ok(file) => file,
                Err(e) => panic!("File not found! {}", e),
            },
            read_head: usize::default(),
            header: KfnHeader::default(),
            data: KfnData::new(),

         }
    }

    /// Constructor for creating a new Kfn struct.
    pub fn new() -> Self {
        Self { 
            file_data: Vec::new(), 
            read_head: 0, 
            header: KfnHeader::default(), 
            data: KfnData::new(),

        }
    }


    /// Method for parsing the file itself.
    pub fn parse(&mut self) -> Result<bool, KfnParseError> {
        println!("Started parsing KFN file.");
        // read file signature
        let signature = match String::from_utf8(self.read_bytes(4)) {
            Ok(s) => s,
            Err(_) => return Err(KfnParseError::Utf8ConversionError),
        };
        // if file signature is not KFNB, end parsing
        if signature != "KFNB" {
            return Err(KfnParseError::InvalidHeaderSignature(signature));
        }
        
        // reading the header
        loop {
            // get signature
            let signature = match String::from_utf8(self.read_bytes(4)) {
                Ok(s) => s,
                Err(_) => return Err(KfnParseError::Utf8ConversionError),
            };
            // get type of the line
            let l_type = self.read_byte();
            let len_or_value = self.read_dword();

            // match for line type > if type 1, it's a value, if type 2 -> it contains header information
            match l_type {
                1 => {

                    match signature.as_str() {

                        "DIFM" => {
                            self.header.diff_men = len_or_value;
                        },
                        "DIFW" => {
                            self.header.diff_women = len_or_value;
                        },
                        "GNRE" => {
                            self.header.genre = len_or_value;
                        },
                        "SFTV" => {
                            self.header.sftv = len_or_value;
                        },
                        "MUSL" => {
                            self.header.musl = len_or_value;
                        },
                        "ANME" => {
                            self.header.anme = len_or_value;
                        },
                        "TYPE" => {
                            self.header.kfn_type = len_or_value;
                        },
                        "RGHT" => {
                            self.header.rght = len_or_value;
                        },
                        "PROV" => {
                            self.header.prov = len_or_value;
                        },
                        _ => println!("Unknown header entry in file."),
                    }
                },
                2 => {
                    // get data into buffer
                    let buffer = self.read_bytes(len_or_value);
                    let buffer_str = String::from_utf8(buffer.clone()).unwrap_or("Unknown".to_string());
                    // match header info and insert into header
                    match signature.as_str() {

                        "FLID" => {
                            self.header.flid = buffer_str;
                        },
                        "LANG" => {
                            self.header.language = buffer_str
                        },
                        "TITL" => {
                            self.header.title = buffer_str
                        },
                        "ALBM" => {
                            self.header.album = buffer_str
                        },
                        "ARTS" => {
                            self.header.artist = buffer_str
                        },
                        "COMP" => {
                            self.header.composer = buffer_str
                        },
                        "COPY" => {
                            self.header.copyright = buffer_str
                        },
                        "SORC" => {
                            self.header.source_file = buffer_str
                        },
                        "YEAR" => {
                            self.header.year = buffer_str
                        },
                        "TRAK" => {
                            self.header.trak = buffer_str
                        },
                        "KFNZ" => {
                            self.header.karafunizer = buffer_str
                        },
                        "IDUS" => {
                            self.header.idus = buffer_str
                        },
                        _ => ()
                    }
                   
                },
                _ => {

                },
            }

            if signature == "ENDH" {
                break;
            }
        }

        // reading the directory
        let num_files = self.read_dword();
        println!("# of files: {}", num_files);

        for _ in 0..num_files {

            let filename_len = self.read_dword();
            let filename = match String::from_utf8(self.read_bytes(filename_len)) {
                Ok(s) => s,
                Err(_) => return Err(KfnParseError::Utf8ConversionError),
            };
            let file_type = FileType::from(self.read_dword());
            let len1 = self.read_dword() as usize;
            let offset = self.read_dword() as usize;
            let len2 = self.read_dword() as usize;
            let flags = self.read_dword() as usize;

            let buf: Vec<u8> = Vec::default();

            self.data.entries.push(Entry {
                filename, file_type, len1, offset, len2, flags, file_bin: buf,
            });
            
        }

        self.data.offset_dir_end = self.read_head;
        // readjust offset
        for i in 0..self.data.entries.len() {
            self.data.entries[i].offset += self.data.offset_dir_end;
            self.data.entries[i].file_bin = 
                        Vec::from(
                            &self.file_data[
                                self.data.entries[i].offset..self.data.entries[i].offset +   self.data.entries[i].len1
                            ]
                        );
        }

        self.data.read_ini();
        
        self.data.song.load_eff();

        Ok(true)
    }

    // ----------------------
    // KFN MANIPULATION BLOCK
    // ----------------------
     
    /// Add file
    pub fn add_file(&mut self, source: &str) {

        self.data.add_entry_from_file(source);
        self.update();
    }

    /// Remove file
    pub fn remove_file(&mut self, target: &str) {

        self.data.remove_entry_by_name(target);
        self.update();
    }

    /// Takes the source filename and sets it as the song to play during playback.
    pub fn set_source(&mut self, target: &str) {

        self.header.source_file = target.to_string();
        self.data.song.set_source(target);
        self.update();
    }

    /// Update the ini file from header
    pub fn update(&mut self) {

        self.data.song.populate_from_header(&self.header);
        self.data.update_ini();
    }

    /// Get texts with syncs in.
    pub fn get_texts_and_syncs(&self) -> Vec<Event>
    //Vec<(usize, String)> 
    {

        /*let mut texts_and_syncs: Vec<(usize, String)> = Vec::new();
        
        let mut events: Vec<Event> = Vec::new();

        for eff in &self.data.song.effs {


            if eff.id >= 51 {
                // skip the 51 and above eff lines, as those are not text-sync entries
                continue;
            }
            let mut texts: Vec<(usize, String)> = Vec::new();
            let mut display: Vec<String> = Vec::new();
            for (i, text) in eff.texts.iter().enumerate() {

                if text == "" {
                    //texts.push(text.to_string());
                }
                if text.contains("/") || text.contains(" ") {

                    
                    let keywords: Vec<String> = text.split(&['/', ' '][..]).collect::<Vec<&str>>().iter().map(|s| s.to_string()).collect();

                    let displayed = text.split(&['/'][..]).collect::<Vec<&str>>().iter().map(|s| s.to_string()).collect::<Vec<String>>().join("");
                    for j in 0..keywords.len() {
                        //texts_and_syncs.push((eff.syncs[texts_and_syncs.len()], keywords[j].clone()));
                        events.push(Event {
                            event_type: EventType::Text(
                                keywords[j].clone()
                            ),
                            time: eff.syncs[texts_and_syncs.len()]
                        })
                    }
                    //dbg!(&keywords);
                    //dbg!(eff.syncs[i],&displayed);
                    
                    //texts.append(&mut keywords);
                
                }
            }
            
            //dbg!(&display);

            if texts.len() > 0 && eff.syncs.len() > 0 {
                // FIXME out of bounds exception happens here sometimes
                for i in 0..eff.texts.len() {
                    //dbg!(&texts_and_syncs);
                    //texts_and_syncs.push((display[i].clone(), (eff.syncs[i], texts[i].clone())))
                }

            }
        }
        events
        //texts_and_syncs */
        let mut events: Vec<Event> = Vec::new();
        for eff in &self.data.song.effs {
            if eff.id >= 51 {
                // skip the 51 and above eff lines, as those are not text-sync entries
                continue;
            }
            for text in &eff.texts {
                events.push(Event {
                    event_type: EventType::Text(text.to_owned()),
                    time: text.fragments[0].0
                })
            }
        }
        events
    }

    /// Co
    pub fn get_bg_events(&self) -> Vec<Event> {
        let mut bg_events: Vec<Event> = Vec::new();
        // Select the Eff# fields in the Songs.ini
            // Select the Anim# lines
            for anim in &self.data.song.effs[0].anims {
                // Separate the time
                let time = anim.time;
                // Go through each AnimEntry for their actions
                for animentry in anim.anim_entries.clone() {
                    bg_events.push(
                        Event {
                            event_type: EventType::Background(animentry),
                            time,
                        }
                    )
                }
            }
        
        //dbg!(&events);
        bg_events
    }

    /// Start playback and returns the thread receiver, that sends
    

    /// Start playback in a separate window.
    pub fn play_kfn(&mut self) {

        // falling back to X11/Xorg, for server side decoration
        std::env::set_var("WAYLAND_DISPLAY", "");



        let window = speedy2d::Window::new_centered(&self.header.title, (800, 600)).unwrap();
        
        let events = self.get_bg_events();
        //dbg!(&events);
        let (sender, receiver) = self.play();
            window.run_loop(
                KfnPlayer::new(self.data.clone(), 
                (800, 600), 
                events, 
                receiver, 
                sender)
        );

    }

    /// Exporting to .kfn 
    pub fn export(&mut self, filename: &str) {
        
        let mut data: Vec<u8> = Vec::new();
        
        data.append(&mut self.header.to_binary());
        data.append(&mut self.data.to_binary());
        
        std::fs::write(filename, data).unwrap();
    }

    /// Extracting all files.
    pub fn extract_all(&mut self, target_dir: &str) {
        
        // iterate over all entries
        for i in 0..self.data.entries.len() {
        
            // get the target directory into the filename
            let mut filename = target_dir.to_string();

            // add the actual filename
            filename.push_str(&self.data.entries[i].clone().filename.to_string());
        
            // send it to extraction
            self.extract(self.data.entries[i].clone(), &filename);
        }
    }

    /// Extracting a single file from the entry to a deisgnated output.
    pub fn extract(&mut self, entry: Entry, output_filename: &str) {
        
        // set the path and prefix
        let path = std::path::Path::new(&output_filename);
        let prefix = path.parent().unwrap();
        
        // create directories if they don't exist
        std::fs::create_dir_all(prefix).unwrap();
        
        let mut output = std::fs::File::create(path).unwrap();

        let buf: Vec<u8> = Vec::from(entry.file_bin);
        
        std::io::Write::write_all(&mut output, &buf).unwrap();
    }
    


    // -----------------------
    // BINARY HELPER FUNCTIONS
    //------------------------

    /// Helper IO function for reading a byte
    fn read_byte(&mut self) -> u8 {
        
        let result = self.file_data[self.read_head as usize];
        
        self.read_head += 1;
        
        (result & 0xFF).into()
    }

    /// Helper IO function for reading a dword
    fn read_dword(&mut self) -> u32 {
        
        let b1 = self.read_byte() as u32;
        let b2 = self.read_byte() as u32;
        let b3 = self.read_byte() as u32;
        let b4 = self.read_byte() as u32;

        b4 << 24 | b3 << 16 | b2 << 8 | b1
    }

    /// Helper IO function for reading a specified amount fo bytes
    fn read_bytes(&mut self, length: u32) -> Vec<u8> {
        
        let mut array: Vec<u8> = Vec::with_capacity(length as usize);
        
        for _ in 0..length {
        
            array.push(self.read_byte());
        }
        
        array
    }

}