use std::fs;

use assert_cmd::Command;
use tempfile::tempdir;

fn cargo_bin() -> Command {
    Command::cargo_bin("awesome-skills-cli").expect("binary builds for integration tests")
}

#[test]
fn setup_writes_all_embedded_meta_skills_to_target_path() {
    let temp_dir = tempdir().expect("temp dir for setup test");
    let cli_skill = fs::read_to_string("src/skills/awesome-skills-cli/SKILL.md")
        .expect("cli meta skill markdown available from repo");
    let recommend_skill = fs::read_to_string("src/skills/recommend-awesome-skills/SKILL.md")
        .expect("recommend meta skill markdown available from repo");

    cargo_bin()
        .args([
            "setup",
            "--path",
            temp_dir.path().to_str().expect("temp path is utf-8"),
        ])
        .assert()
        .success();

    let written_cli =
        fs::read_to_string(temp_dir.path().join("awesome-skills-cli").join("SKILL.md"))
            .expect("setup should write cli meta skill markdown");
    let written_recommend = fs::read_to_string(
        temp_dir
            .path()
            .join("recommend-awesome-skills")
            .join("SKILL.md"),
    )
    .expect("setup should write recommend meta skill markdown");

    assert_eq!(written_cli, cli_skill);
    assert_eq!(written_recommend, recommend_skill);
}

#[test]
fn setup_creates_nested_target_directory() {
    let temp_dir = tempdir().expect("temp dir for setup test");
    let nested = temp_dir.path().join("deep").join("nested").join("dir");
    assert!(!nested.exists());

    cargo_bin()
        .args([
            "setup",
            "--path",
            nested.to_str().expect("nested path is utf-8"),
        ])
        .assert()
        .success();

    assert!(nested.join("awesome-skills-cli").join("SKILL.md").exists());
    assert!(nested
        .join("recommend-awesome-skills")
        .join("SKILL.md")
        .exists());
}

#[test]
fn setup_with_skill_id_only_installs_that_meta_skill() {
    let temp_dir = tempdir().expect("temp dir for setup test");

    cargo_bin()
        .args([
            "setup",
            "awesome-skills-cli",
            "--path",
            temp_dir.path().to_str().expect("temp path is utf-8"),
        ])
        .assert()
        .success();

    assert!(temp_dir
        .path()
        .join("awesome-skills-cli")
        .join("SKILL.md")
        .exists());
    assert!(!temp_dir
        .path()
        .join("recommend-awesome-skills")
        .join("SKILL.md")
        .exists());
}

#[test]
fn setup_unknown_meta_skill_returns_failure() {
    let temp_dir = tempdir().expect("temp dir for setup test");

    cargo_bin()
        .args([
            "setup",
            "not-a-real-meta-skill",
            "--path",
            temp_dir.path().to_str().expect("temp path is utf-8"),
        ])
        .assert()
        .failure();
}
