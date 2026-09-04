//! Urgency levels derived from a Deviation's magnitude.

/// An explainable, staged urgency level for a Need.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Urgency {
    Low,
    Medium,
    High,
}

impl Urgency {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Low => "low",
            Self::Medium => "medium",
            Self::High => "high",
        }
    }

    /// Fixed, example-only thresholds for the Freshness dimension (hours
    /// beyond the expected maximum). Not derived from real POS/SEJ data; see
    /// docs/phases/phase-2-need.md.
    pub fn from_freshness_magnitude(magnitude: i64) -> Option<Self> {
        match magnitude {
            m if m <= 0 => None,
            1..=2 => Some(Self::Low),
            3..=5 => Some(Self::Medium),
            _ => Some(Self::High),
        }
    }

    /// Fixed, example-only thresholds for the StockAvailability dimension
    /// (units below the minimum stock quantity). Not derived from real
    /// POS/SEJ data; see docs/phases/phase-2-need.md.
    pub fn from_stock_magnitude(magnitude: i64) -> Option<Self> {
        match magnitude {
            m if m <= 0 => None,
            1 => Some(Self::Low),
            2..=3 => Some(Self::Medium),
            _ => Some(Self::High),
        }
    }
}
