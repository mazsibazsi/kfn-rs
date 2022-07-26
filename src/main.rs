mod kfn_io;

use kfn_io::KfnFile;

fn main() {
    let mut kfn = KfnFile::new("shika.kfn");
    dbg!(kfn.parse().unwrap());
    kfn.extract_all();
}
