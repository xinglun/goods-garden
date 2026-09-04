//! Remember Care use case.

use goods_domain::care::CareAction;
use goods_domain::memory::{GoodsMemory, MemoryRecord};
use goods_domain::state::GoodsState;

/// Application boundary for remembering one Care episode.
pub struct RememberCare;

impl RememberCare {
    pub fn execute(memory: &mut GoodsMemory, state: GoodsState, action: CareAction) {
        memory.remember(MemoryRecord { state, action });
    }
}
