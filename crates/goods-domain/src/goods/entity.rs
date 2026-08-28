//! Goods aggregate root.

use crate::state::goods_state::Expectation;

use super::{GoodsIdentity, GoodsProfile};

/// A product living in a business context.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Goods {
    pub identity: GoodsIdentity,
    pub profile: GoodsProfile,
}

impl Goods {
    pub fn new(identity: GoodsIdentity, profile: GoodsProfile) -> Self {
        Self { identity, profile }
    }

    pub fn expectation(&self) -> Expectation {
        Expectation { max_age_hours: self.profile.expected_lifetime_hours }
    }
}
