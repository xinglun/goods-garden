//! Observe Goods use case.

use goods_domain::goods::Goods;
use goods_domain::observation::Observation;

use crate::ports::observation_source::ObservationSource;

/// Application boundary for obtaining a sensory input.
pub struct ObserveGoods;

impl ObserveGoods {
    pub fn execute<S: ObservationSource>(
        source: &S,
        goods: &Goods,
    ) -> Result<Observation, S::Error> {
        source.observe(goods)
    }
}
