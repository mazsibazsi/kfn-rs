mod kfn_io;

use kfn_io::KfnFile;

fn main() {
    let mut kfn = KfnFile::new("shika.kfn");
    kfn.dump().unwrap();
    dbg!(kfn.kfn_data);
}
