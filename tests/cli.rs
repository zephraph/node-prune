use assert_cmd::prelude::*;
use node_prune::Stats;
use predicates::str;
use serde_json::json;
use std::process::Command;

#[test]
fn dir_not_exist() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("node-prune")?;
    cmd.arg("-p")
        .arg("node_modules")
        .assert()
        .stderr(str::contains("access node_modules error"));
    Ok(())
}

#[test]
fn dir_is_empty() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("node-prune")?;
    let stats: Stats = Default::default();
    let stats_json = format!("{}", json!(&stats));
    cmd.arg("-p")
        .arg("benches")
        .assert()
        .stdout(str::contains(&stats_json));
    Ok(())
}
