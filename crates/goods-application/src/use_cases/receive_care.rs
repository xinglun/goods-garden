//! Receive Care use case.

use goods_domain::care::{CareAction, CareRequest, HumanFeedback};

/// Application boundary for recording a Care Action from Human Feedback.
pub struct ReceiveCare;

impl ReceiveCare {
    pub fn execute(request: CareRequest, feedback: HumanFeedback) -> CareAction {
        CareAction::record(request, feedback)
    }
}
