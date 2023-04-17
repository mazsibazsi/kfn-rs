pub mod eff;
pub mod trajectory;

use ini::Ini;

use eff::{AnimEntry, Eff, Effect, Action, TransType, Anim};

use trajectory::Trajectory;

use crate::kfn_header::KfnHeader;

use crate::helpers::Entry;
use crate::kfn_ini::eff::TextEntry;


/// The Song.ini file, which is at the very end of a .kfn file.
/// Contains most of the information for replicating a karaoke video.
#[derive(Default, Clone)]
pub struct KfnIni {
    /// The Song.ini file itself, represented using the ini-rust library.
    /// To learn more: https://github.com/zonyitoo/rust-ini
    pub ini: Ini,
    /// Representation of the various effects, texts and syncs.
    pub effs: Vec<Eff>,
}

impl KfnIni {
    /// Creating a new ini file.
    pub fn new() -> Self {
        Self { ini: Ini::new(), effs: Vec::new(), }
    }

    /// Populating the General section with empty data.
    pub fn populate_empty(&mut self) {

        self.ini.with_section(Some("General"))
            .set("Title", "")
            .set("Artist", "")
            .set("Album", "")
            .set("Composer", "")
            .set("Year", "")
            .set("Track", "")
            .set("GenreID", "-1")
            .set("Copyright", "")
            .set("Comment", "")
            .set("Source", "")
            .set("EffectCount", "")
            .set("LanguageID", "")
            .set("DiffMen", "")
            .set("DiffWomen", "")
            .set("KFNType", "0")
            .set("Properties", "")
            .set("KaraokeVersion", "")
            .set("VocalGuide", "")
            .set("KaraFunization", "");

    }

    /// Returns the secondary source / vocal included track, if it exists.
    pub fn get_secondary_source(&self) -> Option<String> {
        if self.ini.get_from(Some("MP3Music"), "Track0") != None {
            let value = self.ini.get_from(Some("MP3Music"), "Track0").unwrap();
            Some(
                self.ini.get_from(Some("MP3Music"), "Track0").unwrap()[..value.rfind(".mp3").unwrap()+4].to_string()
            )
        } else {
            None
        }
    }

    pub fn replaces_track(&self) -> bool {
        if self.ini.get_from(Some("MP3Music"), "Track0") != None {
            if self.ini.get_from(Some("MP3Music"), "Track0").unwrap().split(',').collect::<Vec<&str>>()[2] == "0" {
                return false;
            } else {
                return true;
            }
        }
        false
    }

    /// Populating the General section with empty data.
    pub fn populate_from_header(&mut self, header: &KfnHeader) {

        let mut source = String::new();
        if header.source_file.len() > 4 {
            if &header.source_file[0..=3] != "1,I," {
                source.push_str("1,I,");
            }
        }
        
        source.push_str(&header.source_file);

        

        self.ini.with_section(Some("General"))
            .set("Title", &header.title)
            .set("Artist", &header.artist)
            .set("Album", &header.album)
            .set("Composer", &header.composer)
            .set("Year", &header.year)
            .set("Track", &header.trak)
            .set("GenreID", &header.genre.to_string())
            .set("Copyright", &header.copyright)
            .set("Comment", "")
            .set("Source", source)
            .set("EffectCount", "")
            .set("LanguageID", &header.language)
            .set("DiffMen", header.diff_men.to_string())
            .set("DiffWomen", header.diff_women.to_string())
            .set("KFNType", header.kfn_type.to_string())
            .set("Properties", "")
            .set("KaraokeVersion", "")
            .set("VocalGuide", "")
            .set("KaraFunization", &header.karafunizer);

    }

