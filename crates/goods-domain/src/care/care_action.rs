//! Care Action.

use super::care_request::CareRequest;
use super::human_feedback::HumanFeedback;
use crate::evidence::Evidence;

/// A traceable record binding a CareRequest to the Human Feedback that
/// resolved it.
///
/// The domain does not invent, infer or evaluate the decision content; it
/// only records what the Caregiver said.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CareAction {
    pub request: CareRequest,
    pub feedback: HumanFeedback,
    pub evidence: Evidence,
}

impl CareAction {
    pub fn record(request: CareRequest, feedback: HumanFeedback) -> Self {
        let explanation = format!(
            "{} ({}) responded: {}",
            feedback.caregiver.display_name, feedback.caregiver.role, feedback.decision
        );

        Self { request, feedback, evidence: Evidence::known(explanation) }
    }
}
