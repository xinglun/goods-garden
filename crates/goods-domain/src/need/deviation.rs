//! Deviation value object.

use crate::goods::GoodsProfile;
use crate::observation::Observation;
use crate::state::Expectation;

/// The evaluated dimension a Deviation was derived from.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DeviationDimension {
    /// How far an observed age is beyond the profile's expected lifetime.
    Freshness,
    /// How far an observed quantity is below the profile's minimum stock.
    StockAvailability,
}

/// A quantified gap between an observation and its expectation.
///
/// `magnitude` is positive when the observation is concerning (beyond
/// expectation) and zero or negative when it is within expectation. The unit
/// depends on the dimension: hours for `Freshness`, count for
/// `StockAvailability`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Deviation {
    pub dimension: DeviationDimension,
    pub magnitude: i64,
    pub explanation: String,
}

impl Deviation {
    /// Whether this Deviation is large enough to call for a Need.
    pub fn is_concerning(&self) -> bool {
        self.magnitude > 0
    }

    /// Derive the Freshness Deviation from an age observation and its
    /// profile-derived Expectation.
    pub fn freshness(observation: &Observation, expectation: &Expectation) -> Self {
        let magnitude = i64::from(observation.age_hours) - i64::from(expectation.max_age_hours);
        let explanation = if magnitude > 0 {
            format!(
                "observed age ({} hours) exceeds the expected maximum ({} hours) by {} hours",
                observation.age_hours, expectation.max_age_hours, magnitude
            )
        } else {
            format!(
                "observed age ({} hours) is within the expected maximum ({} hours)",
                observation.age_hours, expectation.max_age_hours
            )
        };

        Self { dimension: DeviationDimension::Freshness, magnitude, explanation }
    }

    /// Derive the StockAvailability Deviation from a quantity observation and
    /// the profile's minimum stock quantity.
    pub fn stock_availability(observation: &Observation, profile: &GoodsProfile) -> Self {
        let magnitude =
            i64::from(profile.minimum_stock_quantity) - i64::from(observation.quantity_on_hand);
        let explanation = if magnitude > 0 {
            format!(
                "observed quantity on hand ({}) is below the minimum stock quantity ({}) by {}",
                observation.quantity_on_hand, profile.minimum_stock_quantity, magnitude
            )
        } else {
            format!(
                "observed quantity on hand ({}) meets the minimum stock quantity ({})",
                observation.quantity_on_hand, profile.minimum_stock_quantity
            )
        };

        Self { dimension: DeviationDimension::StockAvailability, magnitude, explanation }
    }
}