    /// Reading the Eff# headed sections
    pub fn load_eff(&mut self) {
        
        // get the number of effects to parse
        let effect_count = self.ini.get_from(Some("General"), "EffectCount").unwrap_or("0").to_string().parse::<usize>().unwrap();

        // based on the number of effects...
        for i in 1..=effect_count {
            // create a string "Eff#" 
            let eff = format!("Eff{n}", n = &i);
            
            // select the Eff# section based on the string we previously constructed
            let section = self.ini.section(Some(eff)).unwrap();
            
            // TODO implement the rest of the properties
            let id = section.get("ID").unwrap().to_string().parse::<usize>().unwrap();

            // number of animations
            let nb_anim = section.get("NbAnim").unwrap().to_string().parse::<usize>().unwrap();
            // number of text lines
            let text_count = section.get("TextCount").unwrap_or("0").to_string().parse::<usize>().unwrap();
            // starting trajectory
            let initial_trajectory = Trajectory::from(
                section.get("Trajectory").unwrap_or_default()
            );
            // looking for initial library image
            let initial_lib_image = match section.get("LibImage") {
                Some(s) => {
                    if s != "" {
                        Some(s.to_string())
                    } else {
                        None
                    }
                    
                },
                None => None,
            };

            let initial_inactive_color = match section.get("InactiveColor") {
                Some(s) => {
                    Some(s.to_string())
                },
                None => None
            };

            // looking for initial video file
            let initial_video_file = match section.get("VideoFile") {
                Some(s) => {
                    if s != "" {
                        Some(s.to_string())
                    } else {
                        None
                    }
                    
                },
                None => None,
            };
            // looking for initial font
            let initial_font: Option<(String, u32)> = match section.get("Font") {
                Some(s) => {
                    let res: Vec<&str> = s.split("*").collect();
                    dbg!(&res);
                    let filename = res[0];
                    let extension = &filename[filename.len()-4..filename.len()];
                    if extension == ".ttf" || extension == ".TTF" || extension == ".otf" {
                        Some((res[0].to_string(), u32::from_str_radix(res[1], 10).unwrap()))
                    } else {
                        None
                    }
                },
                // if none, revert to Arial Black, as that is the default in the original program
                None => {
                    None
                    //("Arial Black".to_string(), 12)
                }
            };
            dbg!(&initial_font);
            // list of animations in Anim# form
            let mut anims: Vec<Anim> = Vec::new();
            let mut syncs: Vec<usize> = Vec::new();
            let mut texts: Vec<TextEntry> = Vec::new();
            //let mut texts: Vec<String> = Vec::new();
            
            dbg!(nb_anim);
            // reading the animations, if there are any.
            if nb_anim != 0 {
                for j in 0..nb_anim {

                    // create a vector for the AnimEntries
                    let mut anim_entries: Vec<AnimEntry> = Vec::new();

                    // construct the key with the proper number
                    let key = format!("Anim{n}", n = &j);


                    let value = section.get(key).unwrap();
                    
                    // the time in ms, when the anim occurs. The first one will always be the time.
                    let time = value
                                            .split('|')
                                            .collect::<Vec<&str>>()
                                            [0]
                                            .parse::<usize>()
                                            .unwrap();
                    let remaining: Vec<&str> = value
                                                    .split('|').collect::<Vec<&str>>()
                                                    .split_first()
                                                    .unwrap()
                                                    .1
                                                    .to_owned();
                    
                    for i in 0..remaining.len() {
                        let tokens: Vec<&str> = remaining[i].split(',').collect();

                        // first one is always the action
                        let action = Action::from(tokens[0]);
                        
                        let mut effect: Option<Effect> = None;
                        let mut trans_time: f64 = 0.0;
                        let mut trans_type = TransType::default();

                        for j in 0..tokens.len() {
                            let key = tokens[j].split('=').collect::<Vec<&str>>()[0];
                            let value = tokens[j].split('=').collect::<Vec<&str>>()[1];
                            match key  {
                                "Effect" => effect = Some(Effect::from(value)),
                                "TransitionTime" => trans_time = value.parse().unwrap(),
                                "TransitionType" => trans_type = TransType::from(value),
                                &_ => ()
                            }
                        }

                        let anim_entry = AnimEntry { action, effect, trans_time, trans_type };
                        anim_entries.push(anim_entry)
                    }
                    anims.push(Anim {time, anim_entries});
                } // for j in 0..nb_anim {
            } // if nb_anim != 0 {

            // reading sync data
            for (key, value) in section.iter() {
                // guard clause, only read Sync#, not InSync 
                if key.contains("Sync") && !key.contains("InSync") {
                    let mut sync_times: Vec<usize> = value.split(',').collect::<Vec<&str>>().iter().map(|s| s.parse::<usize>().unwrap()).collect::<Vec<usize>>();
                    syncs.append(&mut sync_times);
                }
            }
            dbg!(&text_count);
            
            if text_count != 0 {
                let mut sync_counter = 0;
                dbg!(syncs.len());
                while sync_counter < syncs.len() {
                    for j in 0..text_count {
                        let key = format!("Text{n}", n = &j);
                        let value = section.get(key).unwrap_or_default();
                        if value == "" {
                            continue;
                        }
                        let mut fragments: Vec<(usize, String)> = Vec::new();
                        let fragments_vec: Vec<String> = value.split(&['/', ' '][..]).collect::<Vec<&str>>().iter().map(|s| s.to_string()).collect();
                        let display: String = value.split('/').collect::<Vec<&str>>().iter().map(|s| s.to_string()).collect::<Vec<String>>().join("");
                        for fragment_string in &fragments_vec {
                            dbg!(fragment_string);
                            fragments.push((syncs[sync_counter], fragment_string.to_string()));
                            sync_counter += 1;
                        }
                        texts.push(TextEntry {
                            display,
                            fragments,
                        });
                        
                        //texts.push(value.to_owned());
                    }
                }
                
                
            }
            
            
            //dbg!(&texts);
            self.effs.push(
                Eff { 
                    id,
                    anims,
                    syncs,
                    texts,
                    initial_trajectory,
                    initial_lib_image,
                    initial_video_file,
                    initial_font,
                    initial_inactive_color,
                }
            );
        } // for i in 1..effect_count {
       
    }

