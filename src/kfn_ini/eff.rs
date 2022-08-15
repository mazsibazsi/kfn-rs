use crate::kfn_ini::Trajectory;

/// Representation of an Eff# headed section, which contains animations, texts, and sync data.
#[derive(Debug, Clone)]
pub struct Eff {
    /// The ID of the Eff# layer.
    /// Te background layer's ID is always 51.
    /// Every following starts from 1.
    pub id: usize,
    /// Collection of the animations.
    pub anims: Vec<Anim>,
    /// Collection of the sync timestamps in ms.
    pub syncs: Vec<usize>,
    /// Collection of the songtext lines. Separators: '/' ' '
    pub texts: Vec<String>,
    /// Initial trajectory of the layer.
    pub initial_trajectory: Trajectory,
}

impl Eff {

}

/// Representation of a collection of animations executed at the same time.
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
    ChgTrajectory(Trajectory),
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
            "ChgTrajectory"     => Action::ChgTrajectory(Trajectory::from(value.as_str())),
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

/// Representation of the available visual effects.
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

/// Representation of the various transition types.
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


