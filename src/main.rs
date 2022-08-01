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
    let mut kfn = KfnFile::read("ichido.kfn");
    kfn.dump().unwrap();


    //kfn.data.add_entry_from_file("beyond_the_time.jpg");
    let mut data: Vec<u8> = Vec::new();
    data.append(&mut kfn.header.to_binary());
    data.append(&mut kfn.data.to_binary());
    if let Some(songs_ini) = kfn.data.get_songs_ini() {
        fs::write("Songs.ini", songs_ini.file_bin).unwrap();
    }
    fs::write("asd.mp3", kfn.data.entries[10].clone().file_bin).unwrap();
    
}
