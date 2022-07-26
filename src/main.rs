mod kfn_io;

use kfn_io::KfnFile;

fn main() {
    let mut kfn = KfnFile::read("shika.kfn");
    dbg!(kfn.parse());
}
