//! Observation source port.

use goods_domain::goods::Goods;
use goods_domain::observation::Observation;

/// Port for receiving observations.
pub trait ObservationSource {
    type Error;

    fn observe(&self, goods: &Goods) -> Result<Observation, Self::Error>;
}
