//! Runtime entry point for Observe → Assess and Observe → Identify Need.

use goods_application::ports::observation_source::ObservationSource;
use goods_application::use_cases::assess_state::AssessState;
use goods_application::use_cases::identify_need::IdentifyNeed;
use goods_application::use_cases::observe_goods::ObserveGoods;
use goods_domain::goods::Goods;
use goods_domain::need::NeedAssessment;
use goods_domain::state::GoodsState;

/// Product-independent composition of the Phase 1 and Phase 2 use cases.
pub struct GoodsRuntime<S> {
    source: S,
}

impl<S> GoodsRuntime<S> {
    pub fn new(source: S) -> Self {
        Self { source }
    }
}

impl<S: ObservationSource> GoodsRuntime<S> {
    pub fn observe_and_assess(&self, goods: &Goods) -> Result<GoodsState, S::Error> {
        let observation = ObserveGoods::execute(&self.source, goods)?;
        Ok(AssessState::execute(goods, observation))
    }

    /// Observe once and derive both the Health Assessment and the Need
    /// Assessment from the same observation.
    pub fn observe_and_identify_needs(
        &self,
        goods: &Goods,
    ) -> Result<(GoodsState, NeedAssessment), S::Error> {
        let observation = ObserveGoods::execute(&self.source, goods)?;
        let needs = IdentifyNeed::execute(goods, &observation);
        let state = AssessState::execute(goods, observation);
        Ok((state, needs))
    }
}
