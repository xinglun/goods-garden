//! Care Request.

use crate::evidence::Evidence;
use crate::need::{GoodsNeed, NeedAssessment, NeedConflict};

/// An explainable request for Care, raised when a Need Assessment identifies
/// at least one Need. Requesting Care never decides or performs an action;
/// it only asks a human to review.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CareRequest {
    pub needs: Vec<GoodsNeed>,
    pub conflict: Option<NeedConflict>,
    pub requested_role: String,
    pub evidence: Evidence,
}

impl CareRequest {
    /// Build a CareRequest from a Need Assessment, or `None` when there is no
    /// Need to raise.
    pub fn from_assessment(assessment: &NeedAssessment) -> Option<Self> {
        if assessment.needs.is_empty() {
            return None;
        }

        let requested_role = "store staff".to_owned();
        let explanation = if assessment.conflict.is_some() {
            format!(
                "{} Need(s) identified, including a Need Conflict that requires a human decision.",
                assessment.needs.len()
            )
        } else {
            format!(
                "{} Need(s) identified; requesting store staff to review.",
                assessment.needs.len()
            )
        };

        Some(Self {
            needs: assessment.needs.clone(),
            conflict: assessment.conflict.clone(),
            requested_role,
            evidence: Evidence::known(explanation),
        })
    }
}
