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

#[test]
fn list_reads_embedded_skill_catalog() {
    let output = run_success(&["list"]);
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("ab-test-setup"));
}

#[test]
fn list_with_category_filter_returns_matching_skills() {
    let output = run_success(&["list", "--category", "marketing"]);
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("ab-test-setup"));
}

#[test]
fn list_with_empty_category_prints_not_found_message() {
    let output = run_success(&["list", "--category", "nonexistent"]);
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("No skills found in category \"nonexistent\"."));
}
