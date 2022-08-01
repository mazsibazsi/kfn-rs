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
    dbg!(kfn.data.syncs.len(), kfn.data.text.len());
    let mut data: Vec<u8> = Vec::new();
    data.append(&mut kfn.header.to_binary());

    //kfn.data.add_entry_from_file("beyond_the_time.jpg");
    
    data.append(&mut kfn.data.to_binary());
    fs::write("output.kfn", data).unwrap();


}
