//! `ein` — the drop-in binary, as a library.
//!
//! Three subcommands, and the **delegated** dispatch that gives `saturate` its
//! own parser while still listing it in `ein --help`: `argv[0]` is intercepted
//! before the top-level parser ever sees it, exactly as `cli/__init__.py`'s
//! `_DELEGATED` does — `clap` has no equivalent of "registered so it appears
//! in help, never actually parsed".
//!
//! Exit codes are ein.py's: 0 success, 1 load error, 2 usage error, and 2
//! again for a budget abort. The last two collide there and the collision is
//! reproduced rather than fixed.

#![forbid(unsafe_code)]

pub mod cmdline;
mod common;
mod factdump;
pub mod help_shape;
#[cfg(feature = "einb")]
mod kb;
mod printers;
mod render;
pub mod saturate;
pub mod solve;
pub mod summary;
pub mod version;

use std::process::ExitCode;

use ein_render::{LatticeView, RuleMode};

/// The binary's whole body — argv in, exit status out.
pub fn run() -> ExitCode {
    let argv: Vec<String> = std::env::args().skip(1).collect();
    // The delegated subcommand owns its own flag parsing.
    if argv.first().map(String::as_str) == Some("saturate") {
        return code(saturate::main(&argv[1..]));
    }
    // `--version` is intercepted for the same reason `saturate` is, and only
    // in first position: the top-level parser requires a subcommand, so
    // `clap` would reject `ein --version` before ever reaching the flag.
    // Anywhere else it is not this flag — `ein solve x.ein --version` is a
    // usage error, and staying out of the way is how it keeps being one.
    if matches!(argv.first().map(String::as_str), Some("--version" | "-V")) {
        print!("{}", version::report());
        return ExitCode::SUCCESS;
    }
    let m = cmdline::command().get_matches_from(std::iter::once("ein".to_string()).chain(argv));
    code(dispatch(&m))
}

fn code(status: i32) -> ExitCode {
    ExitCode::from(status.clamp(0, 255) as u8)
}

fn dispatch(m: &clap::ArgMatches) -> i32 {
    match m.subcommand() {
        Some(("solve", sm)) => solve::run(sm),
        Some(("render", sm)) => {
            let file =
                |s: &clap::ArgMatches| s.get_one::<String>("file").expect("required").clone();
            let mode =
                |s: &clap::ArgMatches| match s.get_one::<String>("rule-mode").map(String::as_str) {
                    Some("overlay") => RuleMode::Overlay,
                    _ => RuleMode::SideBySide,
                };
            match sm.subcommand() {
                Some(("rules", a)) => render::cmd_rules(&file(a), mode(a)),
                Some(("rule", a)) => render::cmd_rule(
                    &file(a),
                    a.get_one::<String>("name").expect("required"),
                    mode(a),
                ),
                Some(("constraints", a)) => render::cmd_constraints(&file(a)),
                Some(("lattice", a)) => render::cmd_lattice(
                    &file(a),
                    match a.get_one::<String>("view").map(String::as_str) {
                        Some("full") => LatticeView::Full,
                        _ => LatticeView::Solution,
                    },
                    *a.get_one::<i64>("max-set-size").unwrap_or(&3),
                ),
                _ => 2,
            }
        }
        #[cfg(feature = "einb")]
        Some(("kb", sm)) => match sm.subcommand() {
            Some(("save", a)) => kb::cmd_save(
                a.get_one::<String>("file").expect("required"),
                a.get_one::<String>("out").expect("required"),
                a.get_flag("saturate"),
            ),
            _ => 2,
        },
        // Unreachable: `saturate` is intercepted above, and `clap` requires a
        // subcommand.
        _ => 2,
    }
}
