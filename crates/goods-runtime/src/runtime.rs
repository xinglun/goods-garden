//! Runtime entry point for Observe → Assess.

use goods_application::ports::observation_source::ObservationSource;
use goods_application::use_cases::assess_state::AssessState;
use goods_application::use_cases::observe_goods::ObserveGoods;
use goods_domain::goods::Goods;
use goods_domain::state::GoodsState;

/// Product-independent composition of the Phase 1 use cases.
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
}
