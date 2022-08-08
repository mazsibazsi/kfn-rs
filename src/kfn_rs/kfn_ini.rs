use ini::Ini;

use super::helpers::Entry;

/// Wrapper for the INI file.
#[derive(Default, Clone)]
pub struct KfnIni {
    pub ini: Ini,
}

impl KfnIni {
    pub fn new() -> Self {

        Self { ini: Ini::new() }
    }

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