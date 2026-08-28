//! Goods profile.

/// Stable descriptive profile of a good.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GoodsProfile {
    pub display_name: String,
    pub expected_lifetime_hours: u32,
}
