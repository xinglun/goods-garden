use goods_domain::care::{Caregiver, HumanFeedback};
use goods_domain::goods::{Goods, GoodsIdentity, GoodsProfile};
use goods_domain::memory::GoodsMemory;
use goods_domain::observation::observation::Observation;
use goods_domain::outcome::OutcomeStatus;
use goods_infrastructure::simulator::{DemoHumanFeedbackSource, DemoObservationSource};
use goods_runtime::GoodsRuntime;

fn goods(
    species: &str,
    individual_id: &str,
    expected_lifetime_hours: u32,
    minimum_stock_quantity: u32,
) -> Goods {
    Goods::new(
        GoodsIdentity { species: species.to_owned(), individual_id: individual_id.to_owned() },
        GoodsProfile {
            display_name: species.to_owned(),
            expected_lifetime_hours,
            minimum_stock_quantity,
        },
    )
}

fn observation(age_hours: u32, quantity_on_hand: u32) -> Observation {
    Observation {
        source: "synthetic-example".to_owned(),
        observed_at: "2026-08-28T00:00:00Z".to_owned(),
        age_hours,
        quantity_on_hand,
    }
}

fn feedback_source(decision: &str) -> DemoHumanFeedbackSource {
    DemoHumanFeedbackSource::new(HumanFeedback {
        caregiver: Caregiver {
            role: "store staff".to_owned(),
            display_name: "Demo Staff".to_owned(),
        },
        decision: decision.to_owned(),
        provided_at: "2026-08-28T01:00:00Z".to_owned(),
    })
}

#[test]
fn a_cycle_with_no_pending_action_has_no_verification() {
    let item = goods("rice-ball", "phase-11-demo-001", 8, 2);
    let source = DemoObservationSource::new(observation(4, 6));
    let feedback = feedback_source("acknowledged");
    let mut memory = GoodsMemory::new();

    let cycle = GoodsRuntime::new(source)
        .run_cycle(&item, &feedback, &mut memory, None)
        .expect("the synthetic observation and feedback should be available");

    assert!(cycle.verification.is_none());
    assert!(cycle.needs.needs.is_empty());
    assert!(cycle.request.is_none());
    assert!(cycle.action.is_none());
}

#[test]
fn a_cycle_verifies_and_learns_from_the_prior_cycles_action() {
    let item = goods("rice-ball", "phase-11-demo-002", 8, 2);
    let feedback = feedback_source("Reviewed and pulled the item from the shelf.");
    let mut memory = GoodsMemory::new();

    let first_source = DemoObservationSource::new(observation(11, 6));
    let first_cycle = GoodsRuntime::new(first_source)
        .run_cycle(&item, &feedback, &mut memory, None)
        .expect("the first synthetic observation and feedback should be available");

    assert_eq!(first_cycle.needs.needs.len(), 1);
    assert!(first_cycle.verification.is_none());
    let pending_action = first_cycle.action.expect("a Care Action should have been recorded");

    let follow_up_source = DemoObservationSource::new(observation(2, 6));
    let second_cycle = GoodsRuntime::new(follow_up_source)
        .run_cycle(&item, &feedback, &mut memory, Some(pending_action))
        .expect("the follow-up synthetic observation should be available");

    let learning = second_cycle.verification.expect("a verification should have been produced");
    assert_eq!(learning.outcome.status, OutcomeStatus::Resolved);
    assert_eq!(memory.records().len(), 1);
}
