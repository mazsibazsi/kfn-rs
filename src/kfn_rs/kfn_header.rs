use std::fmt::Debug;
use super::helpers::{u32_to_u8_arr, ToBinary};

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
}


impl KfnHeader {

    

}

impl ToBinary for KfnHeader {
    fn to_binary(&mut self) -> Vec<u8> {
        // create the data vector
        let mut data: Vec<u8> = Vec::new();

        // To learn more about the headers, please read the documentation bundled.

        // beginning of header
        data.append(&mut "KFNB".as_bytes().to_owned());
        // men difficulty
        data.append(&mut "DIFM".as_bytes().to_owned());
        data.push(1_u8);
        data.append(&mut u32_to_u8_arr(self.diff_men));
        // woman difficulty 
        data.append(&mut "DIFW".as_bytes().to_owned());
        data.push(1_u8);
        data.append(&mut u32_to_u8_arr(self.diff_women));
        // genre
        data.append(&mut "GNRE".as_bytes().to_owned());
        data.push(1_u8);
        data.append(&mut u32_to_u8_arr(self.genre));
        // sftv
        data.append(&mut "SFTV".as_bytes().to_owned());
        data.push(1_u8);
        data.append(&mut u32_to_u8_arr(self.sftv));
        // musl
        data.append(&mut "MUSL".as_bytes().to_owned());
        data.push(1_u8);
        data.append(&mut u32_to_u8_arr(self.musl));
        // anme
        data.append(&mut "ANME".as_bytes().to_owned());
        data.push(1_u8);
        data.append(&mut u32_to_u8_arr(self.anme));
        // type
        data.append(&mut "TYPE".as_bytes().to_owned());
        data.push(1_u8);
        data.append(&mut u32_to_u8_arr(self.kfn_type));
        // flid - encryption key
        data.append(&mut "FLID".as_bytes().to_owned());
        data.push(2_u8);
        data.append(&mut u32_to_u8_arr(self.flid.len() as u32));
        data.append(&mut self.flid.as_bytes().to_owned());
        // language
        data.append(&mut "LANG".as_bytes().to_owned());
        data.push(2_u8);
        data.append(&mut u32_to_u8_arr(self.language.len() as u32));
        data.append(&mut self.language.as_bytes().to_owned());
        // title
        data.append(&mut "TITL".as_bytes().to_owned());
        data.push(2_u8);
        data.append(&mut u32_to_u8_arr(self.title.len() as u32));
        data.append(&mut self.title.as_bytes().to_owned());
        // artist
        data.append(&mut "ARTS".as_bytes().to_owned());
        data.push(2_u8);
        data.append(&mut u32_to_u8_arr(self.artist.len() as u32));
        data.append(&mut self.artist.as_bytes().to_owned());
        // album
        data.append(&mut "ALBM".as_bytes().to_owned());
        data.push(2_u8);
        data.append(&mut u32_to_u8_arr(self.album.len() as u32));
        data.append(&mut self.album.as_bytes().to_owned());
        // composer
        data.append(&mut "COMP".as_bytes().to_owned());
        data.push(2_u8);
        data.append(&mut u32_to_u8_arr(self.composer.len() as u32));
        data.append(&mut self.composer.as_bytes().to_owned());
        // copyright
        data.append(&mut "COPY".as_bytes().to_owned());
        data.push(2_u8);
        data.append(&mut u32_to_u8_arr(self.copyright.len() as u32));
        data.append(&mut self.copyright.as_bytes().to_owned());
        // source
        data.append(&mut "SORC".as_bytes().to_owned());
        data.push(2_u8);
        data.append(&mut u32_to_u8_arr(self.source_file.len() as u32));
        data.append(&mut self.source_file.as_bytes().to_owned());
        // year
        data.append(&mut "YEAR".as_bytes().to_owned());
        data.push(2_u8);
        data.append(&mut u32_to_u8_arr(self.year.len() as u32));
        data.append(&mut self.year.as_bytes().to_owned());
        // track / trak
        data.append(&mut "TRAK".as_bytes().to_owned());
        data.push(2_u8);
        data.append(&mut u32_to_u8_arr(self.trak.len() as u32));
        data.append(&mut self.trak.as_bytes().to_owned());
        // karafunizer
        data.append(&mut "KFNZ".as_bytes().to_owned());
        data.push(2_u8);
        data.append(&mut u32_to_u8_arr(self.karafunizer.len() as u32));
        data.append(&mut self.karafunizer.as_bytes().to_owned());
        // rght / right
        data.append(&mut "RGHT".as_bytes().to_owned());
        data.push(1_u8);
        data.append(&mut u32_to_u8_arr(self.rght));
        // prov / 
        data.append(&mut "PROV".as_bytes().to_owned());
        data.push(1_u8);
        data.append(&mut u32_to_u8_arr(self.prov));
        // IDUS 
        data.append(&mut "IDUS".as_bytes().to_owned());
        data.push(2_u8);
        data.append(&mut u32_to_u8_arr(self.idus.len() as u32));
        data.append(&mut self.idus.as_bytes().to_owned());
        // END HEADER
        data.append(&mut "ENDH".as_bytes().to_owned());
        data.push(1_u8);
        data.append(&mut vec![255, 255, 255, 255]);

        data
    }
}