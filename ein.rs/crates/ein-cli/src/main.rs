//! `ein` — the drop-in binary.
//!
//! A stub until [P1a.5](../../../plans/m1a_rust/p1a.5_presentation/README.md):
//! it answers `--version` and exits 2 on every subcommand, so the workspace
//! builds a binary named `ein` from day one without ever *looking* like a
//! working engine. Exit 2 is `argparse`'s own "usage error" code, which is
//! what the real CLI returns for an unrecognised invocation.
//!
//! It is deliberately not installed onto `$PATH` during the port — the
//! conformance harness addresses both engines by explicit path, so there is
//! never a question of which `ein` ran.

#![forbid(unsafe_code)]

fn main() -> std::process::ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.iter().any(|a| a == "--version" || a == "-V") {
        println!("ein {}", env!("CARGO_PKG_VERSION"));
        return std::process::ExitCode::SUCCESS;
    }
    eprintln!(
        "ein.rs: not implemented yet — the engine lands over P1a.1–P1a.5.\n\
         Use ein.py meanwhile: `python3 -m ein.cli {}`",
        args.join(" ")
    );
    std::process::ExitCode::from(2)
}
