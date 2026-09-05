use std::process::Command;

use goods_domain::goods::{Goods, GoodsIdentity, GoodsProfile};
use goods_domain::lifecycle::LifecycleState;

fn goods(species: &str, individual_id: &str) -> Goods {
    Goods::new(
        GoodsIdentity { species: species.to_owned(), individual_id: individual_id.to_owned() },
        GoodsProfile {
            display_name: species.to_owned(),
            expected_lifetime_hours: 8,
            minimum_stock_quantity: 2,
        },
    )
}

#[test]
fn new_goods_defaults_to_active() {
    let item = goods("rice-ball", "phase-10-demo-001");
    assert_eq!(item.lifecycle, LifecycleState::Active);
}

#[test]
fn retire_returns_a_retired_copy_without_mutating_the_original() {
    let item = goods("rice-ball", "phase-10-demo-002");
    let retired = item.retire();

    assert_eq!(item.lifecycle, LifecycleState::Active);
    assert_eq!(retired.lifecycle, LifecycleState::Retired);
    assert_eq!(retired.identity, item.identity);
    assert_eq!(retired.profile, item.profile);
}

#[test]
fn cli_subcommands_print_an_active_lifecycle_tag() {
    for subcommand in ["demo", "seven-day-life", "multiple-individuals", "multiple-goods"] {
        let output = Command::new(env!("CARGO_BIN_EXE_goods-garden-cli"))
            .arg(subcommand)
            .output()
            .unwrap_or_else(|_| panic!("the CLI {subcommand} subcommand should start"));

        assert!(output.status.success());
        let stdout = String::from_utf8(output.stdout).expect("CLI output is UTF-8");
        assert!(
            stdout.contains("lifecycle: active"),
            "{subcommand} stdout should contain 'lifecycle: active'"
        );
    }
}
