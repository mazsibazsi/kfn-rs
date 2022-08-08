/// The module, that is responsible for handling the reading and writing of the KFN file.
pub mod kfn_rs;

#[cfg(test)]
mod tests {
    use crate::kfn_rs::Kfn;

    #[test]
    fn file_reading() {

        let mut kfn = Kfn::read("test/input.kfn");

        match kfn.dump() {
            Ok(true) => {
            },
            Ok(false) => {
            },
            Err(error) => {
                println!("{}", error);
            }
        }
        
    }

    #[test]
    fn file_writing() {
        
        let mut kfn = Kfn::read("test/input.kfn");
        
        kfn.dump().unwrap();
        
        kfn.export("test/output_write_test.kfn");
    }

    #[test]
    fn ini_test() {

        let mut kfn = Kfn::read("test/input.kfn");

        kfn.dump().unwrap();
        
        kfn.data.read_ini();
        kfn.data.update_ini();
        
        kfn.export("test/output_ini_test.kfn");
    }

    #[test]
    fn add_entry_test() {

        let mut kfn = Kfn::read("test/input.kfn");

        kfn.dump().unwrap();

        kfn.add_file("test/art_for_test.jpg");


        kfn.export("test/output_add_test.kfn");
    }

    #[test]
    fn remove_entry_test() {

        let mut kfn = Kfn::read("test/input.kfn");
        kfn.dump().unwrap();

        kfn.remove_file("odo_p1.jpg");

        kfn.export("test/output_remove_test.kfn");
    }

    #[test]
    fn extract_test() {

        let mut kfn = Kfn::read("test/input.kfn");

        kfn.dump().unwrap();

        kfn.extract_all("test/");
    }

    #[test]
    fn create_test() {

        let mut kfn = Kfn::new();

        kfn.add_file("test/Ado - Odo.mp3");
        kfn.add_file("test/Ado - Odo (Karaoke).mp3");
        kfn.add_file("test/art_for_test.jpg");

        kfn.set_source("Ado - Odo (Karaoke).mp3");

        kfn.export("test/new_output.kfn");
    }
}
