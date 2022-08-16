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

use std::io::Cursor;
// standard lib
use std::fs;
use std::fs::File;
use std::io::BufReader;
use std::io::Write;
use std::path::Path;
use std::thread;
use std::time::Instant;

// helpers
use crate::helpers::Entry;
use crate::helpers::file_type::FileType;
use crate::helpers::file_type::ToBinary;

// rodio
use rodio::{OutputStream, Sink};
use speedy2d::dimen::Vector2;


// header
use crate::kfn_header::KfnHeader;

// data
use kfn_data::KfnData;

// player
use kfn_player::KfnPlayer;

// speedy2d helpers
use speedy2d::Window;

// crossbeam
use crossbeam::channel::{Sender, Receiver, unbounded};

// derivative
use derivative::Derivative;


#[derive(Derivative)]
#[derivative(Debug)]
/// Struct representing a KFN file and it's components.
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
            file_data: match fs::read(filename) {
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
                        _ => println!("{}, type 1, value {:x}", signature, len_or_value),
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
                                self.data.entries[i].offset
                                ..
                                self.data.entries[i].offset +   self.data.entries[i].len1
                            ]
                        );
        }

        self.data.read_ini();
        
        self.data.song.read_eff();

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
    pub fn get_texts_and_syncs(&self) -> Vec<(String, (usize, String))> {


        let mut texts_and_syncs: Vec<(String, (usize, String))> = Vec::new();
         
        for eff in &self.data.song.effs {


            if eff.id >= 51 {

                continue;
            }

            let mut texts: Vec<String> = Vec::new();
            let mut display: Vec<String> = Vec::new();
            for text in &eff.texts {

                if text == "" {
                    //texts.push(text.to_string());
                }
                if text.contains("/") || text.contains(" ") {

                    
                    let mut line: Vec<String> = text.split(&['/', ' '][..]).collect::<Vec<&str>>().iter().map(|s| s.to_string()).collect();
                    


                    let displayed = text.split(&['/'][..]).collect::<Vec<&str>>().iter().map(|s| s.to_string()).collect::<Vec<String>>().join("");
                    for _ in 0..line.len() {
                        display.push(displayed.clone());
                    }
                    
                    texts.append(&mut line);
                
                }
            }

            dbg!(&display);

            if texts.len() > 0 && eff.syncs.len() > 0 {

                for i in 0..eff.syncs.len()-1 {

                    texts_and_syncs.push((display[i].clone(), (eff.syncs[i], texts[i].clone())))
                }

            }
        }
        texts_and_syncs
    }

    /// Start playback and returns the thread receiver, that sends
    pub fn play(&mut self) -> (Sender<String>, Receiver<String>) {

        // initialize channels
        let (sender_player, receiver_caller): (Sender<String>, Receiver<String>) = unbounded();
        let (sender_caller, receiver_player): (Sender<String>, Receiver<String>) = unbounded();
        // read audio file
        let cursor: Cursor<Vec<u8>> = Cursor::new(self.data.get_entry_by_name(&self.data.song.get_source_name()).unwrap().file_bin);
        // get sync times
        let syncs_times = self.get_texts_and_syncs();

        dbg!(&syncs_times);
        thread::spawn(move || {
            let (_stream, stream_handle) = OutputStream::try_default().unwrap();
            let sink = Sink::try_new(&stream_handle).unwrap();
            // add it to the sink
            sink.append(rodio::Decoder::new(BufReader::new(cursor)).unwrap());
            
            let start = Instant::now();
            let mut i = 0;

            loop {

                match receiver_player.try_recv() {
                    Ok(s) => {
                        match s.as_str() {
                            "END" => break,
                            &_ => (),
                        }
                    },
                    Err(_) => (),
                }

                if (syncs_times[i].1.0 * 10) as u128 == start.elapsed().as_millis() {
                    sender_player.send(syncs_times[i].0.clone()).unwrap();
                    //sender_player.send(format!("sync {} elapsed {}", syncs_times[i].1.0 * 10, start.elapsed().as_millis())).unwrap();
                    i += 1;
                }
                

            }
            
        });

        (sender_caller, receiver_caller)
    }

    /// Start playback in a separate window.
    pub fn play_kfn(&mut self) {

        // falling back to X11/Xorg, for server side decoration
        std::env::set_var("WAYLAND_DISPLAY", "");



        let window = Window::new_centered("Title", (800, 600)).unwrap();
        
        let (sender, receiver) = self.play();
        window.run_loop(
            KfnPlayer::new(self.data.clone(), (800, 600), receiver)
        );

    }

    /// Exporting to .kfn 
    pub fn export(&mut self, filename: &str) {
        
        let mut data: Vec<u8> = Vec::new();
        
        data.append(&mut self.header.to_binary());
        data.append(&mut self.data.to_binary());
        
        fs::write(filename, data).unwrap();
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
        let path = Path::new(&output_filename);
        let prefix = path.parent().unwrap();
        
        // create directories if they don't exist
        fs::create_dir_all(prefix).unwrap();
        
        let mut output = File::create(path).unwrap();

        let buf: Vec<u8> = Vec::from(entry.file_bin);
        
        output.write_all(&buf).unwrap();
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
