//! Runtime orchestration for the bounded Phase 1 State demo.
//!
//! The runtime currently composes Observe and Assess only. Need, Care,
//! Verify and Learn remain future phases.

pub mod intelligence_loop;
pub mod lifecycle;
pub mod runtime;
pub mod scheduler;

pub use runtime::GoodsRuntime;
