//! Learning.

use crate::outcome::{Outcome, OutcomeStatus};

/// A reviewable statement derived from a verified Outcome.
///
/// Learning never adjusts a threshold, profile field or any other rule by
/// itself; it only records a plain-language, human-reviewable observation.
/// Whether to act on it is a separate human decision, outside this crate's
/// implemented behavior.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Learning {
    pub outcome: Outcome,
    pub statement: String,
}

impl Learning {
    pub fn from_outcome(outcome: Outcome) -> Self {
        let statement = match outcome.status {
            OutcomeStatus::Resolved => {
                "This Care Action appears to have resolved the Need(s) it addressed, based on a \
                 single follow-up observation. This is a reviewable observation, not a rule \
                 change."
                    .to_owned()
            }
            OutcomeStatus::Unresolved => {
                "This Care Action did not resolve the Need(s) it addressed, based on a single \
                 follow-up observation. This is a reviewable observation, not a rule change."
                    .to_owned()
            }
        };

        Self { outcome, statement }
    }
}
