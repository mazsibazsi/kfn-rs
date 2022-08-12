
/// Representation of the trajectories the text or image can take.
#[derive(Debug, Clone)]
pub enum Trajectory {
    PlainBottomToTop    (f64, f64, f64, f64),
    PlainTopToBottom    (f64, f64, f64, f64),
    BottomLeftToTopRight(f64, f64, f64, f64),
    BottomRightToTopLeft(f64, f64, f64, f64),
    TopRightToBottomLeft(f64, f64, f64, f64),
    TopLeftToBottomRight(f64, f64, f64, f64),
    Still               (f64, f64, f64, f64),
    StarWars            (f64, f64, f64, f64),
    MadCircles          (f64, f64, f64, f64),
    BackToFront1        (f64, f64, f64, f64),
    BackToFront2        (f64, f64, f64, f64),
}

impl std::default::Default for Trajectory {
    fn default() -> Self {
        Trajectory::PlainBottomToTop(1.0, 1.0, 1.0, 1.0)
    }
}

impl Trajectory {
    fn concatenate_values(&self, total_time: f64, width: f64, height: f64, depth: f64) -> String {

        let mut value = String::from("");
        value.push('*');
        value.push_str(&total_time.to_string());

        value.push('*');
        value.push_str(&width.to_string());

        value.push('*');
        value.push_str(&height.to_string());

        value.push('*');
        value.push_str(&depth.to_string());

        value
        
    }
}

impl From<&str> for Trajectory {

    fn from(s: &str) -> Self {
        let key          = s.split('*').collect::<Vec<&str>>()[0];
        let value   = s.split('*').collect::<Vec<&str>>();

        dbg!(&key, &value);

        let mut total_time  : f64 = 1.5;
        let mut width       : f64 = 1.0;
        let mut height      : f64 = 1.0;
        let mut depth       : f64 = 1.0;

        if !key.is_empty() {
            total_time  = value[1].parse().unwrap();
            width       = value[2].parse().unwrap();
            height      = value[3].parse().unwrap();
            depth       = value[4].parse().unwrap();
        }

        dbg!(&value);
        match key {
            "PlainBottomToTop"      => Trajectory::PlainBottomToTop     (total_time, width, height, depth),
            "PlainTopToBottom"      => Trajectory::PlainTopToBottom     (total_time, width, height, depth),
            "BottomLeftToTopRight"  => Trajectory::BottomLeftToTopRight (total_time, width, height, depth),
            "BottomRightToTopLeft"  => Trajectory::BottomRightToTopLeft (total_time, width, height, depth),
            "TopRightToBottomLeft"  => Trajectory::TopRightToBottomLeft (total_time, width, height, depth),
            "TopLeftToBottomRight"  => Trajectory::TopLeftToBottomRight (total_time, width, height, depth),
            "Still"                 => Trajectory::Still                (total_time, width, height, depth),
            "StarWars"              => Trajectory::StarWars             (total_time, width, height, depth),
            "MadCircles"            => Trajectory::MadCircles           (total_time, width, height, depth),
            "BackToFront1"          => Trajectory::BackToFront1         (total_time, width, height, depth),
            "BackToFront2"          => Trajectory::BackToFront2         (total_time, width, height, depth),
            &_                      => Trajectory::PlainBottomToTop     (total_time, width, height, depth),
        }
    }
}
 
impl ToString for Trajectory {
    fn to_string(&self) -> String {
        match self {
            &Trajectory::PlainBottomToTop(total_time,width, height, depth) => {
                let mut value = String::from("PlainBottomToTop");
                value.push_str(&self.concatenate_values(total_time, width, height, depth));
                value
            }
            &Trajectory::BottomLeftToTopRight(total_time,width, height, depth) => {
                let mut value = String::from("BottomLeftToTopRight");
                value.push_str(&self.concatenate_values(total_time, width, height, depth));
                value
            }
            &Trajectory::BottomRightToTopLeft(total_time,width, height, depth) => {
                let mut value = String::from("BottomRightToTopLeft");
                value.push_str(&self.concatenate_values(total_time, width, height, depth));
                value
            }
            &Trajectory::TopRightToBottomLeft(total_time,width, height, depth) => {
                let mut value = String::from("TopRightToBottomLeft");
                value.push_str(&self.concatenate_values(total_time, width, height, depth));
                value
            }
            &Trajectory::TopLeftToBottomRight(total_time,width, height, depth) => {
                let mut value = String::from("TopLeftToBottomRight");
                value.push_str(&self.concatenate_values(total_time, width, height, depth));
                value
            }
            &Trajectory::Still(total_time,width, height, depth) => {
                let mut value = String::from("Still");
                value.push_str(&self.concatenate_values(total_time, width, height, depth));
                value
            }
            &Trajectory::StarWars(total_time,width, height, depth) => {
                let mut value = String::from("StarWars");
                value.push_str(&self.concatenate_values(total_time, width, height, depth));
                value
            }
            &Trajectory::MadCircles(total_time,width, height, depth) => {
                let mut value = String::from("MadCircles");
                value.push_str(&self.concatenate_values(total_time, width, height, depth));
                value
            }
            &Trajectory::BackToFront1(total_time,width, height, depth) => {
                let mut value = String::from("BackToFront1");
                value.push_str(&self.concatenate_values(total_time, width, height, depth));
                value
            }
            &Trajectory::BackToFront2(total_time,width, height, depth) => {
                let mut value = String::from("BackToFront2");
                value.push_str(&self.concatenate_values(total_time, width, height, depth));
                value
            }
            &_ => {
                "PlainBottomToTop*1*1*1*1".to_owned()
            }           
        }
    }
}

