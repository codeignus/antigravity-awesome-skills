use std::process::Command;

use serde_json::Value;

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

#[test]
fn basic_pagination_limit_5_offset_0() {
    let output = run_success(&["catalog-for-agent", "--limit", "5", "--offset", "0"]);

    let parsed: Value = serde_json::from_slice(&output.stdout).expect("valid JSON");
    let entries = parsed.as_array().expect("JSON array");
    assert_eq!(entries.len(), 5);

    let stderr = String::from_utf8_lossy(&output.stderr);
    let (total, _, _, returned, has_more) = parse_stderr_metadata(&stderr);
    assert_eq!(total, TOTAL_SKILLS.to_string());
    assert_eq!(returned, "5");
    assert_eq!(has_more, "true");
}

#[test]
fn basic_pagination_limit_1_offset_0() {
    let output = run_success(&["catalog-for-agent", "--limit", "1", "--offset", "0"]);

    let parsed: Value = serde_json::from_slice(&output.stdout).expect("valid JSON");
    let entries = parsed.as_array().expect("JSON array");
    assert_eq!(entries.len(), 1);

    let stderr = String::from_utf8_lossy(&output.stderr);
    let (total, _, _, returned, _) = parse_stderr_metadata(&stderr);
    assert_eq!(total, TOTAL_SKILLS.to_string());
    assert_eq!(returned, "1");
}

#[test]
fn basic_pagination_different_items_at_different_offsets() {
    let output0 = run_success(&["catalog-for-agent", "--limit", "2", "--offset", "0"]);
    let output5 = run_success(&["catalog-for-agent", "--limit", "2", "--offset", "5"]);

    let parsed0: Value = serde_json::from_slice(&output0.stdout).expect("valid JSON");
    let parsed5: Value = serde_json::from_slice(&output5.stdout).expect("valid JSON");

    let ids0: Vec<String> = parsed0
        .as_array()
        .unwrap()
        .iter()
        .map(|e| e["id"].as_str().unwrap().to_string())
        .collect();
    let ids5: Vec<String> = parsed5
        .as_array()
        .unwrap()
        .iter()
        .map(|e| e["id"].as_str().unwrap().to_string())
        .collect();

    assert_ne!(ids0, ids5, "items at offset 5 differ from offset 0");
}

#[test]
fn metadata_stderr_contains_total() {
    let output = run_success(&["catalog-for-agent", "--limit", "5", "--offset", "0"]);
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains(&format!("total={}", TOTAL_SKILLS)));
}

#[test]
fn metadata_returned_matches_json_entries() {
    let output = run_success(&["catalog-for-agent", "--limit", "7", "--offset", "0"]);

    let parsed: Value = serde_json::from_slice(&output.stdout).expect("valid JSON");
    let count = parsed.as_array().unwrap().len();

    let stderr = String::from_utf8_lossy(&output.stderr);
    let (_, _, _, returned, _) = parse_stderr_metadata(&stderr);
    assert_eq!(returned, count.to_string());
}

#[test]
fn metadata_offset_matches_requested() {
    let output = run_success(&["catalog-for-agent", "--limit", "3", "--offset", "42"]);
    let stderr = String::from_utf8_lossy(&output.stderr);
    let (_, offset, _, _, _) = parse_stderr_metadata(&stderr);
    assert_eq!(offset, "42");
}

#[test]
fn metadata_limit_matches_requested() {
    let output = run_success(&["catalog-for-agent", "--limit", "8", "--offset", "0"]);
    let stderr = String::from_utf8_lossy(&output.stderr);
    let (_, _, limit, _, _) = parse_stderr_metadata(&stderr);
    assert_eq!(limit, "8");
}

#[test]
fn edge_case_offset_past_total() {
    let output = run_success(&["catalog-for-agent", "--limit", "10", "--offset", "9999"]);

    let parsed: Value = serde_json::from_slice(&output.stdout).expect("valid JSON");
    let entries = parsed.as_array().expect("JSON array");
    assert_eq!(entries.len(), 0);
    assert_eq!(parsed, Value::Array(vec![]));

    let stderr = String::from_utf8_lossy(&output.stderr);
    let (_, _, _, returned, has_more) = parse_stderr_metadata(&stderr);
    assert_eq!(returned, "0");
    assert_eq!(has_more, "false");
}

#[test]
fn edge_case_limit_larger_than_remaining() {
    let offset = TOTAL_SKILLS - 1;
    let output = run_success(&[
        "catalog-for-agent",
        "--limit",
        "9999",
        "--offset",
        &offset.to_string(),
    ]);

    let parsed: Value = serde_json::from_slice(&output.stdout).expect("valid JSON");
    let entries = parsed.as_array().expect("JSON array");
    assert_eq!(entries.len(), 1);

    let stderr = String::from_utf8_lossy(&output.stderr);
    let (_, _, _, returned, has_more) = parse_stderr_metadata(&stderr);
    assert_eq!(returned, "1");
    assert_eq!(has_more, "false");
}

