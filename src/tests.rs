#[cfg(test)]
mod tests {

    use std::{time::{Instant, Duration}};


    use crate::{Kfn, kfn_header::KfnHeader, helpers::event::EventType};

    #[test]
    fn file_reading() {

        let mut kfn = Kfn::open("test/input.kfn");

        match kfn.parse() {
            Ok(true) => {
            },
            Ok(false) => {
            },
            Err(_) => {
                panic!("KfnParseError");
            }
        }
        
    }

    #[test]
    fn file_writing() {
        
        let mut kfn = Kfn::open("test/input.kfn");
        
        kfn.parse().unwrap();
        
        kfn.export("test/output_write_test.kfn");
    }

    #[test]
    fn ini_test() {

        let mut kfn = Kfn::open("test/input.kfn");

        kfn.parse().unwrap();
        
        kfn.data.read_ini();
        kfn.data.update_ini();
        
        kfn.export("test/output_ini_test.kfn");
    }

    #[test]
    fn add_entry_test() {

        let mut kfn = Kfn::open("test/input.kfn");

        kfn.parse().unwrap();

        kfn.add_file("test/art_for_test.jpg");


        kfn.export("test/output_add_test.kfn");
    }

    #[test]
    fn remove_entry_test() {

        let mut kfn = Kfn::open("test/input.kfn");
        kfn.parse().unwrap();

        //kfn.remove_file("target")

        kfn.export("test/output_remove_test.kfn");
    }

    #[test]
    fn extract_test() {

        let mut kfn = Kfn::open("test/input.kfn");

        kfn.parse().unwrap();

        kfn.extract_all("test/extract/");
    }

    #[test]
    fn create_test() {

        let mut kfn = Kfn::new();

        kfn.add_file("test/insert.mp3");
        kfn.add_file("test/art_for_test.jpg");

        kfn.header = KfnHeader::default();

        kfn.data.song.populate_from_header(&kfn.header);

        kfn.set_source("insert.mp3");

        kfn.export("test/new_output.kfn");
    }

    #[test]
    fn read_anims_test() {

        let mut kfn = Kfn::open("test/input.kfn");
        
        kfn.parse().unwrap();

        kfn.data.song.load_eff();

    }

    #[test]
    fn create_test_read_anims() {

        let mut kfn = Kfn::open("test/input.kfn");
        
        kfn.parse().unwrap();

        kfn.data.song.load_eff();
        
        kfn.data.song.ini.clear();

        kfn.data.song.populate_from_header(&kfn.header);
        
        //kfn.add_file("test/insert.mp3");

        kfn.data.song.set_eff();
        kfn.data.update_ini();

        kfn.extract(kfn.data.get_entry_by_name("Song.ini").unwrap(), "test/new_Song.ini");

        kfn.export("test/new_output_ini.kfn");
    }

    #[test]
    #[ignore]
    fn playback_test() {

        let mut kfn = Kfn::open("test/input.kfn");
    
        kfn.parse().unwrap();

        kfn.data.song.load_eff();

        //kfn.get_texts_and_syncs();

        let (sender_caller, receiver_caller) = kfn.play();
        //sender_caller.send("END".to_string()).unwrap();
        let now = Instant::now();

        loop {
            if now.elapsed() > Duration::from_secs(10) {
                sender_caller.send_deadline("STOP".to_string(), Instant::now()).unwrap();
                break;
            }
            match receiver_caller.try_recv() {
                Ok(e) => {
                    match e.event_type {
                        EventType::Text(t) => {
                            println!("{}", Into::<String>::into(t))
                        }
                        _ => ()
                    }
                },
                Err(_) => (),
            }

        }
    }

    #[test]
    
    fn playback_video_test() {
        let mut kfn = Kfn::open("test/input.kfn");
    
        kfn.parse().unwrap();

        kfn.data.song.load_eff();

        kfn.play_kfn();
    }

    #[test]
    fn lyrics_test() {
        let mut kfn = Kfn::open("test/input.kfn");
        kfn.parse().unwrap();
        dbg!(kfn.get_texts_and_syncs());
    }
}
