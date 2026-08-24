use std::process::Command;

fn daemon(arguments: &[&str]) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_anvil-editd"))
        .args(arguments)
        .output()
        .expect("foundation daemon shell starts")
}

#[test]
fn help_is_non_serving_and_successful() {
    let output = daemon(&["--help"]);
    let stdout = String::from_utf8(output.stdout).expect("help is UTF-8");

    assert!(output.status.success());
    assert!(stdout.contains("starts no server"));
    assert!(output.stderr.is_empty());
}

#[test]
fn version_reports_foundation_status() {
    let output = daemon(&["--version"]);
    let stdout = String::from_utf8(output.stdout).expect("version is UTF-8");

    assert!(output.status.success());
    assert!(stdout.contains("contract 0.2"));
    assert!(stdout.contains("configuration-pinning-and-revision-fence-primitives"));
}

#[test]
fn unknown_arguments_fail_without_starting_work() {
    let output = daemon(&["--serve"]);
    let stderr = String::from_utf8(output.stderr).expect("error is UTF-8");

    assert_eq!(output.status.code(), Some(2));
    assert!(stderr.contains("unsupported arguments"));
}
