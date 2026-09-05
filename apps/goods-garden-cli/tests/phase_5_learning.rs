use goods_domain::care::{Caregiver, HumanFeedback};
use goods_domain::goods::{Goods, GoodsIdentity, GoodsProfile};
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
fn a_follow_up_observation_without_the_need_is_resolved() {
    let item = goods("rice-ball", "phase-5-demo-001", 8, 2);
    let initial_source = DemoObservationSource::new(observation(11, 6));
    let feedback = feedback_source("Reviewed and pulled the item from the shelf.");

    let (_, _, _, action) = GoodsRuntime::new(initial_source)
        .request_care(&item, &feedback)
        .expect("the synthetic observation and feedback should be available");
    let action = action.expect("a Care Action should have been recorded");

    let follow_up_source = DemoObservationSource::new(observation(2, 6));
    let learning = GoodsRuntime::new(follow_up_source)
        .verify_and_learn(&item, action)
        .expect("the follow-up observation should be available");

    assert_eq!(learning.outcome.status, OutcomeStatus::Resolved);
    assert!(learning.statement.contains("resolved"));
}

#[test]
fn a_follow_up_observation_with_the_same_need_is_unresolved() {
    let item = goods("rice-ball", "phase-5-demo-002", 8, 2);
    let initial_source = DemoObservationSource::new(observation(11, 6));
    let feedback = feedback_source("Reviewed and pulled the item from the shelf.");

    let (_, _, _, action) = GoodsRuntime::new(initial_source)
        .request_care(&item, &feedback)
        .expect("the synthetic observation and feedback should be available");
    let action = action.expect("a Care Action should have been recorded");

    let follow_up_source = DemoObservationSource::new(observation(12, 6));
    let learning = GoodsRuntime::new(follow_up_source)
        .verify_and_learn(&item, action)
        .expect("the follow-up observation should be available");

    assert_eq!(learning.outcome.status, OutcomeStatus::Unresolved);
    assert!(learning.statement.contains("did not resolve"));
}
