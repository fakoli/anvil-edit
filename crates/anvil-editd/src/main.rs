#![forbid(unsafe_code)]

use std::env;
use std::process::ExitCode;

use anvil_edit_contracts::FOUNDATION_CONTRACT_VERSION;
use anvil_edit_core::FOUNDATION_STATUS;

const HELP: &str = "Anvil Edit Core foundation daemon shell

Usage: anvil-editd [--help | --version]

This scaffold starts no server, opens no editor, and dispatches no source.
";

fn main() -> ExitCode {
    let mut arguments = env::args().skip(1);

    match (arguments.next().as_deref(), arguments.next()) {
        (None | Some("--help") | Some("-h"), None) => {
            print!("{HELP}");
            ExitCode::SUCCESS
        }
        (Some("--version") | Some("-V"), None) => {
            println!(
                "anvil-editd {} (contract {}; status {FOUNDATION_STATUS})",
                env!("CARGO_PKG_VERSION"),
                FOUNDATION_CONTRACT_VERSION,
            );
            ExitCode::SUCCESS
        }
        _ => {
            eprintln!("unsupported arguments\n\n{HELP}");
            ExitCode::from(2)
        }
    }
}
