mod kfn_io;

use std::env;
use kfn_io::KfnFile;

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
    let mut kfn = KfnFile::new("shika.kfn");
    kfn.dump().unwrap();
    dbg!(kfn.kfn_data.syncs.len(), kfn.kfn_data.text.len());
}
