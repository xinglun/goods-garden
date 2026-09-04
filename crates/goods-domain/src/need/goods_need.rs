//! Goods Need.

use super::deviation::{Deviation, DeviationDimension};
use super::urgency::Urgency;

/// The kind of concern a Need explains. Names describe the concerning
/// condition, not a recommended Care response; Care remains out of scope.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum NeedKind {
    FreshnessConcern,
    StockAvailabilityConcern,
}

/// An explainable condition that calls for care or investigation, derived
/// from a concerning Deviation.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GoodsNeed {
    pub kind: NeedKind,
    pub urgency: Urgency,
    pub deviation: Deviation,
    pub explanation: String,
}

impl GoodsNeed {
    /// Build a GoodsNeed from a Deviation, or return `None` when the
    /// Deviation is not concerning (no Need exists).
    pub fn from_deviation(deviation: Deviation) -> Option<Self> {
        if !deviation.is_concerning() {
            return None;
        }

        let (kind, urgency) = match deviation.dimension {
            DeviationDimension::Freshness => (
                NeedKind::FreshnessConcern,
                Urgency::from_freshness_magnitude(deviation.magnitude)?,
            ),
            DeviationDimension::StockAvailability => (
                NeedKind::StockAvailabilityConcern,
                Urgency::from_stock_magnitude(deviation.magnitude)?,
            ),
        };

        let explanation = deviation.explanation.clone();

        Some(Self { kind, urgency, deviation, explanation })
    }
}
