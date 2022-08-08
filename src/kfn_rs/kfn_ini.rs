use ini::Ini;

use super::helpers::Entry;
use super::helpers::eff::Eff;
use super::helpers::eff::Anim;

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

    pub fn read_eff(&self) {
        
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
            let anims: Vec<Anim> = Vec::new();
            
            dbg!(nb_anim);
            if nb_anim != 0 {
                for j in 0..nb_anim {

                    let mut key = String::from("Anim");
    
                    key.push_str(&j.to_string());
    
                    dbg!(section.get(key));
                    
                    // needs parsing at '|'
    
                }
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