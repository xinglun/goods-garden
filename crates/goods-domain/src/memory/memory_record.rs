//! Memory Record.

use crate::care::CareAction;
use crate::state::GoodsState;

/// A record of one Care episode: the State that prompted a Need, and the
/// Care Action that responded to it.
///
/// A Memory Record does not judge whether the Care Action worked; comparing
/// it against a later State is Phase 5's Verification and Learning.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MemoryRecord {
    pub state: GoodsState,
    pub action: CareAction,
}
