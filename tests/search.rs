use assert_cmd::Command;
use predicates::prelude::*;

fn cargo_bin() -> Command {
    Command::cargo_bin("awesome-skills-cli").expect("binary builds for integration tests")
}

#[test]
fn search_matches_embedded_metadata_offline() {
    cargo_bin()
        .args(["search", "A/B tests"])
        .assert()
        .success()
        .stdout(predicate::str::contains("ab-test-setup"));
}

#[test]
fn search_with_no_results_prints_no_skills_message() {
    cargo_bin()
        .args(["search", "zzzzz-no-skill-should-match-this-xyzzy"])
        .assert()
        .success()
        .stdout(predicate::str::contains("No skills matching"));
}

#[test]
fn search_multi_word_query_returns_results() {
    cargo_bin()
        .args(["search", "A/B test setup"])
        .assert()
        .success()
        .stdout(predicate::str::contains("ab-test-setup"));
}
