//! Evidence and Information State: traceable support for a domain
//! interpretation, tagged with a trust-model information state.

#[allow(clippy::module_inception)]
pub mod evidence;

pub use evidence::{Evidence, InformationState};
