//! Evidence and Information State.
//!
//! `docs/architecture/trust-model.md` defines the information state values a
//! claim can carry: `KNOWN | INFERRED | UNKNOWN | UNAVAILABLE | CONFLICTING`.
//! Evidence pairs a plain-language statement with one of these states,
//! replacing the untyped `explanation: String` fields Phase 1-8 used.

use std::fmt;

/// The trust-model information state a piece of Evidence carries.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum InformationState {
    Known,
    Inferred,
    Unknown,
    Unavailable,
    Conflicting,
}

impl InformationState {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Known => "KNOWN",
            Self::Inferred => "INFERRED",
            Self::Unknown => "UNKNOWN",
            Self::Unavailable => "UNAVAILABLE",
            Self::Conflicting => "CONFLICTING",
        }
    }
}

/// Traceable support for a domain interpretation: a plain-language statement
/// tagged with the Information State it is trusted at.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Evidence {
    pub state: InformationState,
    pub statement: String,
}

impl Evidence {
    /// Evidence for a statement derived from data the domain already
    /// measured or computed directly, with no interpretive rule applied.
    pub fn known(statement: impl Into<String>) -> Self {
        Self { state: InformationState::Known, statement: statement.into() }
    }

    /// Evidence for a statement derived via an explicitly
    /// hypothesis/example-labelled interpretive rule rather than measured
    /// data (see `Urgency`'s fixed thresholds).
    pub fn inferred(statement: impl Into<String>) -> Self {
        Self { state: InformationState::Inferred, statement: statement.into() }
    }
}

impl fmt::Display for Evidence {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} [{}]", self.statement, self.state.as_str())
    }
}
