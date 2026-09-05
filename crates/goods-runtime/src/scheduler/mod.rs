//! Scheduler: a bounded, foreground-only driver that runs a fixed,
//! synthetic script of Intelligence Loop cycles unattended, pacing them
//! over real time. This is the one place in the repository where a cycle
//! runs without a human invoking it directly.
//!
//! Deliberately NOT generic over ObservationSource/HumanFeedbackSource:
//! only the synthetic DemoObservationSource/DemoHumanFeedbackSource can be
//! driven this way. Running against a real adapter requires rewriting this
//! module — that is the point: swapping to real data must be a fresh,
//! visible decision, not a config change. This module performs no file or
//! network I/O of its own; the caller decides how to surface each cycle
//! (stdout, a log file, both).

use std::error::Error as StdError;
use std::thread;
use std::time::Duration;

use goods_domain::care::{CareAction, HumanFeedback};
use goods_domain::goods::Goods;
use goods_domain::memory::GoodsMemory;
use goods_domain::observation::Observation;
use goods_infrastructure::simulator::{DemoHumanFeedbackSource, DemoObservationSource};

use crate::GoodsRuntime;
use crate::intelligence_loop::IntelligenceCycleOutcome;

/// One scripted entry the scheduler will run unattended.
pub struct ScheduledCycle {
    pub label: &'static str,
    pub observation: Observation,
    pub feedback: HumanFeedback,
}

/// Why the scheduler stopped running cycles.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum StopReason {
    ScriptExhausted,
    MaxCyclesReached,
}

/// Run `script` unattended, one Intelligence Loop cycle per entry, pausing
/// `interval` between cycles. Stops after `max_cycles` regardless of how
/// long `script` is; a human must invoke this again to continue (no state
/// is carried over between calls beyond what `memory` already holds).
pub fn run_scheduled(
    goods: &Goods,
    script: &[ScheduledCycle],
    max_cycles: u32,
    interval: Duration,
    memory: &mut GoodsMemory,
    mut on_cycle: impl FnMut(usize, &ScheduledCycle, &IntelligenceCycleOutcome),
) -> Result<StopReason, Box<dyn StdError>> {
    let mut pending_action: Option<CareAction> = None;

    for (index, entry) in script.iter().enumerate() {
        if index as u32 >= max_cycles {
            return Ok(StopReason::MaxCyclesReached);
        }

        let runtime = GoodsRuntime::new(DemoObservationSource::new(entry.observation.clone()));
        let feedback_source = DemoHumanFeedbackSource::new(entry.feedback.clone());
        let cycle = runtime.run_cycle(goods, &feedback_source, memory, pending_action.take())?;

        on_cycle(index, entry, &cycle);
        pending_action = cycle.action.clone();

        if index + 1 < script.len() {
            thread::sleep(interval);
        }
    }

    Ok(StopReason::ScriptExhausted)
}
