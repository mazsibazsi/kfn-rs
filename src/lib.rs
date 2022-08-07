/// The module, that is responsible for handling the reading and writing of the KFN file.
pub mod kfn_rs;

#[cfg(test)]
mod tests {
    use crate::kfn_rs::Kfn;

    #[test]
    fn file_reading() {
        let mut kfn = Kfn::read("input.kfn");
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
        let mut kfn = Kfn::read("input.kfn");
        kfn.dump().unwrap();
        kfn.export("output_write_test.kfn");
    }

    #[test]
    fn ini_test() {
        let mut kfn = Kfn::read("input.kfn");
        kfn.dump().unwrap();
        kfn.data.read_ini();
        kfn.data.update_ini();
        kfn.export("output_ini_test.kfn");
    }

    #[test]
    fn add_entry_test() {
        let mut kfn = Kfn::read("input.kfn");
        kfn.dump().unwrap();

        kfn.data.add_entry_from_file("art_for_test.jpg");

        kfn.data.read_ini();
        kfn.data.update_ini();

        kfn.export("output_add_test.kfn");
    }

    #[test]
    fn remove_entry_test() {
        let mut kfn = Kfn::read("input.kfn");
        kfn.dump().unwrap();

        kfn.data.remove_entry_by_name("odo_p1.jpg");

        kfn.data.read_ini();
        kfn.data.update_ini();

        kfn.export("output_remove_test.kfn");
    }
}
