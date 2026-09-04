use goods_domain::goods::{Goods, GoodsIdentity, GoodsProfile};
use goods_domain::need::NeedKind;
use goods_domain::observation::observation::Observation;
use goods_infrastructure::simulator::DemoObservationSource;
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

#[test]
fn within_both_expectations_identifies_no_need() {
    let item = goods("rice-ball", "phase-2-demo-001", 8, 2);
    let source = DemoObservationSource::new(observation(4, 6));

    let (_, assessment) = GoodsRuntime::new(source)
        .observe_and_identify_needs(&item)
        .expect("the synthetic observation should be available");

    assert!(assessment.needs.is_empty());
    assert!(assessment.conflict.is_none());
}

#[test]
fn freshness_deviation_alone_identifies_a_freshness_concern_without_conflict() {
    let item = goods("rice-ball", "phase-2-demo-002", 8, 2);
    let source = DemoObservationSource::new(observation(11, 6));

    let (_, assessment) = GoodsRuntime::new(source)
        .observe_and_identify_needs(&item)
        .expect("the synthetic observation should be available");

    assert_eq!(assessment.needs.len(), 1);
    assert_eq!(assessment.needs[0].kind, NeedKind::FreshnessConcern);
    assert!(assessment.conflict.is_none());
}

#[test]
fn stock_deviation_alone_identifies_a_stock_availability_concern_without_conflict() {
    let item = goods("rice-ball", "phase-2-demo-003", 8, 5);
    let source = DemoObservationSource::new(observation(4, 1));

    let (_, assessment) = GoodsRuntime::new(source)
        .observe_and_identify_needs(&item)
        .expect("the synthetic observation should be available");

    assert_eq!(assessment.needs.len(), 1);
    assert_eq!(assessment.needs[0].kind, NeedKind::StockAvailabilityConcern);
    assert!(assessment.conflict.is_none());
}

#[test]
fn both_deviations_identify_two_needs_and_a_need_conflict() {
    let item = goods("rice-ball", "phase-2-demo-004", 8, 5);
    let source = DemoObservationSource::new(observation(11, 1));

    let (_, assessment) = GoodsRuntime::new(source)
        .observe_and_identify_needs(&item)
        .expect("the synthetic observation should be available");

    assert_eq!(assessment.needs.len(), 2);
    let conflict = assessment.conflict.expect("a Need Conflict should be identified");
    assert_eq!(conflict.freshness_need.kind, NeedKind::FreshnessConcern);
    assert_eq!(conflict.availability_need.kind, NeedKind::StockAvailabilityConcern);
}
