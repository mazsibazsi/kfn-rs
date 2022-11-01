use kfn_rs::Kfn;
use kfn_rs::kfn_player::KfnPlayer;

fn main() {
    let mut kfn = Kfn::open("test/input.kfn");
    kfn.parse().unwrap();
    kfn.data.song.load_eff();

    //dbg!(kfn.get_animation_events());

    kfn.play_kfn();
}