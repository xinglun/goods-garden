//! Goods Garden local Phase 1 demo.

use std::error::Error;
use std::process;

use goods_domain::goods::{Goods, GoodsIdentity, GoodsProfile};
use goods_domain::memory::GoodsMemory;
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
        _ => {
            println!("Usage: goods-garden-cli demo");
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
    let (state, needs, request, action) = GoodsRuntime::new(source).request_care_and_remember(
        &item,
        &feedback_source,
        &mut memory,
    )?;

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

    Ok(())
}
