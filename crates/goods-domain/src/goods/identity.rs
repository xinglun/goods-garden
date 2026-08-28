//! Goods identity.

/// Identity of an individual good.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GoodsIdentity {
    pub species: String,
    pub individual_id: String,
}
