//! Local synthetic observation adapter for the Phase 1 demo.

use std::error::Error;
use std::fmt::{Display, Formatter};

use goods_application::ports::observation_source::ObservationSource;
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

        Ok(Self::new(Observation { source, observed_at, age_hours }))
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
