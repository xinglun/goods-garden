//! Request Care use case.

use goods_domain::care::CareRequest;
use goods_domain::need::NeedAssessment;

/// Application boundary for raising a bounded Care Request.
pub struct RequestCare;

impl RequestCare {
    pub fn execute(assessment: &NeedAssessment) -> Option<CareRequest> {
        CareRequest::from_assessment(assessment)
    }
}
