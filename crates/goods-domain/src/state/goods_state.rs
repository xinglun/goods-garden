//! Current Goods State and its Phase 1 health assessment.

use crate::goods::Goods;
use crate::goods::GoodsIdentity;
use crate::observation::Observation;

/// The expectation used by this Phase 1 demo.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Expectation {
    pub max_age_hours: u32,
}

/// The bounded health result produced from an observation.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum HealthStatus {
    Healthy,
    Unhealthy,
}

impl HealthStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Healthy => "healthy",
            Self::Unhealthy => "unhealthy",
        }
    }
}

/// An explainable Phase 1 health result.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct HealthAssessment {
    pub status: HealthStatus,
    pub explanation: String,
}

/// Domain representation of a good's current state.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GoodsState {
    pub identity: GoodsIdentity,
    pub observation: Observation,
    pub expectation: Expectation,
    pub health: HealthAssessment,
}

impl GoodsState {
    pub fn assess(goods: &Goods, observation: Observation) -> Self {
        let expectation = goods.expectation();
        let status = if observation.age_hours <= expectation.max_age_hours {
            HealthStatus::Healthy
        } else {
            HealthStatus::Unhealthy
        };
        let explanation = match status {
            HealthStatus::Healthy => format!(
                "observed age ({} hours) is within the expected maximum ({} hours)",
                observation.age_hours, expectation.max_age_hours
            ),
            HealthStatus::Unhealthy => format!(
                "observed age ({} hours) exceeds the expected maximum ({} hours)",
                observation.age_hours, expectation.max_age_hours
            ),
        };

        Self {
            identity: goods.identity.clone(),
            observation,
            expectation,
            health: HealthAssessment { status, explanation },
        }
    }
}
