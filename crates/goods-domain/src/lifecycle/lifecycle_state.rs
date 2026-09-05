//! Lifecycle state of a Goods individual.

/// Whether a Goods individual is still monitored by Goods Garden (`Active`)
/// or has left it (`Retired`: sold out, disposed, discontinued, etc.).
///
/// Transitions only happen through an explicit call such as `Goods::retire`;
/// the domain never infers a transition from a CareAction or HumanFeedback's
/// free-text content.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum LifecycleState {
    Active,
    Retired,
}

impl LifecycleState {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Active => "active",
            Self::Retired => "retired",
        }
    }
}
