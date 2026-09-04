//! Need Assessment: the Phase 2 counterpart to Health Assessment.

use crate::goods::Goods;
use crate::observation::Observation;

use super::deviation::Deviation;
use super::goods_need::GoodsNeed;
use super::need_conflict::NeedConflict;

/// The bounded set of Needs, and any Need Conflict between them, identified
/// from a single observation.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct NeedAssessment {
    pub needs: Vec<GoodsNeed>,
    pub conflict: Option<NeedConflict>,
}

impl NeedAssessment {
    pub fn identify(goods: &Goods, observation: &Observation) -> Self {
        let expectation = goods.expectation();

        let mut needs = Vec::new();
        if let Some(need) =
            GoodsNeed::from_deviation(Deviation::freshness(observation, &expectation))
        {
            needs.push(need);
        }
        if let Some(need) =
            GoodsNeed::from_deviation(Deviation::stock_availability(observation, &goods.profile))
        {
            needs.push(need);
        }

        let conflict = NeedConflict::detect(&needs);

        Self { needs, conflict }
    }
}
