use assert_cmd::Command;
use predicates::prelude::*;

fn cargo_bin() -> Command {
    Command::cargo_bin("awesome-skills-cli").expect("binary builds for integration tests")
}

#[test]
fn legacy_install_subcommand_is_rejected_after_the_rename() {
    cargo_bin()
        .args(["install", "ab-test-setup", "--path", "/tmp/skills"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("add"));
}
