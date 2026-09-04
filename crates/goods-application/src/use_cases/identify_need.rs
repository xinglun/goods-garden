//! Identify Need use case.

use goods_domain::goods::Goods;
use goods_domain::need::NeedAssessment;
use goods_domain::observation::Observation;

/// Application boundary for deriving a bounded Need Assessment.
pub struct IdentifyNeed;

impl IdentifyNeed {
    pub fn execute(goods: &Goods, observation: &Observation) -> NeedAssessment {
        NeedAssessment::identify(goods, observation)
    }
}
