//! Phase 2 Need model: Deviation, Urgency, GoodsNeed and Need Conflict.

pub mod deviation;
pub mod goods_need;
pub mod need_assessment;
pub mod need_conflict;
pub mod urgency;

pub use deviation::{Deviation, DeviationDimension};
pub use goods_need::{GoodsNeed, NeedKind};
pub use need_assessment::NeedAssessment;
pub use need_conflict::NeedConflict;
pub use urgency::Urgency;
