pub mod user_interactions {
    
    use std::time::Duration;
    use crate::kfn_player::KfnPlayer;

    impl KfnPlayer {
            /// Function for pausing and resuming the sink thread.
    pub fn play_pause(&mut self) {
        if self.paused {
            self.time.start_time = std::time::Instant::now();
            self.sender.send("RESUME".to_string()).unwrap();
            self.paused = false;
            println!("KFN-PLAYER: RESUME signal sent.")
        } else {
            self.time.offset = self.time.start_time.elapsed() + self.time.offset;
            self.sender.send("PAUSE".to_string()).unwrap();
            self.paused = true;
            println!("KFN-PLAYER: PAUSE signal sent.")
        }
    }

    /// Sends a CH_TRACK signal to the backend to change to the Vocal/Off-Vocal track.
    pub fn change_track(&mut self) {
        self.sender.send("CH_TRACK".to_string()).unwrap();
    }

    pub fn forward(&mut self) {
        self.sender.send("FW".to_string()).unwrap();
        self.time.offset += Duration::from_secs(5);
    }
    pub fn backward(&mut self) {
        self.sender.send("BW".to_string()).unwrap();
        self.time.offset -= Duration::from_secs(5);
    }
    }
}