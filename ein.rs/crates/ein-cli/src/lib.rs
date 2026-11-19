//! `ein` — the binary, as a library.
//!
//! Six subcommands, and the **delegated** dispatch that gives `saturate` its
//! own parser while still listing it in `ein --help`: `argv[0]` is intercepted
//! before the top-level parser ever sees it — `clap` has no equivalent of
//! "registered so it appears in help, never actually parsed".
//!
//! Exit codes: 0 success, 1 load error, 2 usage error, and 2 again for a
//! budget abort ([`07-diagnostics.md` §2](../../../../docs/ein/user/04-reference/07-diagnostics.md#2-errors-and-exit-codes)).
//! The last two collide.
//!
//! **`test` is the one exception, and it is deliberate**: there 1 means *an
//! expectation is false* and a load error takes 2, because a test runner that
//! cannot tell a broken file from a false claim fails at its one job
//! ([`05-cli.md` § 2.3](../../../../docs/ein/user/04-reference/05-cli.md#23-ein-test)).

#![forbid(unsafe_code)]

pub mod cmdline;
mod common;
mod factdump;
pub mod help_shape;
#[cfg(feature = "einb")]
mod kb;
mod modes;
mod oracle;
mod printers;
mod render;
pub mod saturate;
mod show;
pub mod solve;
pub mod summary;
pub mod test;
pub mod version;

use std::process::ExitCode;

use ein_render::RuleMode;

/// The binary's whole body — argv in, exit status out.
pub fn run() -> ExitCode {
    let argv: Vec<String> = std::env::args().skip(1).collect();
    // The delegated subcommand owns its own flag parsing.
    if argv.first().map(String::as_str) == Some("saturate") {
        let status = saturate::main(&argv[1..]);
        common::say_warnings();
        return code(status);
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
    let status = dispatch(&m);
    // What the load and the run warned of; `ein test` says it per file.
    common::say_warnings();
    code(status)
}

fn code(status: i32) -> ExitCode {
    ExitCode::from(status.clamp(0, 255) as u8)
}

fn dispatch(m: &clap::ArgMatches) -> i32 {
    match m.subcommand() {
        Some(("solve", sm)) => solve::run(sm),
        Some(("oracle", sm)) => oracle::run(sm),
        Some(("test", sm)) => test::run(sm),
        Some(("render", sm)) => {
            let file =
                |s: &clap::ArgMatches| s.get_one::<String>("file").expect("required").clone();
            let mode =
                |s: &clap::ArgMatches| match s.get_one::<String>("rule-mode").map(String::as_str) {
                    Some("overlay") => RuleMode::Overlay,
                    _ => RuleMode::SideBySide,
                };
            let fold = |s: &clap::ArgMatches| s.get_flag("fold");
            match sm.subcommand() {
                Some(("rules", a)) => {
                    render::cmd_rules(&file(a), mode(a), fold(a), &common::load_options(a))
                }
                Some(("rule", a)) => render::cmd_rule(
                    &file(a),
                    a.get_one::<String>("name").expect("required"),
                    mode(a),
                    fold(a),
                    &common::load_options(a),
                ),
                Some(("constraints", a)) => {
                    render::cmd_constraints(&file(a), fold(a), &common::load_options(a))
                }
                Some(("program", a)) => {
                    render::cmd_program(&file(a), mode(a), fold(a), &common::load_options(a))
                }
                Some(("kb", a)) => render::cmd_kb(
                    &render::KbArgs {
                        file: &file(a),
                        since: a.get_one::<String>("since").map(String::as_str),
                        colour_by: a
                            .get_one::<String>("colour-by")
                            .and_then(|c| ein_render::ColourBy::parse(c))
                            .unwrap_or(ein_render::ColourBy::Relation),
                        fold: fold(a),
                        goal: a.get_one::<String>("goal").map(String::as_str),
                        query: a.get_one::<u64>("query").map(|&n| n as usize),
                    },
                    &common::load_options(a),
                ),
                Some(("region", a)) => render::cmd_region(
                    a.get_one::<String>("dir").expect("required"),
                    a.get_one::<String>("view")
                        .map(String::as_str)
                        .unwrap_or("auto"),
                    a.get_one::<String>("visited").map(String::as_str),
                    a.get_one::<String>("answer").map(String::as_str),
                ),
                // A retired depth cap, refused as `solve -m` is.
                Some(("lattice", a))
                    if solve::refuse_retired(
                        a,
                        &[(
                            "max-set-size",
                            "--max-set-size is retired with `ein solve -m`: \
                             `--region-budget FACTS` bounds the run drawn",
                        )],
                    ) =>
                {
                    2
                }
                Some(("lattice", a)) => render::cmd_lattice(
                    &file(a),
                    a.get_one::<String>("view").map(String::as_str) == Some("full"),
                    a.get_one::<i64>("region-budget").map(|&b| b as u64),
                    &common::load_options(a),
                ),
                Some((name, a)) if name == ein_render::structure::COMMAND => render::cmd_h_space(
                    &render::HSpaceArgs {
                        file: &file(a),
                        view: a
                            .get_one::<String>("view")
                            .map(String::as_str)
                            .unwrap_or("summary"),
                        dump: a.get_one::<String>("dump").map(String::as_str),
                        supports: a.get_flag("supports"),
                        budget: a.get_one::<i64>("region-budget").map(|&b| b as u64),
                        jobs: solve::resolve_jobs(*a.get_one::<i64>("jobs").unwrap_or(&1)),
                    },
                    &common::load_options(a),
                ),
                _ => 2,
            }
        }
        Some(("show", sm)) => match sm.subcommand() {
            Some(("hyps", a)) => show::cmd_hyps(
                a.get_one::<String>("file").expect("required"),
                &common::load_options(a),
            ),
            _ => 2,
        },
        #[cfg(feature = "einb")]
        Some(("kb", sm)) => match sm.subcommand() {
            Some(("save", a)) => kb::cmd_save(
                a.get_one::<String>("file").expect("required"),
                a.get_one::<String>("out").expect("required"),
                a.get_flag("saturate"),
                &common::load_options(a),
            ),
            _ => 2,
        },
        // Unreachable: `saturate` is intercepted above, and `clap` requires a
        // subcommand.
        _ => 2,
    }
}
