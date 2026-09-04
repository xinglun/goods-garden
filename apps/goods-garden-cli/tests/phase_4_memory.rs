use goods_domain::care::{Caregiver, HumanFeedback};
use goods_domain::goods::{Goods, GoodsIdentity, GoodsProfile};
use goods_domain::memory::GoodsMemory;
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
fn no_need_remembers_no_episode() {
    let item = goods("rice-ball", "phase-4-demo-001", 8, 2);
    let source = DemoObservationSource::new(observation(4, 6));
    let feedback = feedback_source("acknowledged");
    let mut memory = GoodsMemory::new();

    GoodsRuntime::new(source)
        .request_care_and_remember(&item, &feedback, &mut memory)
        .expect("the synthetic observation and feedback should be available");

    assert!(memory.records().is_empty());
}

#[test]
fn a_care_action_is_remembered_as_one_episode() {
    let item = goods("rice-ball", "phase-4-demo-002", 8, 2);
    let source = DemoObservationSource::new(observation(11, 6));
    let feedback = feedback_source("Reviewed and pulled the item from the shelf.");
    let mut memory = GoodsMemory::new();

    let (_, _, _, action) = GoodsRuntime::new(source)
        .request_care_and_remember(&item, &feedback, &mut memory)
        .expect("the synthetic observation and feedback should be available");

    assert_eq!(memory.records().len(), 1);
    let record = &memory.records()[0];
    assert_eq!(record.action, action.expect("a Care Action should have been recorded"));
}

#[test]
fn repeated_episodes_accumulate_without_evicting_prior_records() {
    let item = goods("rice-ball", "phase-4-demo-003", 8, 2);
    let mut memory = GoodsMemory::new();

    for _ in 0..3 {
        let source = DemoObservationSource::new(observation(11, 6));
        let feedback = feedback_source("Reviewed and pulled the item from the shelf.");
        GoodsRuntime::new(source)
            .request_care_and_remember(&item, &feedback, &mut memory)
            .expect("the synthetic observation and feedback should be available");
    }

    assert_eq!(memory.records().len(), 3);
}
