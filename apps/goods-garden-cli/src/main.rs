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
        Some("multiple-individuals") => run_multiple_individuals(),
        Some("multiple-goods") => run_multiple_goods(),
        _ => {
            println!(
                "Usage: goods-garden-cli <demo|seven-day-life|multiple-individuals|multiple-goods>"
            );
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
    println!("lifecycle: {}", item.lifecycle.as_str());
    println!(
        "observation: {} (age {} hours)",
        state.observation.observed_at, state.observation.age_hours
    );
    println!("expectation: maximum age {} hours", state.expectation.max_age_hours);
    println!("health: {}", state.health.status.as_str());
    println!("explanation: {}", state.health.evidence);

    if needs.needs.is_empty() {
        println!("needs: <none identified>");
    } else {
        for need in &needs.needs {
            println!(
                "need: {:?} (urgency: {}) — {}",
                need.kind,
                need.urgency.as_str(),
                need.evidence
            );
        }
    }
    match &needs.conflict {
        Some(conflict) => println!("need conflict: {}", conflict.evidence),
        None => println!("need conflict: <none identified>"),
    }

    match &request {
        Some(request) => {
            println!("care request: ({}) {}", request.requested_role, request.evidence)
        }
        None => println!("care request: <none identified>"),
    }
    match &action {
        Some(action) => println!("care action: {}", action.evidence),
        None => println!("care action: <none identified>"),
    }
    println!("memory: {} care episode(s) remembered", memory.records().len());

    match action {
        Some(action) => {
            let learning = runtime.verify_and_learn(&item, action)?;
            println!("outcome: {:?} — {}", learning.outcome.status, learning.outcome.evidence);
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
    println!("lifecycle: {}", item.lifecycle.as_str());
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

        let cycle =
            runtime.run_cycle(&item, &feedback_source, &mut memory, pending_action.take())?;

        if let Some(learning) = &cycle.verification {
            println!(
                "outcome (follow-up on prior Care Action): {:?} — {}",
                learning.outcome.status, learning.outcome.evidence
            );
            println!("learning: {}", learning.statement);
        }

        println!(
            "health: {} — {}",
            cycle.state.health.status.as_str(),
            cycle.state.health.evidence
        );
        if cycle.needs.needs.is_empty() {
            println!("needs: <none identified>");
        } else {
            for need in &cycle.needs.needs {
                println!(
                    "need: {:?} (urgency: {}) — {}",
                    need.kind,
                    need.urgency.as_str(),
                    need.evidence
                );
            }
        }
        match &cycle.request {
            Some(request) => {
                println!("care request: ({}) {}", request.requested_role, request.evidence)
            }
            None => println!("care request: <none identified>"),
        }
        match &cycle.action {
            Some(action) => println!("care action: {}", action.evidence),
            None => println!("care action: <none identified>"),
        }

        pending_action = cycle.action;
    }

    println!();
    println!("== Week summary ==");
    println!("{} care episode(s) remembered across seven synthetic days.", memory.records().len());

    Ok(())
}

/// One monitored individual in the Phase 7 Multiple Individuals demo: a
/// distinct identity and its own Memory, sharing the species-level
/// GoodsProfile with every other individual.
struct MonitoredIndividual {
    label: &'static str,
    individual_id: &'static str,
    age_hours: u32,
    quantity_on_hand: u32,
    feedback_decision: &'static str,
}

fn run_multiple_individuals() -> Result<(), Box<dyn Error>> {
    // The Species: a shared, reusable GoodsProfile. Cloning it for each
    // individual demonstrates that Species-level data is shared, while each
    // individual's Memory below is not.
    let species_profile = GoodsProfile {
        display_name: "Tuna-mayo rice ball".to_owned(),
        expected_lifetime_hours: 8,
        minimum_stock_quantity: 2,
    };

    let individuals = [
        MonitoredIndividual {
            label: "TunaMayo@StoreA",
            individual_id: "tuna-mayo@store-a",
            age_hours: 10,
            quantity_on_hand: 6,
            feedback_decision: "Reviewed and pulled the item from the shelf at Store A.",
        },
        MonitoredIndividual {
            label: "TunaMayo@StoreB",
            individual_id: "tuna-mayo@store-b",
            age_hours: 3,
            quantity_on_hand: 5,
            feedback_decision: "N/A (no Care Request expected at Store B)",
        },
    ];

    println!("Goods Garden Multiple Individuals demo (synthetic-example)");
    println!("species: rice-ball / \"{}\"", species_profile.display_name);

    let mut memory_counts = Vec::new();

    for individual in &individuals {
        println!();
        println!("== {} ==", individual.label);

        let item = Goods::new(
            GoodsIdentity {
                species: "rice-ball".to_owned(),
                individual_id: individual.individual_id.to_owned(),
            },
            species_profile.clone(),
        );
        println!("lifecycle: {}", item.lifecycle.as_str());
        let observation = Observation {
            source: "synthetic-example".to_owned(),
            observed_at: format!("{} (synthetic)", individual.label),
            age_hours: individual.age_hours,
            quantity_on_hand: individual.quantity_on_hand,
        };
        let feedback = HumanFeedback {
            caregiver: Caregiver {
                role: "store staff".to_owned(),
                display_name: "Demo Staff".to_owned(),
            },
            decision: individual.feedback_decision.to_owned(),
            provided_at: format!("{} (synthetic)", individual.label),
        };
        let runtime = GoodsRuntime::new(DemoObservationSource::new(observation));
        let feedback_source = DemoHumanFeedbackSource::new(feedback);
        let mut memory = GoodsMemory::new();

        let (state, needs, request, action) =
            runtime.request_care_and_remember(&item, &feedback_source, &mut memory)?;

        println!("health: {} — {}", state.health.status.as_str(), state.health.evidence);
        if needs.needs.is_empty() {
            println!("needs: <none identified>");
        } else {
            for need in &needs.needs {
                println!(
                    "need: {:?} (urgency: {}) — {}",
                    need.kind,
                    need.urgency.as_str(),
                    need.evidence
                );
            }
        }
        match &request {
            Some(request) => {
                println!("care request: ({}) {}", request.requested_role, request.evidence)
            }
            None => println!("care request: <none identified>"),
        }
        match &action {
            Some(action) => println!("care action: {}", action.evidence),
            None => println!("care action: <none identified>"),
        }
        println!(
            "memory: {} care episode(s) remembered for {}",
            memory.records().len(),
            individual.label
        );

        memory_counts.push((individual.label, memory.records().len()));
    }

    println!();
    println!("== Separation summary ==");
    for (label, count) in &memory_counts {
        println!(
            "{label}: {count} care episode(s) remembered, held in its own separate GoodsMemory"
        );
    }
    println!(
        "Each individual's Memory is a distinct value owned only by that individual; \
         neither individual's memory contains the other's records, even though both share the \
         same species profile."
    );

    Ok(())
}

/// One product species in the Phase 8 Multiple Goods demo. Goods
/// Intelligence is a class of capability: every species below is processed
/// by the identical `request_care_and_remember` call, differing only in the
/// data carried by its `GoodsProfile` and `Observation`.
struct ProductSpecies {
    species: &'static str,
    display_name: &'static str,
    expected_lifetime_hours: u32,
    minimum_stock_quantity: u32,
    age_hours: u32,
    quantity_on_hand: u32,
}

const MULTIPLE_GOODS_SCRIPT: [ProductSpecies; 4] = [
    ProductSpecies {
        species: "rice-ball-salmon",
        display_name: "Salmon rice ball",
        expected_lifetime_hours: 8,
        minimum_stock_quantity: 2,
        age_hours: 3,
        quantity_on_hand: 5,
    },
    ProductSpecies {
        species: "coffee",
        display_name: "Iced coffee",
        expected_lifetime_hours: 4,
        minimum_stock_quantity: 3,
        age_hours: 5,
        quantity_on_hand: 4,
    },
    ProductSpecies {
        species: "sandwich",
        display_name: "Egg sandwich",
        expected_lifetime_hours: 6,
        minimum_stock_quantity: 2,
        age_hours: 2,
        quantity_on_hand: 4,
    },
    ProductSpecies {
        species: "bento",
        display_name: "Chicken bento",
        expected_lifetime_hours: 10,
        minimum_stock_quantity: 1,
        age_hours: 4,
        quantity_on_hand: 3,
    },
];

fn run_multiple_goods() -> Result<(), Box<dyn Error>> {
    println!("Goods Garden Multiple Goods demo (synthetic-example)");

    let mut any_need_identified = false;

    for product in &MULTIPLE_GOODS_SCRIPT {
        println!();
        println!("== {} ({}) ==", product.display_name, product.species);

        let item = Goods::new(
            GoodsIdentity {
                species: product.species.to_owned(),
                individual_id: format!("{}-demo-001", product.species),
            },
            GoodsProfile {
                display_name: product.display_name.to_owned(),
                expected_lifetime_hours: product.expected_lifetime_hours,
                minimum_stock_quantity: product.minimum_stock_quantity,
            },
        );
        println!("lifecycle: {}", item.lifecycle.as_str());
        let observation = Observation {
            source: "synthetic-example".to_owned(),
            observed_at: format!("{} (synthetic)", product.display_name),
            age_hours: product.age_hours,
            quantity_on_hand: product.quantity_on_hand,
        };
        let feedback = HumanFeedback {
            caregiver: Caregiver {
                role: "store staff".to_owned(),
                display_name: "Demo Staff".to_owned(),
            },
            decision: "Reviewed and pulled the item from the shelf.".to_owned(),
            provided_at: format!("{} (synthetic)", product.display_name),
        };
        let runtime = GoodsRuntime::new(DemoObservationSource::new(observation));
        let feedback_source = DemoHumanFeedbackSource::new(feedback);
        let mut memory = GoodsMemory::new();

        // The same call, unmodified, for every species: no product-specific
        // branch exists in goods-domain, goods-application or goods-runtime.
        let (state, needs, request, action) =
            runtime.request_care_and_remember(&item, &feedback_source, &mut memory)?;

        println!(
            "expectation: maximum age {} hours, minimum stock {}",
            state.expectation.max_age_hours, product.minimum_stock_quantity
        );
        println!("health: {} — {}", state.health.status.as_str(), state.health.evidence);
        if needs.needs.is_empty() {
            println!("needs: <none identified>");
        } else {
            any_need_identified = true;
            for need in &needs.needs {
                println!(
                    "need: {:?} (urgency: {}) — {}",
                    need.kind,
                    need.urgency.as_str(),
                    need.evidence
                );
            }
        }
        match &request {
            Some(request) => {
                println!("care request: ({}) {}", request.requested_role, request.evidence)
            }
            None => println!("care request: <none identified>"),
        }
        match &action {
            Some(action) => println!("care action: {}", action.evidence),
            None => println!("care action: <none identified>"),
        }
    }

    println!();
    println!("== Capability summary ==");
    println!(
        "{} distinct product species were processed by the identical GoodsRuntime call above; \
         at least one Need was identified: {any_need_identified}.",
        MULTIPLE_GOODS_SCRIPT.len()
    );
    println!(
        "No product-specific code branch was needed in goods-domain, goods-application or \
         goods-runtime: Goods Intelligence is a class of capability, and each product above is \
         only data — an object/instance of that capability."
    );

    Ok(())
}
