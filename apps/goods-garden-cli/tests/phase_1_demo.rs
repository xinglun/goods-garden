use std::process::Command;

use goods_domain::goods::{Goods, GoodsIdentity, GoodsProfile};
use goods_domain::observation::observation::Observation;
use goods_domain::state::goods_state::HealthStatus;
use goods_infrastructure::simulator::DemoObservationSource;
use goods_runtime::GoodsRuntime;

fn goods(species: &str, individual_id: &str, expected_lifetime_hours: u32) -> Goods {
    Goods::new(
        GoodsIdentity { species: species.to_owned(), individual_id: individual_id.to_owned() },
        GoodsProfile { display_name: species.to_owned(), expected_lifetime_hours },
    )
}

#[test]
fn synthetic_fixture_produces_a_healthy_reference_state() {
    let source = DemoObservationSource::from_fixture(include_str!(
        "../../../examples/tuna-mayo/observation.example.txt"
    ))
    .expect("the bundled fixture is valid");
    let item = goods("rice-ball", "tuna-mayo-demo-001", 8);

    let state = GoodsRuntime::new(source)
        .observe_and_assess(&item)
        .expect("the synthetic observation should be available");

    assert_eq!(state.identity, item.identity);
    assert_eq!(state.observation.source, "synthetic-example");
    assert_eq!(state.expectation.max_age_hours, 8);
    assert_eq!(state.health.status, HealthStatus::Healthy);
}

#[test]
fn observation_beyond_expectation_is_unhealthy_and_explained() {
    let source = DemoObservationSource::new(Observation {
        source: "synthetic-example".to_owned(),
        observed_at: "2026-08-28T00:00:00Z".to_owned(),
        age_hours: 9,
    });
    let item = goods("rice-ball", "tuna-mayo-demo-002", 8);

    let state = GoodsRuntime::new(source)
        .observe_and_assess(&item)
        .expect("the synthetic observation should be available");

    assert_eq!(state.health.status, HealthStatus::Unhealthy);
    assert!(state.health.explanation.contains("exceeds"));
}

#[test]
fn the_same_runtime_supports_a_different_goods_profile() {
    let source = DemoObservationSource::new(Observation {
        source: "synthetic-example".to_owned(),
        observed_at: "2026-08-28T00:00:00Z".to_owned(),
        age_hours: 2,
    });
    let item = goods("coffee", "coffee-demo-001", 4);

    let state = GoodsRuntime::new(source)
        .observe_and_assess(&item)
        .expect("the synthetic observation should be available");

    assert_eq!(state.identity.species, "coffee");
    assert_eq!(state.health.status, HealthStatus::Healthy);
}

#[test]
fn cli_demo_labels_its_input_as_synthetic_and_prints_state_fields() {
    let output = Command::new(env!("CARGO_BIN_EXE_goods-garden-cli"))
        .arg("demo")
        .output()
        .expect("the CLI demo should start");

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).expect("CLI output is UTF-8");
    assert!(stdout.contains("source: synthetic-example"));
    assert!(stdout.contains("identity:"));
    assert!(stdout.contains("observation:"));
    assert!(stdout.contains("expectation:"));
    assert!(stdout.contains("health: healthy"));
}
