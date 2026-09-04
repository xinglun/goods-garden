//! Need Conflict.

use super::goods_need::{GoodsNeed, NeedKind};

/// An explainable contradiction between two simultaneous Needs that
/// recommend opposing directions of attention. NeedConflict does not resolve
/// the contradiction and does not recommend either direction; that remains a
/// Care decision, out of scope for Phase 2.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct NeedConflict {
    pub freshness_need: GoodsNeed,
    pub availability_need: GoodsNeed,
    pub explanation: String,
}

impl NeedConflict {
    /// Detect a conflict when a FreshnessConcern and a
    /// StockAvailabilityConcern Need are both present. This is the only
    /// conflict pattern Phase 2 defines; additional dimensions may need
    /// additional patterns in a later phase.
    pub fn detect(needs: &[GoodsNeed]) -> Option<Self> {
        let freshness_need =
            needs.iter().find(|need| need.kind == NeedKind::FreshnessConcern)?.clone();
        let availability_need =
            needs.iter().find(|need| need.kind == NeedKind::StockAvailabilityConcern)?.clone();

        let explanation = "This good has two Needs that recommend opposite directions: the \
            freshness Need suggests removing it from the shelf, while the stock-availability \
            Need suggests keeping it stocked and replenishing it. Goods Garden surfaces this \
            Need Conflict as an explainable contradiction; it does not resolve it automatically \
            and does not recommend either action."
            .to_owned();

        Some(Self { freshness_need, availability_need, explanation })
    }
}
