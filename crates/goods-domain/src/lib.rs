//! Domain language for the first living good.
//!
//! Phase 1 established the minimal State and Health Assessment model needed
//! by the synthetic local demo. Phase 2 added Deviation, Urgency, GoodsNeed
//! and Need Conflict; Phase 3 added CareRequest, Caregiver, Human Feedback and
//! CareAction; Phase 4 added an append-only GoodsMemory of Care episodes, with
//! no retention policy; Phase 5 adds Outcome and Learning, comparing a Care
//! Action against a follow-up observation. Learning is a reviewable
//! statement only — it never adjusts a threshold, profile field or other
//! rule by itself.

pub mod care;
pub mod evidence;
pub mod goods;
pub mod learning;
pub mod lifecycle;
pub mod memory;
pub mod need;
pub mod observation;
pub mod outcome;
pub mod state;
