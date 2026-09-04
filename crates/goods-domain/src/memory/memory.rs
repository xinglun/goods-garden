//! Goods Memory.

use super::memory_record::MemoryRecord;

/// An append-only Relationship Memory for one good: what happened and what
/// Care Action responded to it.
///
/// Phase 4 intentionally does not define a retention or eviction policy;
/// records are never removed. This type has no persistence of its own — it
/// is an in-process value owned by its caller across repeated observations.
/// See `docs/phases/phase-4-memory.md`.
#[derive(Clone, Debug, Eq, PartialEq, Default)]
pub struct GoodsMemory {
    records: Vec<MemoryRecord>,
}

impl GoodsMemory {
    pub fn new() -> Self {
        Self::default()
    }

    /// Remember one Care episode. This never removes or expires a prior
    /// record.
    pub fn remember(&mut self, record: MemoryRecord) {
        self.records.push(record);
    }

    pub fn records(&self) -> &[MemoryRecord] {
        &self.records
    }
}
