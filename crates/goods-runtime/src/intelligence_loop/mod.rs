//! Intelligence Loop: one cycle of Observe → Assess → Identify Needs →
//! Request Care → Remember, plus (when a prior cycle left a pending Care
//! Action) Verify → Learn against this cycle's observation.

use std::error::Error as StdError;

use goods_application::ports::human_feedback_source::HumanFeedbackSource;
use goods_application::ports::observation_source::ObservationSource;
use goods_domain::care::{CareAction, CareRequest};
use goods_domain::goods::Goods;
use goods_domain::learning::Learning;
use goods_domain::memory::GoodsMemory;
use goods_domain::need::NeedAssessment;
use goods_domain::state::GoodsState;

use crate::GoodsRuntime;

/// The result of running one Intelligence Loop cycle.
pub struct IntelligenceCycleOutcome {
    pub state: GoodsState,
    pub needs: NeedAssessment,
    pub request: Option<CareRequest>,
    pub action: Option<CareAction>,
    pub verification: Option<Learning>,
}

impl<S: ObservationSource> GoodsRuntime<S> {
    /// Run one Intelligence Loop cycle: if `pending_action` is `Some`,
    /// verify and learn from it against this cycle's observation first;
    /// then observe again, identify Need, and — when a Need exists —
    /// request and remember Care. This is exactly the sequence
    /// `apps/goods-garden-cli`'s Seven Day Life milestone orchestrated by
    /// hand; it adds no new business rule.
    pub fn run_cycle<F>(
        &self,
        goods: &Goods,
        feedback_source: &F,
        memory: &mut GoodsMemory,
        pending_action: Option<CareAction>,
    ) -> Result<IntelligenceCycleOutcome, Box<dyn StdError>>
    where
        F: HumanFeedbackSource,
        F::Error: StdError + 'static,
        S::Error: StdError + 'static,
    {
        let verification = match pending_action {
            Some(action) => Some(self.verify_and_learn(goods, action)?),
            None => None,
        };
        let (state, needs, request, action) =
            self.request_care_and_remember(goods, feedback_source, memory)?;
        Ok(IntelligenceCycleOutcome { state, needs, request, action, verification })
    }
}
