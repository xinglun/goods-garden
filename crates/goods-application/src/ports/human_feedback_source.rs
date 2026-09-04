//! Human Feedback source port.

use goods_domain::care::{CareRequest, HumanFeedback};

/// Port for receiving Human Feedback in response to a CareRequest.
pub trait HumanFeedbackSource {
    type Error;

    fn provide_feedback(&self, request: &CareRequest) -> Result<HumanFeedback, Self::Error>;
}
