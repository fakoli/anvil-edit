#![forbid(unsafe_code)]

use std::env;
use std::path::{Path, PathBuf};
use std::process::{Command, ExitCode, Stdio};

const USAGE: &str = "Usage: cargo xtask <check|rust|guidance>";

fn main() -> ExitCode {
    let result = match env::args().nth(1).as_deref() {
        Some("check") => check(),
        Some("rust") => check_rust(),
        Some("guidance") => check_guidance(),
        _ => {
            eprintln!("{USAGE}");
            return ExitCode::from(2);
        }
    };

    match result {
        Ok(()) => ExitCode::SUCCESS,
        Err(message) => {
            eprintln!("xtask failed: {message}");
            ExitCode::FAILURE
        }
    }
}

fn check() -> Result<(), String> {
    check_rust()?;
    check_guidance()
}

fn check_rust() -> Result<(), String> {
    let cargo = env::var("CARGO").unwrap_or_else(|_| "cargo".to_owned());
    let root = workspace_root();

    run(&cargo, &["fmt", "--all", "--check"], &root, &[])?;
    run(
        &cargo,
        &["check", "--workspace", "--all-targets", "--locked"],
        &root,
        &[],
    )?;
    run(
        &cargo,
        &[
            "clippy",
            "--workspace",
            "--all-targets",
            "--locked",
            "--",
            "-D",
            "warnings",
        ],
        &root,
        &[],
    )?;
    run(&cargo, &["test", "--workspace", "--locked"], &root, &[])?;
    run(
        &cargo,
        &["doc", "--workspace", "--no-deps", "--locked"],
        &root,
        &[("RUSTDOCFLAGS", "-D warnings")],
    )
}

fn check_guidance() -> Result<(), String> {
    let root = workspace_root();
    let python = find_python(&root)?;

    run(
        &python,
        &[
            "-m",
            "unittest",
            "discover",
            "-s",
            "plugins/anvil-edit-development/tests",
            "-p",
            "test_*.py",
        ],
        &root,
        &[],
    )?;
    run(
        &python,
        &["plugins/anvil-edit-development/scripts/validate_guidance.py"],
        &root,
        &[],
    )
}

fn workspace_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("xtask is a direct workspace child")
        .to_path_buf()
}

fn find_python(root: &Path) -> Result<String, String> {
    let candidates = env::var("PYTHON")
        .into_iter()
        .chain(["python3".to_owned(), "python".to_owned()]);

    for candidate in candidates {
        let found = Command::new(&candidate)
            .arg("--version")
            .current_dir(root)
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status()
            .is_ok_and(|status| status.success());
        if found {
            return Ok(candidate);
        }
    }

    Err("no Python interpreter found; set PYTHON to run guidance checks".to_owned())
}

fn run(
    program: &str,
    arguments: &[&str],
    root: &Path,
    environment: &[(&str, &str)],
) -> Result<(), String> {
    println!("+ {program} {}", arguments.join(" "));
    let status = Command::new(program)
        .args(arguments)
        .current_dir(root)
        .envs(environment.iter().copied())
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status()
        .map_err(|error| format!("could not start {program}: {error}"))?;

    if status.success() {
        Ok(())
    } else {
        Err(format!("{program} exited with {status}"))
    }
}
