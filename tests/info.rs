use assert_cmd::Command;
use predicates::prelude::*;

fn cargo_bin() -> Command {
    Command::cargo_bin("awesome-skills-cli").expect("binary builds for integration tests")
}

#[test]
fn info_prints_metadata_and_embedded_markdown() {
    cargo_bin()
        .args(["info", "ab-test-setup"])
        .assert()
        .success()
        .stdout(predicate::str::contains("ID:          ab-test-setup"))
        .stdout(predicate::str::contains("Category:    marketing"))
        .stdout(predicate::str::contains("# A/B Test Setup"));
}

#[test]
fn info_skill_without_date_added_omits_line() {
    cargo_bin()
        .args(["info", "agentmail"])
        .assert()
        .success()
        .stdout(predicate::str::contains("ID:          agentmail"))
        .stdout(predicate::str::contains("Date Added").not());
}

#[test]
fn info_unknown_skill_with_typo_shows_suggestion() {
    cargo_bin()
        .args(["info", "ab-test-setp"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("Skill not found: ab-test-setp"))
        .stderr(predicate::str::contains("Did you mean"));
}

#[test]
fn unknown_skill_reports_a_clear_error_path() {
    cargo_bin()
        .args(["info", "definitely-not-a-real-skill"])
        .assert()
        .failure()
        .stderr(predicate::str::contains(
            "Skill not found: definitely-not-a-real-skill",
        ));
}

#[test]
fn info_does_not_resolve_meta_skills_from_setup_catalog() {
    cargo_bin()
        .args(["info", "awesome-skills-cli"])
        .assert()
        .failure()
        .stderr(predicate::str::contains(
            "Skill not found: awesome-skills-cli",
        ));
}
