//! Phase 4 Memory model: an append-only Relationship Memory of Care episodes.

#[allow(clippy::module_inception)]
pub mod memory;
pub mod memory_record;

pub use memory::GoodsMemory;
pub use memory_record::MemoryRecord;
