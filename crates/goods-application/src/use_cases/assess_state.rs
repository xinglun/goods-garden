//! Assess State use case.

use goods_domain::goods::Goods;
use goods_domain::observation::Observation;
use goods_domain::state::GoodsState;

/// Application boundary for deriving a bounded Goods State.
pub struct AssessState;

impl AssessState {
    pub fn execute(goods: &Goods, observation: Observation) -> GoodsState {
        GoodsState::assess(goods, observation)
    }
}
