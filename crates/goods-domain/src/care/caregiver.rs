//! Caregiver.

/// A human participant who can understand, decide and provide Care.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Caregiver {
    pub role: String,
    pub display_name: String,
}
