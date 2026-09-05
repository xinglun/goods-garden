//! Verify Outcome use case.

use goods_domain::care::CareAction;
use goods_domain::need::NeedAssessment;
use goods_domain::outcome::Outcome;
use goods_domain::state::GoodsState;

/// Application boundary for verifying a CareAction against a follow-up
/// observation.
pub struct VerifyOutcome;

impl VerifyOutcome {
    pub fn execute(
        action: CareAction,
        new_state: GoodsState,
        new_needs: NeedAssessment,
    ) -> Outcome {
        Outcome::verify(action, new_state, new_needs)
    }
}