#[test]
fn edge_case_limit_zero() {
    let output = run(&["catalog-for-agent", "--limit", "0", "--offset", "0"]);

    if output.status.success() {
        let parsed: Value = serde_json::from_slice(&output.stdout).expect("valid JSON");
        let entries = parsed.as_array().expect("JSON array");
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
    let output = run_success(&[
        "catalog-for-agent",
        "--limit",
        "10",
        "--offset",
        &offset.to_string(),
    ]);

    let parsed: Value = serde_json::from_slice(&output.stdout).expect("valid JSON");
    let entries = parsed.as_array().expect("JSON array");
    assert_eq!(entries.len(), expected_count);

    let stderr = String::from_utf8_lossy(&output.stderr);
    let (_, _, _, returned, has_more) = parse_stderr_metadata(&stderr);
    assert_eq!(returned, expected_count.to_string());
    assert_eq!(has_more, "false");
}

#[test]
fn second_page_differs_from_first() {
    let output_first = run_success(&["catalog-for-agent", "--limit", "3", "--offset", "0"]);
    let output_second = run_success(&["catalog-for-agent", "--limit", "3", "--offset", "3"]);

    let first: Value = serde_json::from_slice(&output_first.stdout).expect("valid JSON");
    let second: Value = serde_json::from_slice(&output_second.stdout).expect("valid JSON");

    let first_id = first.as_array().unwrap()[0]["id"].as_str().unwrap();
    let second_id = second.as_array().unwrap()[0]["id"].as_str().unwrap();
    assert_ne!(first_id, second_id, "second page first item differs");
}

#[test]
fn stdout_valid_json_array_when_empty() {
    let output = run_success(&["catalog-for-agent", "--limit", "5", "--offset", "9999"]);
    let stdout_bytes = &output.stdout;
    let parsed: Value = serde_json::from_slice(stdout_bytes).expect("empty array is valid JSON");
    assert!(parsed.is_array());
    assert_eq!(parsed.as_array().unwrap().len(), 0);
}

#[test]
fn each_entry_has_exactly_four_keys() {
    let output = run_success(&["catalog-for-agent", "--limit", "10", "--offset", "0"]);
    let parsed: Value = serde_json::from_slice(&output.stdout).expect("valid JSON");

    for (i, entry) in parsed.as_array().unwrap().iter().enumerate() {
        let obj = entry
            .as_object()
            .unwrap_or_else(|| panic!("entry {i} is not an object"));
        let mut keys: Vec<&String> = obj.keys().collect();
        keys.sort();
        let expected = vec!["category", "description", "id", "risk"];
        let actual: Vec<&str> = keys.iter().map(|k| k.as_str()).collect();
        assert_eq!(actual, expected, "entry {i} keys mismatch");
    }
}

#[test]
fn stdout_contains_only_json() {
    let output = run_success(&["catalog-for-agent", "--limit", "3", "--offset", "0"]);
    let stdout = String::from_utf8_lossy(&output.stdout);
    let trimmed = stdout.trim();
    let _: Value = serde_json::from_str(trimmed).expect("trimmed stdout is valid JSON");
    assert!(!trimmed.starts_with('{'), "stdout is array not object");
    assert!(trimmed.starts_with('['), "stdout starts with [");
}

#[test]
fn stderr_metadata_line_starts_with_total() {
    let output = run_success(&["catalog-for-agent", "--limit", "1", "--offset", "0"]);
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.trim().starts_with("total="));
}

#[test]
fn error_negative_offset() {
    let output = run(&["catalog-for-agent", "--limit", "2", "--offset", "-1"]);
    assert!(!output.status.success());
}

#[test]
fn error_negative_limit() {
    let output = run(&["catalog-for-agent", "--limit", "-1", "--offset", "0"]);
    assert!(!output.status.success());
}

#[test]
fn error_missing_required_args() {
    let output = run(&["catalog-for-agent"]);
    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(!stderr.is_empty());

    let output = run(&["catalog-for-agent", "--limit", "5"]);
    assert!(!output.status.success());

    let output = run(&["catalog-for-agent", "--offset", "0"]);
    assert!(!output.status.success());
}

#[test]
fn consecutive_pages_cover_all_items_no_duplicates() {
    let limit = 500;
    let mut all_ids: Vec<String> = Vec::new();
    let mut offset = 0;

    loop {
        let output = run_success(&[
            "catalog-for-agent",
            "--limit",
            &limit.to_string(),
            "--offset",
            &offset.to_string(),
        ]);

        let parsed: Value = serde_json::from_slice(&output.stdout).expect("valid JSON");
        let entries = parsed.as_array().expect("JSON array");

        for entry in entries {
            let id = entry["id"].as_str().expect("entry has id").to_string();
            all_ids.push(id);
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
