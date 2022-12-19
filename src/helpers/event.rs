use crate::kfn_ini::eff::{AnimEntry, TextEntry};

/// Representing the states, that the player has to implement and display.
#[derive(Debug, Clone, Default)]
pub struct Event {
    pub event_type: EventType,
    pub time: usize,
}


#[derive(Debug, Clone, Default)]
pub enum EventType {
    Background(AnimEntry),
    Animation(AnimEntry),
    Text(TextEntry),
    #[default]
    None,
}
