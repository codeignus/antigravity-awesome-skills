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
fn search_matches_embedded_metadata_offline() {
    let output = run_success(&["search", "ab test"]);
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("ab-test-setup"));
}

#[test]
fn search_multi_word_query_returns_results() {
    let output = run_success(&["search", "ab test marketing"]);
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("ab-test-setup"));
}

#[test]
fn search_with_no_results_prints_no_skills_message() {
    let output = run_success(&["search", "nonexistent"]);
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("No skills matching \"nonexistent\"."));
}