    /// Returns the name of the source sound file. 
    pub fn get_source_name(&self) -> String {
        dbg!(self.ini.get_from(Some("General"), "Source").unwrap_or_default()[4..].to_string());
        self.ini.get_from(Some("General"), "Source").unwrap()[4..].to_string()
    }

    /// Method for setting up the effect in the Ini file.
    pub fn set_eff(&mut self) {

        // Set the EffectCount - number of Eff sections in the Ini.
        self.ini.section_mut(Some("General")).unwrap().insert("EffectCount", self.effs.len().to_string());

        // Iterate through ID of effects
        for eff_n in 0..self.effs.clone().len() {
            
            // prepare for section header
            let mut eff_section = String::from("Eff");

            // push number to section header, indexing starts at 1!
            eff_section.push_str((eff_n + 1).to_string().as_str());
            
            let mut section = self.ini.with_section(Some(eff_section.clone()));
            let eff = &self.effs[eff_n];
            // get essential fields
            section
                .set("ID", &eff.id.to_string())
                .set("NbAnim", eff.anims.len().to_string())
                .set("TextCount", eff.texts.len().to_string())
                .set("Trajectory", eff.initial_trajectory.to_string());

            // iterate through Anim# 
            for anim_n in 0..self.effs[eff_n].anims.len() {

                // get into the appropriate section
                let mut section = self.ini.with_section(Some(eff_section.as_str()));

                // clone the Anim#
                let anim = self.effs[eff_n].anims[anim_n].clone();

                // prepare string for manipulation
                let mut anim_key = String::from("Anim");
                // attach row #
                anim_key.push_str(anim_n.to_string().as_str());

                // prepare value
                let mut anim_val = String::new();

                // add time, as that is always the first value in line
                anim_val.push_str(anim.time.to_string().as_str());

                // separator
                anim_val.push('|');

                // iterate through the entries
                for anim_entry in anim.anim_entries {
                    // and push the appropriate value
                    anim_val.push_str(anim_entry.action.to_string().as_str())
                }
                
                // lastly set it
                section.set(anim_key, anim_val);
                
            }
        
            self.ini.with_section(Some(eff_section.clone())).set("Sync0", self.effs[eff_n].syncs.to_owned().iter().map(|n| n.to_string()).collect::<Vec<String>>().join(","));
            
            for text_n in 0..self.effs[eff_n].texts.len() {
                let mut section = self.ini.with_section(Some(eff_section.as_str()));

                let text_value = match self.effs[eff_n].texts[text_n].clone() {
                    TextEntry { display, fragments } => display
                };

                // prepare string for manipulation
                let mut text_key = String::from("Text");
                // attach row #
                text_key.push_str(text_n.to_string().as_str());

                section.set(text_key, text_value);
            }
        }
    }

    /// Setting the source file for the KFN. This must be a music type file.
    pub fn set_source(&mut self, source: &str) {

        let mut value = String::from("1,I,");
        
        value.push_str(source);
        
        self.ini.with_section(Some("General")).set("Source", value);
    
    }

    /// Sets the list of files in the ini, based on the entries given.
    pub fn set_materials(&mut self, materials: Vec<Entry>) {

        let mat_count = materials.len()-1;

        self.ini.with_section(Some("Materials")).set("MatCount", mat_count.to_string());

        for n in 0..mat_count {

            let mut key = String::from("Mat");
            
            key.push_str(n.to_string().as_str());

            let value = &materials[n].filename;
            
            self.ini.with_section(Some("Materials")).set(key.as_str(), value.as_str());
            
        }

    }


}