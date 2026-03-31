use std::process::Command;

const TOTAL_SKILLS: usize = 1329;

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

fn run(args: &[&str]) -> std::process::Output {
    let mut cmd = cargo_bin();
    cmd.args(args);
    cmd.output().expect("binary runs")
}

fn parse_stderr_metadata(stderr: &str) -> (&str, &str, &str, &str, &str) {
    let total = stderr
        .split("total=")
        .nth(1)
        .and_then(|s| s.split_whitespace().next())
        .expect("total= in stderr");
    let offset = stderr
        .split("offset=")
        .nth(1)
        .and_then(|s| s.split_whitespace().next())
        .expect("offset= in stderr");
    let limit = stderr
        .split("limit=")
        .nth(1)
        .and_then(|s| s.split_whitespace().next())
        .expect("limit= in stderr");
    let returned = stderr
        .split("returned=")
        .nth(1)
        .and_then(|s| s.split_whitespace().next())
        .expect("returned= in stderr");
    let has_more = stderr
        .split("has_more=")
        .nth(1)
        .and_then(|s| s.split_whitespace().next())
        .expect("has_more= in stderr");
    (total, offset, limit, returned, has_more)
}

fn parse_entries(stdout: &str) -> Vec<Vec<&str>> {
    stdout
        .lines()
        .skip(1)
        .map(|line| line.splitn(4, " | ").collect::<Vec<&str>>())
        .collect()
}

#[test]
fn basic_pagination_limit_5_offset_0() {
    let output = run_success(&["list", "--limit", "5", "--offset", "0"]);

    let stdout = String::from_utf8_lossy(&output.stdout);
    let entries = parse_entries(&stdout);
    assert_eq!(entries.len(), 5);

    let stderr = String::from_utf8_lossy(&output.stderr);
    let (total, _, _, returned, has_more) = parse_stderr_metadata(&stderr);
    assert_eq!(total, TOTAL_SKILLS.to_string());
    assert_eq!(returned, "5");
    assert_eq!(has_more, "true");
}

#[test]
fn basic_pagination_limit_1_offset_0() {
    let output = run_success(&["list", "--limit", "1", "--offset", "0"]);

    let stdout = String::from_utf8_lossy(&output.stdout);
    let entries = parse_entries(&stdout);
    assert_eq!(entries.len(), 1);

    let stderr = String::from_utf8_lossy(&output.stderr);
    let (total, _, _, returned, _) = parse_stderr_metadata(&stderr);
    assert_eq!(total, TOTAL_SKILLS.to_string());
    assert_eq!(returned, "1");
}

#[test]
fn basic_pagination_different_items_at_different_offsets() {
    let output0 = run_success(&["list", "--limit", "2", "--offset", "0"]);
    let output5 = run_success(&["list", "--limit", "2", "--offset", "5"]);

    let stdout0 = String::from_utf8_lossy(&output0.stdout);
    let stdout5 = String::from_utf8_lossy(&output5.stdout);

    let ids0: Vec<&str> = parse_entries(&stdout0).iter().map(|e| e[0]).collect();
    let ids5: Vec<&str> = parse_entries(&stdout5).iter().map(|e| e[0]).collect();

    assert_ne!(ids0, ids5, "items at offset 5 differ from offset 0");
}

#[test]
fn metadata_stderr_contains_total() {
    let output = run_success(&["list", "--limit", "5", "--offset", "0"]);
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains(&format!("total={}", TOTAL_SKILLS)));
}

#[test]
fn metadata_returned_matches_stdout_entries() {
    let output = run_success(&["list", "--limit", "7", "--offset", "0"]);

    let stdout = String::from_utf8_lossy(&output.stdout);
    let entries = parse_entries(&stdout);

    let stderr = String::from_utf8_lossy(&output.stderr);
    let (_, _, _, returned, _) = parse_stderr_metadata(&stderr);
    assert_eq!(returned, entries.len().to_string());
}

#[test]
fn metadata_offset_matches_requested() {
    let output = run_success(&["list", "--limit", "3", "--offset", "42"]);
    let stderr = String::from_utf8_lossy(&output.stderr);
    let (_, offset, _, _, _) = parse_stderr_metadata(&stderr);
    assert_eq!(offset, "42");
}

#[test]
fn metadata_limit_matches_requested() {
    let output = run_success(&["list", "--limit", "8", "--offset", "0"]);
    let stderr = String::from_utf8_lossy(&output.stderr);
    let (_, _, limit, _, _) = parse_stderr_metadata(&stderr);
    assert_eq!(limit, "8");
}

#[test]
fn edge_case_offset_past_total() {
    let output = run_success(&["list", "--limit", "10", "--offset", "9999"]);

    let stdout = String::from_utf8_lossy(&output.stdout);
    let entries = parse_entries(&stdout);
    assert_eq!(entries.len(), 0);

    let heading = stdout.lines().next().expect("has heading");
    assert_eq!(heading, "id | category | risk | description");

    let stderr = String::from_utf8_lossy(&output.stderr);
    let (_, _, _, returned, has_more) = parse_stderr_metadata(&stderr);
    assert_eq!(returned, "0");
    assert_eq!(has_more, "false");
}

