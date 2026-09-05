//! Learn from Outcome use case.

use goods_domain::learning::Learning;
use goods_domain::outcome::Outcome;

/// Application boundary for deriving a reviewable Learning statement from a
/// verified Outcome.
pub struct LearnFromOutcome;

impl LearnFromOutcome {
    pub fn execute(outcome: Outcome) -> Learning {
        Learning::from_outcome(outcome)
    }
}
