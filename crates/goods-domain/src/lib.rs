//! Domain language for the first living good.
//!
//! Phase 1 established the minimal State and Health Assessment model needed
//! by the synthetic local demo. Phase 2 added Deviation, Urgency, GoodsNeed
//! and Need Conflict; Phase 3 adds CareRequest, Caregiver, Human Feedback and
//! CareAction. Memory and Learning remain outside this crate's implemented
//! behavior.

pub mod care;
pub mod evidence;
pub mod goods;
pub mod learning;
pub mod lifecycle;
pub mod memory;
pub mod need;
pub mod observation;
pub mod state;
