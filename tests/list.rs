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
    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    output
}

#[test]
fn list_has_heading_row_in_stdout() {
    let output = run_success(&["list"]);
    let stdout = String::from_utf8_lossy(&output.stdout);
    let first_line = stdout.lines().next().expect("stdout has at least one line");
    assert_eq!(first_line, "id | category | risk | description");
}

#[test]
fn list_has_entries_in_stdout() {
    let output = run_success(&["list"]);
    let stdout = String::from_utf8_lossy(&output.stdout);
    let lines: Vec<&str> = stdout.lines().collect();
    assert!(lines.len() > 1, "heading + at least one entry");
    assert!(stdout.contains("ab-test-setup"));
}

#[test]
fn list_category_filter_returns_matching_skills() {
    let output = run_success(&["list", "--category", "marketing"]);
    let stdout = String::from_utf8_lossy(&output.stdout);
    let lines: Vec<&str> = stdout.lines().collect();
    assert!(lines.len() > 1);
    for line in &lines[1..] {
        let parts: Vec<&str> = line.splitn(4, " | ").collect();
        assert!(parts.len() >= 2, "line has category field: {line}");
        assert_eq!(parts[1], "marketing", "all entries match category: {line}");
    }
}

#[test]
fn list_category_nonexistent_prints_heading_and_metadata() {
    let output = run_success(&["list", "--category", "nonexistent"]);
    let stdout = String::from_utf8_lossy(&output.stdout);
    let first_line = stdout.lines().next().expect("stdout has heading");
    assert_eq!(first_line, "id | category | risk | description");

    let lines: Vec<&str> = stdout.lines().collect();
    assert_eq!(lines.len(), 1, "only heading row, no entries");

    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("returned=0"));
    assert!(stderr.contains("has_more=false"));
}

#[test]
fn list_stderr_contains_metadata() {
    let output = run_success(&["list"]);
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("total="));
    assert!(stderr.contains("offset="));
    assert!(stderr.contains("limit="));
    assert!(stderr.contains("returned="));
    assert!(stderr.contains("has_more="));
}
