use assert_cmd::Command;
use predicates::prelude::*;

fn cargo_bin() -> Command {
    Command::cargo_bin("awesome-skills-cli").expect("binary builds for integration tests")
}

#[test]
fn version_prints_the_binary_version() {
    cargo_bin()
        .arg("version")
        .assert()
        .success()
        .stdout(predicate::str::contains(env!("CARGO_PKG_VERSION")));
}

#[test]
fn help_mentions_offline_commands() {
    cargo_bin()
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("list"))
        .stdout(predicate::str::contains("search"))
        .stdout(predicate::str::contains("catalog-for-agent"))
        .stdout(predicate::str::contains("info"))
        .stdout(predicate::str::contains("add"))
        .stdout(predicate::str::contains("setup"))
        .stdout(predicate::str::contains("version"));
}

#[test]
fn unknown_subcommand_returns_error() {
    cargo_bin()
        .arg("not-a-real-subcommand")
        .assert()
        .failure()
        .stderr(predicate::str::contains("error").or(predicate::str::contains("unrecognized")));
}
