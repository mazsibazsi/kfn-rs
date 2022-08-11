
#[derive(Debug, Clone)]
pub struct Eff {
    pub id: usize,
    pub anims: Vec<Anim>,
    pub syncs: Vec<usize>,
    pub texts: Vec<String>,
    pub trajectory: Trajectory,
}

impl Eff {

}

#[derive(Debug, Clone, Default)]
pub struct Anim {
    pub time: usize,
    pub anim_entries: Vec<AnimEntry>,
}

#[derive(Debug, Clone, Default)]
pub struct AnimEntry {
    pub action: Action,
    pub effect: Option<Effect>,
    pub trans_time: f64,
    pub trans_type: TransType,
}

/// Representation of an animation action.
#[derive(Debug, Clone, Default)]
pub enum Action {
    #[default]
    None,
    ChgBgImg(String),
    ChgColColor(String),
    ChgColImageColor(String),
    ChgAlphaBlending(String),
    ChgFloatOffsetX(f64),
    ChgFloatOffsetY(f64),
    ChgFloatDepth(f64),
    ChgTrajectory(String),
}

impl From<&str> for Action {

    fn from(s: &str) -> Self {

        let colon_split = s.split(':').collect::<Vec<&str>>();
        let equal_split = s.split('=').collect::<Vec<&str>>();

        let key = colon_split[0];
        let value = equal_split[1].to_string();

        // else do this
        match key {
            "ChgBgImg"          => Action::ChgBgImg(value),
            "ChgColColor"       => Action::ChgColColor(value),
            "ChgColImageColor"  => Action::ChgColImageColor(value),
            "ChgAlphaBlending"  => Action::ChgAlphaBlending(value),
            "ChgFloatOffsetX"   => Action::ChgFloatOffsetX(value.parse::<f64>().unwrap()),
            "ChgFloatOffsetY"   => Action::ChgFloatOffsetY(value.parse::<f64>().unwrap()),
            "ChgFloatDepth"     => Action::ChgFloatDepth(value.parse::<f64>().unwrap()),
            "ChgTrajectory"     => Action::ChgTrajectory(value),
            &_                  => Action::None,
        }
    }
}

impl ToString for Action {
    fn to_string(&self) -> String {
        match self {
            Action::ChgBgImg(val) => {
                let mut ret = String::from("ChgBgImg:LibImage=");
                ret.push_str(val.as_str());
                ret
            },
            _ => " ".to_owned() 
        }
    }
}

#[derive(Debug, Clone, Default)]
pub enum Effect {
    #[default]
    None,
    AlphaBlending,
    MoveRight,
    MoveLeft,
    MoveTop,
    MoveBottom,
    // TODO the rest of the effects
}

impl From<&str> for Effect {

    fn from(s: &str) -> Self {
        match s {
            "AlphaBlending" => Effect::AlphaBlending,
            "MoveRight"     => Effect::MoveRight,
            "MoveLeft"      => Effect::MoveLeft,
            "MoveTop"       => Effect::MoveTop,
            "MoveBottom"    => Effect::MoveBottom,
            &_              => Effect::None,
        }
    }
}

#[derive(Debug, Clone, Default)]
pub enum TransType {
    #[default]
    None,
    Linear,
    Smooth,
    Falling,
    FallingBouncing,
    Bend1,
    Bend3,
    Bend5,
    Bounce1,
    Bounce3,
    Bounce5,
}

impl From<&str> for TransType {

    fn from(s: &str) -> Self {
        match s {
            "Linear"            => TransType::Linear,
            "Smooth"            => TransType::Smooth,
            "Falling"           => TransType::Falling,
            "FallingBouncing"   => TransType::FallingBouncing,
            "Bend1"             => TransType::Bend1,
            "Bend3"             => TransType::Bend3,
            "Bend5"             => TransType::Bend5,
            "Bounce1"           => TransType::Bounce1,
            "Bounce3"           => TransType::Bounce3,
            "Bounce5"           => TransType::Bounce5,
            &_                  => TransType::None,
        }
    }
}


#[derive(Debug, Clone)]
pub enum Trajectory {
    PlainBottomToTop(u32, u32, u32, u32),
}

impl std::default::Default for Trajectory {
    fn default() -> Self {
        Trajectory::PlainBottomToTop(1, 1, 1, 1)
    }
}

impl From<&str> for Trajectory {

    fn from(s: &str) -> Self {
        let key = s.split('*').collect::<Vec<&str>>()[0];
        let value = s.split('*').collect::<Vec<&str>>();
        dbg!(&value);
        match key {
            "PlainBottomToTop" => Trajectory::PlainBottomToTop(
                                        value[1].parse().unwrap(),
                                        value[2].parse().unwrap(),
                                        value[3].parse().unwrap(),
                                        value[4].parse().unwrap()
                                    ),
            &_ => Trajectory::PlainBottomToTop(1, 1, 1, 1),
        }
    }
}
