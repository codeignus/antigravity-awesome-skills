use std::process::{Command, Stdio};

fn cargo_bin() -> Command {
    let mut cmd = Command::new(env!("CARGO_BIN_EXE_awesome-skills-cli"));
    cmd.env_clear();
    cmd
}

#[test]
fn list_exits_cleanly_when_stdout_pipe_closes_early() {
    let mut child = cargo_bin()
        .arg("list")
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("binary starts");

    drop(child.stdout.take());

    let output = child.wait_with_output().expect("process exits");

    assert!(output.status.success());

    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(!stderr.contains("Broken pipe"));
}
