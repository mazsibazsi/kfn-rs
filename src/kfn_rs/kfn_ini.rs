use ini::Ini;

use crate::kfn_rs::helpers::eff::AnimEntry;

use super::helpers::Entry;
use super::helpers::eff::{Eff, Effect, Anim, Action, TransType};

/// Wrapper for the INI file.
#[derive(Default, Clone)]
pub struct KfnIni {
    pub ini: Ini,
    
    eff: Vec<Eff>,
}

impl KfnIni {
    pub fn new() -> Self {

        Self { ini: Ini::new(), eff: Vec::new(), }
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

            // number of animations
            let nb_anim = section.get("NbAnim").unwrap().to_string().parse::<usize>().unwrap();
            // list of animations in Anim# form
            let mut anims: Vec<Anim> = Vec::new();
            
            dbg!(nb_anim);
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
            self.eff.push(Eff {anims});
        } // for i in 1..effect_count {
        dbg!(&self.eff);
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