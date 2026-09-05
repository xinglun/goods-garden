use std::process::Command;

#[test]
fn seven_day_life_narrates_anomaly_care_verification_and_memory() {
    let output = Command::new(env!("CARGO_BIN_EXE_goods-garden-cli"))
        .arg("seven-day-life")
        .output()
        .expect("the seven-day-life subcommand should start");

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).expect("CLI output is UTF-8");

    assert!(stdout.contains("Goods Garden Seven Day Life demo (synthetic-example)"));

    // Normal days.
    assert!(stdout.contains("Day 1 (normal)"));
    assert!(stdout.contains("Day 2 (normal)"));
    assert!(stdout.contains("Day 5 (normal)"));

    // Anomaly, investigation, Care Request and Human Feedback.
    assert!(stdout.contains("Day 3 (anomaly: freshness)"));
    assert!(stdout.contains("need: FreshnessConcern"));
    assert!(stdout.contains("care request: (store staff)"));
    assert!(stdout.contains("restocked with fresh units"));
    assert!(stdout.contains("Day 6 (anomaly: stock availability)"));
    assert!(stdout.contains("need: StockAvailabilityConcern"));
    assert!(stdout.contains("Placed a restock order"));

    // Follow-up verification and Learning.
    assert!(stdout.contains("outcome (follow-up on prior Care Action): Resolved"));
    assert!(stdout.matches("learning: This Care Action appears to have resolved").count() >= 2);

    // Final memory tally.
    assert!(stdout.contains("Week summary"));
    assert!(stdout.contains("2 care episode(s) remembered across seven synthetic days."));
}

#[test]
fn the_existing_demo_subcommand_is_unchanged() {
    let output = Command::new(env!("CARGO_BIN_EXE_goods-garden-cli"))
        .arg("demo")
        .output()
        .expect("the demo subcommand should start");

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).expect("CLI output is UTF-8");
    assert!(stdout.contains("Goods Garden Phase 1 demo"));
    assert!(stdout.contains("health: healthy"));
    assert!(stdout.contains("learning: <none pending>"));
}
