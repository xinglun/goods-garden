use std::fs;
use std::process::Command;
use std::time::Duration;

use goods_domain::care::{Caregiver, HumanFeedback};
use goods_domain::goods::{Goods, GoodsIdentity, GoodsProfile};
use goods_domain::memory::GoodsMemory;
use goods_domain::observation::Observation;
use goods_runtime::scheduler::{self, ScheduledCycle, StopReason};

fn goods(individual_id: &str) -> Goods {
    Goods::new(
        GoodsIdentity { species: "rice-ball".to_owned(), individual_id: individual_id.to_owned() },
        GoodsProfile {
            display_name: "rice-ball".to_owned(),
            expected_lifetime_hours: 8,
            minimum_stock_quantity: 2,
        },
    )
}

fn script(len: usize) -> Vec<ScheduledCycle> {
    (0..len)
        .map(|day| ScheduledCycle {
            label: "day",
            observation: Observation {
                source: "synthetic-example".to_owned(),
                observed_at: format!("day {day} (synthetic)"),
                age_hours: 2,
                quantity_on_hand: 6,
            },
            feedback: HumanFeedback {
                caregiver: Caregiver {
                    role: "store staff".to_owned(),
                    display_name: "Demo Staff".to_owned(),
                },
                decision: "N/A (no Care Request today)".to_owned(),
                provided_at: format!("day {day} (synthetic)"),
            },
        })
        .collect()
}

#[test]
fn stops_at_the_safety_cap_before_the_script_is_exhausted() {
    let item = goods("phase-12-demo-001");
    let script = script(7);
    let mut memory = GoodsMemory::new();
    let mut cycles_run = 0;

    let stop_reason = scheduler::run_scheduled(
        &item,
        &script,
        5,
        Duration::from_millis(0),
        &mut memory,
        |_, _, _| cycles_run += 1,
    )
    .expect("the synthetic script should be available");

    assert_eq!(stop_reason, StopReason::MaxCyclesReached);
    assert_eq!(cycles_run, 5);
}

#[test]
fn exhausts_the_script_when_the_cap_is_not_reached() {
    let item = goods("phase-12-demo-002");
    let script = script(3);
    let mut memory = GoodsMemory::new();
    let mut cycles_run = 0;

    let stop_reason = scheduler::run_scheduled(
        &item,
        &script,
        5,
        Duration::from_millis(0),
        &mut memory,
        |_, _, _| cycles_run += 1,
    )
    .expect("the synthetic script should be available");

    assert_eq!(stop_reason, StopReason::ScriptExhausted);
    assert_eq!(cycles_run, 3);
}

#[test]
fn cli_scheduled_seven_day_life_stops_at_the_safety_cap_and_logs_to_a_file() {
    let log_path = std::env::temp_dir().join("goods-garden-scheduler.log");
    let _ = fs::remove_file(&log_path);

    let output = Command::new(env!("CARGO_BIN_EXE_goods-garden-cli"))
        .arg("scheduled-seven-day-life")
        .output()
        .expect("the scheduled-seven-day-life subcommand should start");

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).expect("CLI output is UTF-8");

    assert!(stdout.contains("Day 1 (normal)"));
    assert!(stdout.contains("Day 5 (normal)"));
    assert!(!stdout.contains("Day 6 (anomaly: stock availability)"));
    assert!(!stdout.contains("Day 7 (verification + restock)"));
    assert!(stdout.contains("Stopped: safety cap reached (5 cycle(s))"));
    assert!(stdout.contains("No progress is persisted between runs"));
    assert!(stdout.contains("log written to:"));

    let log_contents =
        fs::read_to_string(&log_path).expect("the log file should have been written");
    assert!(log_contents.contains("Day 1 (normal)"));
    assert!(log_contents.contains("Day 5 (normal)"));
    assert!(!log_contents.contains("Day 6 (anomaly: stock availability)"));

    let _ = fs::remove_file(&log_path);
}
