use std::process::Command;

use goods_domain::evidence::InformationState;
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
fn health_assessment_evidence_is_known_whether_healthy_or_unhealthy() {
    let healthy_item = goods("rice-ball", "phase-9-demo-001", 8, 2);
    let healthy_source = DemoObservationSource::new(observation(4, 6));
    let healthy_state = GoodsRuntime::new(healthy_source)
        .observe_and_assess(&healthy_item)
        .expect("the synthetic observation should be available");
    assert_eq!(healthy_state.health.evidence.state, InformationState::Known);

    let unhealthy_item = goods("rice-ball", "phase-9-demo-002", 8, 2);
    let unhealthy_source = DemoObservationSource::new(observation(11, 6));
    let unhealthy_state = GoodsRuntime::new(unhealthy_source)
        .observe_and_assess(&unhealthy_item)
        .expect("the synthetic observation should be available");
    assert_eq!(unhealthy_state.health.evidence.state, InformationState::Known);
}

#[test]
fn goods_need_is_inferred_while_its_deviation_is_known() {
    let item = goods("rice-ball", "phase-9-demo-003", 8, 2);
    let source = DemoObservationSource::new(observation(11, 6));

    let (_, assessment) = GoodsRuntime::new(source)
        .observe_and_identify_needs(&item)
        .expect("the synthetic observation should be available");

    assert_eq!(assessment.needs.len(), 1);
    let need = &assessment.needs[0];
    assert_eq!(need.kind, NeedKind::FreshnessConcern);
    assert_eq!(need.deviation.evidence.state, InformationState::Known);
    assert_eq!(need.evidence.state, InformationState::Inferred);
}

#[test]
fn cli_demo_prints_a_known_tag() {
    let output = Command::new(env!("CARGO_BIN_EXE_goods-garden-cli"))
        .arg("demo")
        .output()
        .expect("the CLI demo should start");

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).expect("CLI output is UTF-8");
    assert!(stdout.contains("[KNOWN]"));
}

#[test]
fn cli_multiple_goods_prints_an_inferred_tag() {
    let output = Command::new(env!("CARGO_BIN_EXE_goods-garden-cli"))
        .arg("multiple-goods")
        .output()
        .expect("the CLI multiple-goods subcommand should start");

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).expect("CLI output is UTF-8");
    assert!(stdout.contains("[INFERRED]"));
}
