use goods_domain::care::{Caregiver, HumanFeedback};
use goods_domain::goods::{Goods, GoodsIdentity, GoodsProfile};
use goods_domain::observation::observation::Observation;
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
fn within_both_expectations_raises_no_care_request() {
    let item = goods("rice-ball", "phase-3-demo-001", 8, 2);
    let source = DemoObservationSource::new(observation(4, 6));
    let feedback = feedback_source("acknowledged");

    let (_, _, request, action) = GoodsRuntime::new(source)
        .request_care(&item, &feedback)
        .expect("the synthetic observation and feedback should be available");

    assert!(request.is_none());
    assert!(action.is_none());
}

#[test]
fn a_single_need_raises_a_care_request_and_records_a_care_action_from_feedback() {
    let item = goods("rice-ball", "phase-3-demo-002", 8, 2);
    let source = DemoObservationSource::new(observation(11, 6));
    let feedback = feedback_source("Reviewed and pulled the item from the shelf.");

    let (_, needs, request, action) = GoodsRuntime::new(source)
        .request_care(&item, &feedback)
        .expect("the synthetic observation and feedback should be available");

    assert_eq!(needs.needs.len(), 1);
    let request = request.expect("a Care Request should be raised");
    assert_eq!(request.requested_role, "store staff");
    assert_eq!(request.needs.len(), 1);

    let action = action.expect("a Care Action should be recorded from Human Feedback");
    assert_eq!(action.feedback.decision, "Reviewed and pulled the item from the shelf.");
    assert!(action.evidence.statement.contains("Reviewed and pulled the item from the shelf."));
}

#[test]
fn a_need_conflict_is_carried_into_the_care_request_explanation() {
    let item = goods("rice-ball", "phase-3-demo-003", 8, 5);
    let source = DemoObservationSource::new(observation(11, 1));
    let feedback = feedback_source("Escalated to the shift lead for a decision.");

    let (_, needs, request, action) = GoodsRuntime::new(source)
        .request_care(&item, &feedback)
        .expect("the synthetic observation and feedback should be available");

    assert!(needs.conflict.is_some());
    let request = request.expect("a Care Request should be raised");
    assert!(request.conflict.is_some());
    assert!(request.evidence.statement.contains("Need Conflict"));

    let action = action.expect("a Care Action should be recorded from Human Feedback");
    assert_eq!(action.feedback.decision, "Escalated to the shift lead for a decision.");
}
