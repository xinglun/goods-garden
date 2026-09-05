//! Outcome.

use crate::care::CareAction;
use crate::evidence::Evidence;
use crate::need::NeedAssessment;
use crate::state::GoodsState;

/// Whether the NeedKind(s) a CareAction addressed are still present in a
/// follow-up observation.
///
/// This is a factual comparison, not a judgment of whether the Caregiver's
/// decision was good or sufficient.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum OutcomeStatus {
    Resolved,
    Unresolved,
}

/// The result of comparing a remembered CareAction against a follow-up
/// observation: does the Need it addressed still show up in the new Need
/// Assessment?
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Outcome {
    pub action: CareAction,
    pub new_state: GoodsState,
    pub new_needs: NeedAssessment,
    pub status: OutcomeStatus,
    pub evidence: Evidence,
}

impl Outcome {
    /// Verify a CareAction against a follow-up State and Need Assessment.
    pub fn verify(action: CareAction, new_state: GoodsState, new_needs: NeedAssessment) -> Self {
        let addressed_kinds: Vec<_> = action.request.needs.iter().map(|need| need.kind).collect();
        let still_present = new_needs.needs.iter().any(|need| addressed_kinds.contains(&need.kind));

        let status =
            if still_present { OutcomeStatus::Unresolved } else { OutcomeStatus::Resolved };
        let explanation = match status {
            OutcomeStatus::Resolved => {
                "None of the Need(s) this Care Action addressed are present in the follow-up \
                 observation."
                    .to_owned()
            }
            OutcomeStatus::Unresolved => {
                "At least one of the Need(s) this Care Action addressed is still present in the \
                 follow-up observation."
                    .to_owned()
            }
        };

        Self { action, new_state, new_needs, status, evidence: Evidence::known(explanation) }
    }
}
