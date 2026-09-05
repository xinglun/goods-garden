//! Goods Garden local Phase 1 demo.

use std::error::Error;
use std::process;

use goods_domain::care::{Caregiver, HumanFeedback};
use goods_domain::goods::{Goods, GoodsIdentity, GoodsProfile};
use goods_domain::memory::GoodsMemory;
use goods_domain::observation::Observation;
use goods_infrastructure::simulator::{DemoHumanFeedbackSource, DemoObservationSource};
use goods_runtime::GoodsRuntime;

const TUNA_MAYO_FIXTURE: &str = include_str!("../../../examples/tuna-mayo/observation.example.txt");
const HUMAN_FEEDBACK_FIXTURE: &str =
    include_str!("../../../examples/tuna-mayo/human_feedback.example.txt");

fn main() {
    if let Err(error) = run() {
        eprintln!("error: {error}");
        process::exit(1);
    }
}

fn run() -> Result<(), Box<dyn Error>> {
    match std::env::args().nth(1).as_deref() {
        Some("demo") => run_demo(),
        Some("seven-day-life") => run_seven_day_life(),
        _ => {
            println!("Usage: goods-garden-cli <demo|seven-day-life>");
            Ok(())
        }
    }
}

fn run_demo() -> Result<(), Box<dyn Error>> {
    let item = Goods::new(
        GoodsIdentity {
            species: "rice-ball".to_owned(),
            individual_id: "tuna-mayo-demo-001".to_owned(),
        },
        GoodsProfile {
            display_name: "Tuna-mayo rice ball".to_owned(),
            expected_lifetime_hours: 8,
            minimum_stock_quantity: 2,
        },
    );
    let source = DemoObservationSource::from_fixture(TUNA_MAYO_FIXTURE)?;
    let feedback_source = DemoHumanFeedbackSource::from_fixture(HUMAN_FEEDBACK_FIXTURE)?;
    let mut memory = GoodsMemory::new();
    let runtime = GoodsRuntime::new(source);
    let (state, needs, request, action) =
        runtime.request_care_and_remember(&item, &feedback_source, &mut memory)?;

    println!("Goods Garden Phase 1 demo");
    println!("source: {}", state.observation.source);
    println!("identity: {} / {}", state.identity.species, state.identity.individual_id);
    println!(
        "observation: {} (age {} hours)",
        state.observation.observed_at, state.observation.age_hours
    );
    println!("expectation: maximum age {} hours", state.expectation.max_age_hours);
    println!("health: {}", state.health.status.as_str());
    println!("explanation: {}", state.health.explanation);

    if needs.needs.is_empty() {
        println!("needs: <none identified>");
    } else {
        for need in &needs.needs {
            println!(
                "need: {:?} (urgency: {}) — {}",
                need.kind,
                need.urgency.as_str(),
                need.explanation
            );
        }
    }
    match &needs.conflict {
        Some(conflict) => println!("need conflict: {}", conflict.explanation),
        None => println!("need conflict: <none identified>"),
    }

    match &request {
        Some(request) => {
            println!("care request: ({}) {}", request.requested_role, request.explanation)
        }
        None => println!("care request: <none identified>"),
    }
    match &action {
        Some(action) => println!("care action: {}", action.explanation),
        None => println!("care action: <none identified>"),
    }
    println!("memory: {} care episode(s) remembered", memory.records().len());

    match action {
        Some(action) => {
            let learning = runtime.verify_and_learn(&item, action)?;
            println!("outcome: {:?} — {}", learning.outcome.status, learning.outcome.explanation);
            println!("learning: {}", learning.statement);
        }
        None => println!("learning: <none pending>"),
    }

    Ok(())
}

/// One scripted synthetic day in the Phase 6 Seven Day Life milestone. The
/// good's identity represents a monitored retail slot, not a single physical
/// unit forced to age for a week; a restock resets `age_hours` on later
/// days, which is how a real shelf position behaves.
struct SyntheticDay {
    label: &'static str,
    age_hours: u32,
    quantity_on_hand: u32,
    /// `Some` only on days where the script expects a Need to be raised and
    /// a Caregiver to respond; this text is never invented by the runtime.
    feedback_decision: Option<&'static str>,
}

