use std::fs;
use std::process::Command;

use tempfile::tempdir;

fn cargo_bin() -> Command {
    let mut cmd = Command::new(env!("CARGO_BIN_EXE_awesome-skills-cli"));
    cmd.env_clear();
    cmd
}

fn run_success(args: &[&str]) {
    let mut cmd = cargo_bin();
    cmd.args(args);
    let output = cmd.output().expect("binary runs");
    assert!(output.status.success(), "stderr: {}", String::from_utf8_lossy(&output.stderr));
}

#[test]
fn setup_writes_all_embedded_meta_skills_to_target_path() {
    let temp_dir = tempdir().expect("temp dir for setup test");
    let cli_skill = fs::read_to_string("src/skills/awesome-skills-cli/SKILL.md")
        .expect("cli meta skill markdown available from repo");
    let recommend_skill = fs::read_to_string("src/skills/recommend-awesome-skills/SKILL.md")
        .expect("recommend meta skill markdown available from repo");

    run_success(&[
        "setup",
        "--path",
        temp_dir.path().to_str().expect("temp path is utf-8"),
    ]);

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

    run_success(&[
        "setup",
        "--path",
        nested.to_str().expect("nested path is utf-8"),
    ]);

    assert!(nested.join("awesome-skills-cli").join("SKILL.md").exists());
    assert!(
        nested
            .join("recommend-awesome-skills")
            .join("SKILL.md")
            .exists()
    );
}

#[test]
fn setup_with_skill_id_only_installs_that_meta_skill() {
    let temp_dir = tempdir().expect("temp dir for setup test");

    run_success(&[
        "setup",
        "awesome-skills-cli",
        "--path",
        temp_dir.path().to_str().expect("temp path is utf-8"),
    ]);

    assert!(
        temp_dir
            .path()
            .join("awesome-skills-cli")
            .join("SKILL.md")
            .exists()
    );
    assert!(
        !temp_dir
            .path()
            .join("recommend-awesome-skills")
            .join("SKILL.md")
            .exists()
    );
}

#[test]
fn setup_unknown_meta_skill_returns_failure() {
    let temp_dir = tempdir().expect("temp dir for setup test");

    let mut cmd = cargo_bin();
    cmd.args([
        "setup",
        "not-a-real-meta-skill",
        "--path",
        temp_dir.path().to_str().expect("temp path is utf-8"),
    ]);
    let output = cmd.output().expect("binary runs");
    assert!(!output.status.success());
}
