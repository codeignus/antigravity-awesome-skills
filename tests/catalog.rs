use std::process::Command;

use serde_json::Value;

fn cargo_bin() -> Command {
    let mut cmd = Command::new(env!("CARGO_BIN_EXE_awesome-skills-cli"));
    cmd.env_clear();
    cmd
}

fn run_success(args: &[&str]) -> std::process::Output {
    let mut cmd = cargo_bin();
    cmd.args(args);
    let output = cmd.output().expect("binary runs");
    assert!(output.status.success(), "stderr: {}", String::from_utf8_lossy(&output.stderr));
    output
}

fn run(args: &[&str]) -> std::process::Output {
    let mut cmd = cargo_bin();
    cmd.args(args);
    cmd.output().expect("binary runs")
}

#[test]
fn catalog_for_agent_emits_condensed_json_array() {
    let output = run_success(&["catalog-for-agent", "--limit", "100", "--offset", "0"]);

    let parsed: Value =
        serde_json::from_slice(&output.stdout).expect("catalog-for-agent should emit JSON");
    let entries = parsed
        .as_array()
        .expect("catalog-for-agent should emit a JSON array");
    let skill = entries
        .iter()
        .find(|entry: &&Value| entry.get("id") == Some(&Value::String("ab-test-setup".to_string())))
        .expect("catalog includes a known skill");

    let object = skill
        .as_object()
        .expect("catalog entries are JSON objects");

    assert!(object.len() == 4, "catalog entries stay condensed");
}

#[test]
fn catalog_for_agent_rejects_negative_offset() {
    let output = run(&["catalog-for-agent", "--limit", "2", "--offset", "-1"]);
    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("unexpected argument") || stderr.contains("invalid"));
}
