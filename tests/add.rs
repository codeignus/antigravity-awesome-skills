use std::fs;

use assert_cmd::Command;
use predicates::prelude::*;
use tempfile::tempdir;

fn cargo_bin() -> Command {
    Command::cargo_bin("awesome-skills-cli").expect("binary builds for integration tests")
}

#[test]
fn add_writes_embedded_skill_markdown_to_target_path() {
    let temp_dir = tempdir().expect("temp dir for add test");
    let expected = fs::read_to_string("skills/ab-test-setup/SKILL.md")
        .expect("fixture skill markdown available from repo");

    cargo_bin()
        .args([
            "add",
            "ab-test-setup",
            "--path",
            temp_dir.path().to_str().expect("temp path is utf-8"),
        ])
        .assert()
        .success();

    let written = fs::read_to_string(temp_dir.path().join("ab-test-setup").join("SKILL.md"))
        .expect("add should write embedded skill markdown");

    assert_eq!(written, expected);
}

#[test]
fn add_multiple_skills_to_same_path() {
    let temp_dir = tempdir().expect("temp dir for add test");

    cargo_bin()
        .args([
            "add",
            "ab-test-setup",
            "agentmail",
            "--path",
            temp_dir.path().to_str().expect("temp path is utf-8"),
        ])
        .assert()
        .success();

    assert!(temp_dir
        .path()
        .join("ab-test-setup")
        .join("SKILL.md")
        .exists());
    assert!(temp_dir.path().join("agentmail").join("SKILL.md").exists());
}

#[test]
fn add_overwrites_existing_skill() {
    let temp_dir = tempdir().expect("temp dir for add test");

    cargo_bin()
        .args([
            "add",
            "ab-test-setup",
            "--path",
            temp_dir.path().to_str().expect("temp path is utf-8"),
        ])
        .assert()
        .success();

    let first = fs::read_to_string(temp_dir.path().join("ab-test-setup").join("SKILL.md"))
        .expect("first write");

    cargo_bin()
        .args([
            "add",
            "ab-test-setup",
            "--path",
            temp_dir.path().to_str().expect("temp path is utf-8"),
        ])
        .assert()
        .success();

    let second = fs::read_to_string(temp_dir.path().join("ab-test-setup").join("SKILL.md"))
        .expect("second write");

    assert_eq!(first, second, "overwrite produces identical content");
}

#[test]
fn add_creates_nested_target_directory() {
    let temp_dir = tempdir().expect("temp dir for add test");
    let nested = temp_dir.path().join("deep").join("nested").join("dir");
    assert!(!nested.exists());

    cargo_bin()
        .args([
            "add",
            "ab-test-setup",
            "--path",
            nested.to_str().expect("nested path is utf-8"),
        ])
        .assert()
        .success();

    assert!(nested.join("ab-test-setup").join("SKILL.md").exists());
}

#[test]
fn add_all_unknown_batch_reports_all_errors() {
    let temp_dir = tempdir().expect("temp dir for add test");

    cargo_bin()
        .args([
            "add",
            "fake-skill-a",
            "fake-skill-b",
            "--path",
            temp_dir.path().to_str().expect("temp path is utf-8"),
        ])
        .assert()
        .failure()
        .stderr(predicate::str::contains("Skill not found: fake-skill-a"))
        .stderr(predicate::str::contains("Skill not found: fake-skill-b"));
}

#[test]
fn add_continues_processing_after_an_unknown_skill_and_returns_failure() {
    let temp_dir = tempdir().expect("temp dir for add test");

    cargo_bin()
        .args([
            "add",
            "ab-test-setup",
            "definitely-not-a-real-skill",
            "finishing-a-branch",
            "--path",
            temp_dir.path().to_str().expect("temp path is utf-8"),
        ])
        .assert()
        .failure()
        .stderr(predicate::str::contains(
            "Skill not found: definitely-not-a-real-skill",
        ));

    assert!(temp_dir
        .path()
        .join("ab-test-setup")
        .join("SKILL.md")
        .exists());
    assert!(temp_dir
        .path()
        .join("finishing-a-development-branch")
        .join("SKILL.md")
        .exists());
}

#[test]
fn add_via_alias_resolves_canonical_id() {
    let temp_dir = tempdir().expect("temp dir for add test");

    cargo_bin()
        .args([
            "add",
            "finishing-a-branch",
            "--path",
            temp_dir.path().to_str().expect("temp path is utf-8"),
        ])
        .assert()
        .success();

    assert!(temp_dir
        .path()
        .join("finishing-a-development-branch")
        .join("SKILL.md")
        .exists());
}