#[test]
fn edge_case_limit_larger_than_remaining() {
    let offset = TOTAL_SKILLS - 1;
    let output = run_success(&["list", "--limit", "9999", "--offset", &offset.to_string()]);

    let stdout = String::from_utf8_lossy(&output.stdout);
    let entries = parse_entries(&stdout);
    assert_eq!(entries.len(), 1);

    let stderr = String::from_utf8_lossy(&output.stderr);
    let (_, _, _, returned, has_more) = parse_stderr_metadata(&stderr);
    assert_eq!(returned, "1");
    assert_eq!(has_more, "false");
}

#[test]
fn edge_case_limit_zero() {
    let output = run(&["list", "--limit", "0", "--offset", "0"]);

    if output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        let entries = parse_entries(&stdout);
        assert_eq!(entries.len(), 0);

        let stderr = String::from_utf8_lossy(&output.stderr);
        let (_, _, _, returned, _) = parse_stderr_metadata(&stderr);
        assert_eq!(returned, "0");
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        assert!(
            stderr.contains("invalid") || stderr.contains("error"),
            "should explain why limit=0 is rejected"
        );
    }
}

#[test]
fn edge_case_large_offset_near_total() {
    let offset = TOTAL_SKILLS - 4;
    let expected_count = TOTAL_SKILLS - offset;
    let output = run_success(&["list", "--limit", "10", "--offset", &offset.to_string()]);

    let stdout = String::from_utf8_lossy(&output.stdout);
    let entries = parse_entries(&stdout);
    assert_eq!(entries.len(), expected_count);

    let stderr = String::from_utf8_lossy(&output.stderr);
    let (_, _, _, returned, has_more) = parse_stderr_metadata(&stderr);
    assert_eq!(returned, expected_count.to_string());
    assert_eq!(has_more, "false");
}

#[test]
fn second_page_differs_from_first() {
    let output_first = run_success(&["list", "--limit", "3", "--offset", "0"]);
    let output_second = run_success(&["list", "--limit", "3", "--offset", "3"]);

    let stdout_first = String::from_utf8_lossy(&output_first.stdout);
    let stdout_second = String::from_utf8_lossy(&output_second.stdout);

    let first_entries = parse_entries(&stdout_first);
    let second_entries = parse_entries(&stdout_second);

    assert_ne!(
        first_entries[0][0], second_entries[0][0],
        "second page first item differs"
    );
}

#[test]
fn stdout_heading_row_when_empty() {
    let output = run_success(&["list", "--limit", "5", "--offset", "9999"]);
    let stdout = String::from_utf8_lossy(&output.stdout);
    let lines: Vec<&str> = stdout.lines().collect();
    assert_eq!(lines.len(), 1, "only heading row");
    assert_eq!(lines[0], "id | category | risk | description");
}

#[test]
fn each_entry_has_four_fields() {
    let output = run_success(&["list", "--limit", "10", "--offset", "0"]);
    let stdout = String::from_utf8_lossy(&output.stdout);

    for (i, entry) in parse_entries(&stdout).iter().enumerate() {
        assert_eq!(
            entry.len(),
            4,
            "entry {i} should have 4 fields (id, category, risk, description)"
        );
    }
}

#[test]
fn stderr_metadata_line_starts_with_total() {
    let output = run_success(&["list", "--limit", "1", "--offset", "0"]);
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.trim().starts_with("total="));
}

#[test]
fn error_negative_offset() {
    let output = run(&["list", "--limit", "2", "--offset", "-1"]);
    assert!(!output.status.success());
}

#[test]
fn error_negative_limit() {
    let output = run(&["list", "--limit", "-1", "--offset", "0"]);
    assert!(!output.status.success());
}

#[test]
fn catalog_for_agent_command_rejected() {
    let output = run(&["catalog-for-agent", "--limit", "5", "--offset", "0"]);
    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("unrecognized") || stderr.contains("not found") || stderr.contains("error"),
        "catalog-for-agent should be rejected: {stderr}"
    );
}

#[test]
fn consecutive_pages_cover_all_items_no_duplicates() {
    let limit = 500;
    let mut all_ids: Vec<String> = Vec::new();
    let mut offset = 0;

    loop {
        let output = run_success(&[
            "list",
            "--limit",
            &limit.to_string(),
            "--offset",
            &offset.to_string(),
        ]);

        let stdout = String::from_utf8_lossy(&output.stdout);
        let entries = parse_entries(&stdout);

        for entry in &entries {
            all_ids.push(entry[0].to_string());
        }

        let stderr = String::from_utf8_lossy(&output.stderr);
        let (_, _, _, _, has_more) = parse_stderr_metadata(&stderr);

        if has_more != "true" {
            break;
        }
        offset += limit;
    }

    assert_eq!(
        all_ids.len(),
        TOTAL_SKILLS,
        "total entries across all pages"
    );

    let mut unique = all_ids.clone();
    unique.sort();
    unique.dedup();
    assert_eq!(unique.len(), TOTAL_SKILLS, "no duplicate IDs");
}
