//! External adapter ownership for Goods Garden.
//!
//! Phase 1 contains only a local synthetic observation adapter. No database,
//! provider SDK, HTTP client or real POS contract is introduced.

pub mod inventory;
pub mod llm;
pub mod persistence;
pub mod pos;
pub mod simulator;
pub mod weather;
