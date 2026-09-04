//! Local synthetic adapters for the Phase 1-3 demo: observation and, since
//! Phase 3, Human Feedback in response to a Care Request.

use std::error::Error;
use std::fmt::{Display, Formatter};

use goods_application::ports::human_feedback_source::HumanFeedbackSource;
use goods_application::ports::observation_source::ObservationSource;
use goods_domain::care::{CareRequest, Caregiver, HumanFeedback};
use goods_domain::goods::Goods;
use goods_domain::observation::Observation;

const SYNTHETIC_SOURCE: &str = "synthetic-example";

/// A local observation source used only by the runnable demo.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DemoObservationSource {
    observation: Observation,
}

impl DemoObservationSource {
    pub fn new(observation: Observation) -> Self {
        Self { observation }
    }

    pub fn from_fixture(contents: &str) -> Result<Self, DemoObservationError> {
        let mut source = None;
        let mut observed_at = None;
        let mut age_hours = None;
        let mut quantity_on_hand = None;

        for line in contents.lines() {
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') {
                continue;
            }

            let (key, value) =
                line.split_once('=').ok_or_else(|| DemoObservationError::invalid_line(line))?;
            match key.trim() {
                "source_status" => source = Some(value.trim().to_owned()),
                "observed_at" => observed_at = Some(value.trim().to_owned()),
                "age_hours" => {
                    age_hours = Some(
                        value
                            .trim()
                            .parse()
                            .map_err(|_| DemoObservationError::invalid_age(value.trim()))?,
                    )
                }
                "quantity_on_hand" => {
                    quantity_on_hand = Some(
                        value
                            .trim()
                            .parse()
                            .map_err(|_| DemoObservationError::invalid_quantity(value.trim()))?,
                    )
                }
                _ => {}
            }
        }

        let source = source.ok_or_else(|| DemoObservationError::missing("source_status"))?;
        if source != SYNTHETIC_SOURCE {
            return Err(DemoObservationError::unsupported_source(source));
        }

        let observed_at =
            observed_at.ok_or_else(|| DemoObservationError::missing("observed_at"))?;
        if observed_at.is_empty() {
            return Err(DemoObservationError::empty("observed_at"));
        }

        let age_hours = age_hours.ok_or_else(|| DemoObservationError::missing("age_hours"))?;
        let quantity_on_hand =
            quantity_on_hand.ok_or_else(|| DemoObservationError::missing("quantity_on_hand"))?;

        Ok(Self::new(Observation { source, observed_at, age_hours, quantity_on_hand }))
    }
}

impl ObservationSource for DemoObservationSource {
    type Error = DemoObservationError;

    fn observe(&self, _goods: &Goods) -> Result<Observation, Self::Error> {
        Ok(self.observation.clone())
    }
}

/// Parsing or boundary errors for the local synthetic fixture.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DemoObservationError(String);

impl DemoObservationError {
    fn missing(field: &'static str) -> Self {
        Self(format!("missing fixture field: {field}"))
    }

    fn empty(field: &'static str) -> Self {
        Self(format!("fixture field is empty: {field}"))
    }

    fn invalid_age(value: &str) -> Self {
        Self(format!("invalid age_hours value: {value}"))
    }

    fn invalid_quantity(value: &str) -> Self {
        Self(format!("invalid quantity_on_hand value: {value}"))
    }

    fn invalid_line(line: &str) -> Self {
        Self(format!("invalid fixture line: {line}"))
    }

    fn unsupported_source(source: String) -> Self {
        Self(format!("unsupported observation source: {source}"))
    }
}

impl Display for DemoObservationError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(&self.0)
    }
}

impl Error for DemoObservationError {}

/// A local Human Feedback source used only by the runnable demo. The
/// feedback is a synthetic stand-in for a real Caregiver's response; it is
/// never computed by this adapter, only read from a fixture.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DemoHumanFeedbackSource {
    feedback: HumanFeedback,
}

impl DemoHumanFeedbackSource {
    pub fn new(feedback: HumanFeedback) -> Self {
        Self { feedback }
    }

    pub fn from_fixture(contents: &str) -> Result<Self, DemoHumanFeedbackError> {
        let mut role = None;
        let mut display_name = None;
        let mut decision = None;
        let mut provided_at = None;

        for line in contents.lines() {
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') {
                continue;
            }

            let (key, value) =
                line.split_once('=').ok_or_else(|| DemoHumanFeedbackError::invalid_line(line))?;
            match key.trim() {
                "caregiver_role" => role = Some(value.trim().to_owned()),
                "caregiver_name" => display_name = Some(value.trim().to_owned()),
                "decision" => decision = Some(value.trim().to_owned()),
                "provided_at" => provided_at = Some(value.trim().to_owned()),
                _ => {}
            }
        }

        let role = role.ok_or_else(|| DemoHumanFeedbackError::missing("caregiver_role"))?;
        let display_name =
            display_name.ok_or_else(|| DemoHumanFeedbackError::missing("caregiver_name"))?;
        let decision = decision.ok_or_else(|| DemoHumanFeedbackError::missing("decision"))?;
        if decision.is_empty() {
            return Err(DemoHumanFeedbackError::empty("decision"));
        }
        let provided_at =
            provided_at.ok_or_else(|| DemoHumanFeedbackError::missing("provided_at"))?;

        Ok(Self::new(HumanFeedback {
            caregiver: Caregiver { role, display_name },
            decision,
            provided_at,
        }))
    }
}

impl HumanFeedbackSource for DemoHumanFeedbackSource {
    type Error = DemoHumanFeedbackError;

    fn provide_feedback(&self, _request: &CareRequest) -> Result<HumanFeedback, Self::Error> {
        Ok(self.feedback.clone())
    }
}

/// Parsing or boundary errors for the local synthetic Human Feedback fixture.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DemoHumanFeedbackError(String);

impl DemoHumanFeedbackError {
    fn missing(field: &'static str) -> Self {
        Self(format!("missing fixture field: {field}"))
    }

    fn empty(field: &'static str) -> Self {
        Self(format!("fixture field is empty: {field}"))
    }

    fn invalid_line(line: &str) -> Self {
        Self(format!("invalid fixture line: {line}"))
    }
}

impl Display for DemoHumanFeedbackError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(&self.0)
    }
}

impl Error for DemoHumanFeedbackError {}