const SEVEN_DAY_SCRIPT: [SyntheticDay; 7] = [
    SyntheticDay {
        label: "Day 1 (normal)",
        age_hours: 2,
        quantity_on_hand: 6,
        feedback_decision: None,
    },
    SyntheticDay {
        label: "Day 2 (normal)",
        age_hours: 3,
        quantity_on_hand: 5,
        feedback_decision: None,
    },
    SyntheticDay {
        label: "Day 3 (anomaly: freshness)",
        age_hours: 10,
        quantity_on_hand: 6,
        feedback_decision: Some(
            "Reviewed and pulled the item from the shelf; restocked with fresh units.",
        ),
    },
    SyntheticDay {
        label: "Day 4 (verification + restock)",
        age_hours: 1,
        quantity_on_hand: 6,
        feedback_decision: None,
    },
    SyntheticDay {
        label: "Day 5 (normal)",
        age_hours: 2,
        quantity_on_hand: 5,
        feedback_decision: None,
    },
    SyntheticDay {
        label: "Day 6 (anomaly: stock availability)",
        age_hours: 3,
        quantity_on_hand: 1,
        feedback_decision: Some("Placed a restock order; will replenish before tomorrow."),
    },
    SyntheticDay {
        label: "Day 7 (verification + restock)",
        age_hours: 1,
        quantity_on_hand: 5,
        feedback_decision: None,
    },
];

fn run_seven_day_life() -> Result<(), Box<dyn Error>> {
    let item = Goods::new(
        GoodsIdentity {
            species: "rice-ball".to_owned(),
            individual_id: "tuna-mayo-demo-001".to_owned(),
        },
        GoodsProfile {
            display_name: "Tuna-mayo rice ball".to_owned(),
            expected_lifetime_hours: 8,
            minimum_stock_quantity: 2,
        },
    );

    println!("Goods Garden Seven Day Life demo (synthetic-example)");
    let mut memory = GoodsMemory::new();
    let mut pending_action = None;

    for day in &SEVEN_DAY_SCRIPT {
        println!();
        println!("== {} ==", day.label);

        let observation = Observation {
            source: "synthetic-example".to_owned(),
            observed_at: format!("{} (synthetic)", day.label),
            age_hours: day.age_hours,
            quantity_on_hand: day.quantity_on_hand,
        };
        let feedback = HumanFeedback {
            caregiver: Caregiver {
                role: "store staff".to_owned(),
                display_name: "Demo Staff".to_owned(),
            },
            decision: day.feedback_decision.unwrap_or("N/A (no Care Request today)").to_owned(),
            provided_at: format!("{} (synthetic)", day.label),
        };
        let runtime = GoodsRuntime::new(DemoObservationSource::new(observation));
        let feedback_source = DemoHumanFeedbackSource::new(feedback);

        if let Some(action) = pending_action.take() {
            let learning = runtime.verify_and_learn(&item, action)?;
            println!(
                "outcome (follow-up on prior Care Action): {:?} — {}",
                learning.outcome.status, learning.outcome.explanation
            );
            println!("learning: {}", learning.statement);
        }

        let (state, needs, request, action) =
            runtime.request_care_and_remember(&item, &feedback_source, &mut memory)?;

        println!("health: {} — {}", state.health.status.as_str(), state.health.explanation);
        if needs.needs.is_empty() {
            println!("needs: <none identified>");
        } else {
            for need in &needs.needs {
                println!("need: {:?} (urgency: {})", need.kind, need.urgency.as_str());
            }
        }
        match &request {
            Some(request) => {
                println!("care request: ({}) {}", request.requested_role, request.explanation)
            }
            None => println!("care request: <none identified>"),
        }
        match &action {
            Some(action) => println!("care action: {}", action.explanation),
            None => println!("care action: <none identified>"),
        }

        pending_action = action;
    }

    println!();
    println!("== Week summary ==");
    println!("{} care episode(s) remembered across seven synthetic days.", memory.records().len());

    Ok(())
}
