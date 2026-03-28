use assert_cmd::Command;
use predicates::prelude::*;

fn cargo_bin() -> Command {
    Command::cargo_bin("awesome-skills-cli").expect("binary builds for integration tests")
}

#[test]
fn list_reads_embedded_skill_catalog() {
    cargo_bin()
        .arg("list")
        .assert()
        .success()
        .stdout(predicate::str::contains("ab-test-setup"))
        .stdout(predicate::str::contains("marketing"));
}

#[test]
fn list_with_category_filter_returns_matching_skills() {
    cargo_bin()
        .args(["list", "--category", "marketing"])
        .assert()
        .success()
        .stdout(predicate::str::contains("ab-test-setup"))
        .stdout(predicate::str::contains("45 skills total."));
}

#[test]
fn list_with_empty_category_prints_not_found_message() {
    cargo_bin()
        .args(["list", "--category", "nonexistent-category"])
        .assert()
        .success()
        .stdout(predicate::str::contains(
            "No skills found in category \"nonexistent-category\"",
        ));
}
