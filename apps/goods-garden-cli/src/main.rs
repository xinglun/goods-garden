//! Goods Garden local Phase 1 demo.

use std::error::Error;
use std::process;

use goods_domain::goods::{Goods, GoodsIdentity, GoodsProfile};
use goods_infrastructure::simulator::DemoObservationSource;
use goods_runtime::GoodsRuntime;

const TUNA_MAYO_FIXTURE: &str = include_str!("../../../examples/tuna-mayo/observation.example.txt");

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
        GoodsProfile { display_name: "Tuna-mayo rice ball".to_owned(), expected_lifetime_hours: 8 },
    );
    let source = DemoObservationSource::from_fixture(TUNA_MAYO_FIXTURE)?;
    let state = GoodsRuntime::new(source).observe_and_assess(&item)?;

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

    Ok(())
}
