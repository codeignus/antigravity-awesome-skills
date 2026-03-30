use std::process::Command;

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
fn info_does_not_resolve_meta_skills_from_setup_catalog() {
    let output = run_success(&["info", "ab-test-setup"]);
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("ab-test-setup"));
}

#[test]
fn info_prints_metadata_and_embedded_markdown() {
    let output = run_success(&["info", "ab-test-setup"]);
    let stderr = String::from_utf8_lossy(&output.stderr);
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stderr.contains("ID:"));
    assert!(stderr.contains("Name:"));
    assert!(stderr.contains("Category:"));
    assert!(stderr.contains("Description:"));
    assert!(stderr.contains("Risk:"));
    assert!(stderr.contains("Source:"));
    assert!(stderr.contains("Date added:"));
    assert!(stdout.contains("ab-test-setup"));
}

#[test]
fn info_skill_without_date_added_omits_line() {
    let output = run_success(&["info", "agentmail"]);
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(!stderr.contains("Date added:"), "stderr: {stderr}");
}

#[test]
fn info_unknown_skill_with_typo_shows_suggestion() {
    let output = run(&["info", "unknown"]);
    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("not found"), "stderr: {stderr}");
}
