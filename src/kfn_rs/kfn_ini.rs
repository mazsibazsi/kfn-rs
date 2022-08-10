use ini::Ini;
use ini::Properties;

use crate::kfn_rs::helpers::eff::AnimEntry;
use crate::kfn_rs::helpers::eff::Trajectory;

use super::helpers::Entry;
use super::helpers::eff::{Eff, Effect, Anim, Action, TransType};

/// Wrapper for the INI file.
#[derive(Default, Clone)]
pub struct KfnIni {
    pub ini: Ini,
    
    pub effs: Vec<Eff>,
}

impl KfnIni {
    pub fn new() -> Self {

        Self { ini: Ini::new(), effs: Vec::new(), }
    }

    /// Populating the [General] section with empty data.
    pub fn populate(&mut self) {

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

    pub fn read_eff(&mut self) {
        
        // get the number of effects to parse
        let effect_count = self.ini.get_from(Some("General"), "EffectCount").unwrap().to_string().parse::<usize>().unwrap();

        for i in 1..effect_count {
            
            let mut eff = String::from("Eff");

            eff.push_str(&i.to_string());
            
            let section = self.ini.section(Some(eff)).unwrap();
            
            // TODO implement the rest of the properties
            let id = section.get("ID").unwrap().to_string().parse::<usize>().unwrap();

            // number of animations
            let nb_anim = section.get("NbAnim").unwrap().to_string().parse::<usize>().unwrap();
            let text_count = section.get("TextCount").unwrap_or("0").to_string().parse::<usize>().unwrap();
            let trajectory = Trajectory::from(
                section.get("Trajectory").unwrap_or_default()
            );
            // list of animations in Anim# form
            let mut anims: Vec<Anim> = Vec::new();
            let mut syncs: Vec<usize> = Vec::new();
            let mut texts: Vec<String> = Vec::new();
            
            dbg!(nb_anim);
            // reading the animations, if there are any.
            if nb_anim != 0 {
                for j in 0..nb_anim {

                    let mut anim_entries: Vec<AnimEntry> = Vec::new();

                    let mut key = String::from("Anim");
    
                    key.push_str(&j.to_string());
    
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
            dbg!(&syncs);

            if text_count != 0 {

                for j in 0..text_count {
                    let mut key = String::from("Text");
                    key.push_str(&j.to_string());

                    let value = section.get(key).unwrap();

                    texts.push(value.to_owned());
                }
                
            }
            dbg!(&texts);
            self.effs.push(Eff { id, anims, syncs, texts, trajectory});
        } // for i in 1..effect_count {
       
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
            
            // get essential fields
            self.ini.with_section(Some(eff_section.clone())).set("ID", self.effs[eff_n].id.to_string());
            self.ini.with_section(Some(eff_section.clone())).set("NbAnim", self.effs[eff_n].anims.len().to_string());
            self.ini.with_section(Some(eff_section.clone())).set("TextCount", self.effs[eff_n].texts.len().to_string());
            
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

                let text_value = self.effs[eff_n].texts[text_n].clone();

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