//! Human Feedback.

use super::caregiver::Caregiver;

/// Human input received in response to a CareRequest.
///
/// This is external Human Input, not a value the domain or application layer
/// computes: the Trust Model requires evidence over fluency, so the domain
/// never invents, infers or synthesizes what a Caregiver decided.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct HumanFeedback {
    pub caregiver: Caregiver,
    pub decision: String,
    pub provided_at: String,
}
