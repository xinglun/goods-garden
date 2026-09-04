//! Observation value object.

/// A sensory input to the living-goods model.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Observation {
    pub source: String,
    pub observed_at: String,
    pub age_hours: u32,
    pub quantity_on_hand: u32,
}
