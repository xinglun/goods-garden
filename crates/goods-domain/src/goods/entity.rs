//! Goods aggregate root.

use crate::lifecycle::LifecycleState;
use crate::state::goods_state::Expectation;

use super::{GoodsIdentity, GoodsProfile};

/// A product living in a business context.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Goods {
    pub identity: GoodsIdentity,
    pub profile: GoodsProfile,
    pub lifecycle: LifecycleState,
}

impl Goods {
    pub fn new(identity: GoodsIdentity, profile: GoodsProfile) -> Self {
        Self { identity, profile, lifecycle: LifecycleState::Active }
    }

    pub fn expectation(&self) -> Expectation {
        Expectation { max_age_hours: self.profile.expected_lifetime_hours }
    }

    /// Return a copy of this Goods with its lifecycle marked Retired. This
    /// is the only way to transition a Goods' lifecycle; the domain never
    /// infers it from a CareAction or HumanFeedback's free-text content.
    pub fn retire(&self) -> Self {
        Self { lifecycle: LifecycleState::Retired, ..self.clone() }
    }
}
