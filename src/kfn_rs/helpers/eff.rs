
#[derive(Debug, Clone)]
pub struct Eff {
    anims: Vec<Anim>,
}

#[derive(Debug, Clone, Default)]
pub struct Anim {
    trans_time: usize,
    trans_type: TransType,
    action: Action,
    effect: Effect,
}

#[derive(Debug, Clone, Default)]
enum Action {
    #[default]
    None,
    ChgBgImg,
    ChgColColor,
    ChgColImageColor,
    ChgAlphaBlending,
    ChgFloatOffsetX,
    ChgFloatOffsetY,
    ChgFloatDepth,
}

#[derive(Debug, Clone, Default)]
enum Effect {
    #[default]
    AlphaBlending,
    MoveRight,
    MoveLeft,
    MoveTop,
    MoveBottom,
    // TODO the rest of the effects
}

#[derive(Debug, Clone, Default)]
enum TransType {
    #[default]
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
