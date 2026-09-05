use std::process::Command;

#[test]
fn multiple_goods_processes_four_species_with_no_product_specific_branch() {
    let output = Command::new(env!("CARGO_BIN_EXE_goods-garden-cli"))
        .arg("multiple-goods")
        .output()
        .expect("the multiple-goods subcommand should start");

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).expect("CLI output is UTF-8");

    assert!(stdout.contains("Goods Garden Multiple Goods demo (synthetic-example)"));

    // All four species from the phase document are present.
    assert!(stdout.contains("Salmon rice ball (rice-ball-salmon)"));
    assert!(stdout.contains("Iced coffee (coffee)"));
    assert!(stdout.contains("Egg sandwich (sandwich)"));
    assert!(stdout.contains("Chicken bento (bento)"));

    // Coffee (shortest expected lifetime) raises a Need/Care episode; the
    // others stay healthy.
    assert!(stdout.contains("need: FreshnessConcern"));
    assert!(stdout.matches("needs: <none identified>").count() == 3);

    // Explicit capability-vs-instance statement.
    assert!(stdout.contains("Capability summary"));
    assert!(
        stdout.contains(
            "4 distinct product species were processed by the identical GoodsRuntime call"
        )
    );
    assert!(stdout.contains("No product-specific code branch was needed"));
}

#[test]
fn the_existing_subcommands_are_unchanged() {
    for (arg, marker) in [
        ("demo", "Goods Garden Phase 1 demo"),
        ("seven-day-life", "Goods Garden Seven Day Life demo (synthetic-example)"),
        ("multiple-individuals", "Goods Garden Multiple Individuals demo (synthetic-example)"),
    ] {
        let output = Command::new(env!("CARGO_BIN_EXE_goods-garden-cli"))
            .arg(arg)
            .output()
            .unwrap_or_else(|_| panic!("the {arg} subcommand should start"));
        assert!(output.status.success());
        let stdout = String::from_utf8(output.stdout).expect("CLI output is UTF-8");
        assert!(stdout.contains(marker));
    }
}
