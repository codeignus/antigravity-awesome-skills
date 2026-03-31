use std::process::Command;

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

#[test]
fn version_prints_the_binary_version() {
    let output = run_success(&["version"]);
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains(env!("CARGO_PKG_VERSION")));
}

#[test]
fn help_mentions_offline_commands() {
    let output = run_success(&["--help"]);
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("list"));
    assert!(stdout.contains("search"));
    assert!(!stdout.contains("catalog-for-agent"));
    assert!(stdout.contains("info"));
    assert!(stdout.contains("add"));
    assert!(stdout.contains("setup"));
    assert!(stdout.contains("version"));
}

#[test]
fn unknown_subcommand_returns_error() {
    let output = run(&["not-a-real-subcommand"]);
    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("error") || stderr.contains("unrecognized"),
        "stderr: {stderr}"
    );
}
