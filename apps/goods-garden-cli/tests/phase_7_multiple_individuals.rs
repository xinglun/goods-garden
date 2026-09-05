use std::process::Command;

#[test]
fn multiple_individuals_share_species_but_keep_memory_separate() {
    let output = Command::new(env!("CARGO_BIN_EXE_goods-garden-cli"))
        .arg("multiple-individuals")
        .output()
        .expect("the multiple-individuals subcommand should start");

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).expect("CLI output is UTF-8");

    assert!(stdout.contains("Goods Garden Multiple Individuals demo (synthetic-example)"));
    assert!(stdout.contains("species: rice-ball"));

    // Two distinct individuals of the same species.
    assert!(stdout.contains("TunaMayo@StoreA"));
    assert!(stdout.contains("TunaMayo@StoreB"));

    // Store A has a Need/Care episode; Store B does not.
    assert!(stdout.contains("need: FreshnessConcern"));
    assert!(stdout.contains("memory: 1 care episode(s) remembered for TunaMayo@StoreA"));
    assert!(stdout.contains("memory: 0 care episode(s) remembered for TunaMayo@StoreB"));

    // Explicit separation statement.
    assert!(stdout.contains("Separation summary"));
    assert!(stdout.contains("neither individual's memory contains the other's records"));
}

#[test]
fn the_existing_demo_and_seven_day_life_subcommands_are_unchanged() {
    let demo = Command::new(env!("CARGO_BIN_EXE_goods-garden-cli"))
        .arg("demo")
        .output()
        .expect("the demo subcommand should start");
    assert!(demo.status.success());
    let demo_stdout = String::from_utf8(demo.stdout).expect("CLI output is UTF-8");
    assert!(demo_stdout.contains("Goods Garden Phase 1 demo"));

    let seven_day = Command::new(env!("CARGO_BIN_EXE_goods-garden-cli"))
        .arg("seven-day-life")
        .output()
        .expect("the seven-day-life subcommand should start");
    assert!(seven_day.status.success());
    let seven_day_stdout = String::from_utf8(seven_day.stdout).expect("CLI output is UTF-8");
    assert!(seven_day_stdout.contains("Goods Garden Seven Day Life demo (synthetic-example)"));
}
