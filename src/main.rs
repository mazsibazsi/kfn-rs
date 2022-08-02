mod kfn_io;

use std::fs;

//use std::env;
use kfn_io::KfnFile;
use kfn_io::helpers::ToBinary;


fn main() {
    /* let mut file = String::from("None");
    let args: Vec<String> = env::args().collect();
    match args[1].as_str() {
        "-f" => {
            if args.len() > 2 && args[2].contains(".kfn") {
                file = String::from(args[2].clone());
            }
        },
        _ => ()
    }
    dbg!(args);
    if file == "None" {
        return;
    }*/
    let mut kfn = KfnFile::read("brave.kfn");
    kfn.dump().unwrap();

    //kfn.data.remove_entry_by_id(1);
    //kfn.data.add_entry_from_file("beyond_the_time.jpg");
    dbg!(&kfn.data.path_song_ini);
    kfn.data.adjust_dir_offset();
    let mut data: Vec<u8> = Vec::new();
    data.append(&mut kfn.header.to_binary());
    
    data.append(&mut kfn.data.to_binary());
    fs::write("output.kfn", data).unwrap();
    fs::write("song.ini", kfn.data.get_songs_ini().unwrap().file_bin).unwrap();
}
