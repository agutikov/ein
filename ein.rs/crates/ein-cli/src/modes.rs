//! `ein solve`'s region modes — M1e2
//! [S1e2.1a.9](../../../../plans/m1e2_correctness/p1e2.1a_the_solution_region/s1e2.1a.9_solve_describes_the_region.md)
//! T1e2.1a.9.4: `--minimal`, `--maximal`, `--single` and `--region-budget`,
//! with `-n`, answered by [`ein_infer::explore`] and, since
//! [T1e2.1a.5.2](../../../../plans/m1e2_correctness/p1e2.1a_the_solution_region/s1e2.1a.5_the_read_out.md),
//! printed in the read-out of
//! [`read_out.md`](../../../../plans/m1e2_correctness/p1e2.1a_the_solution_region/read_out.md)
//! by [`ein_render::read_out`].
//!
//! **Until S1e2.1a.5's switch the default is still the walk** (ruling 4: the
//! goldens move once). One of the four region flags is therefore what takes a
//! run here, and every option of the walk's is refused beside them rather than
//! ignored: a flag that does nothing is a claim nobody checks.
//!
//! The program is loaded and solved **once**, and every `(query …)` it holds
//! is answered from that one solve (the diff's § 7.1).

use std::path::Path;
use std::time::{Duration, Instant};

use clap::ArgMatches;
use clap::parser::ValueSource;
use ein_core::Terms;
use ein_infer::explore::{self, Ask, Mode, Stopped, Which};
use ein_infer::read_out::{self, ReadOut};
use ein_ir::Ast;

/// The `solve` options a region mode takes. Every other one is the walk's.
const TAKES: [&str; 11] = [
    "file",
    "solutions",
    "max-time",
    "jobs",
    "region-limit",
    "stats",
    "minimal",
    "maximal",
    "single",
    "region-budget",
    "dump-answer",
];

/// Whether this `solve` run asks a region mode.
pub fn wanted(m: &ArgMatches) -> bool {
    m.get_flag("minimal")
        || m.get_flag("maximal")
        || m.get_flag("single")
        || m.get_one::<i64>("region-budget").is_some()
}

/// The mode the flags ask for.
pub fn mode_of(m: &ArgMatches) -> Mode {
    let which = if m.get_flag("minimal") {
        Which::Minimal
    } else if m.get_flag("maximal") {
        Which::Maximal
    } else {
        Which::All
    };
    let ask = if m.get_flag("single") {
        Ask::Single
    } else if m.value_source("solutions") == Some(ValueSource::CommandLine) {
        Ask::Find(*m.get_one::<i64>("solutions").expect("defaulted") as u64)
    } else {
        Ask::Describe
    };
    Mode { which, ask }
}

/// The arguments after `command`, as typed, less `-j N`: the header names the
/// question, and the same question at another `-j` prints the same lines
/// (C4).
pub fn header_argv(command: &str) -> String {
    let args: Vec<String> = std::env::args().collect();
    let start = args
        .iter()
        .position(|a| a == command)
        .map_or(args.len(), |i| i + 1);
    let mut shown: Vec<&str> = Vec::new();
    let mut it = args[start..].iter();
    while let Some(a) = it.next() {
        if a == "-j" || a == "--jobs" {
            it.next();
            continue;
        }
        if a.starts_with("--jobs=") || (a.starts_with("-j") && a.len() > 2) {
            continue;
        }
        shown.push(a);
    }
    shown.join(" ")
}

pub fn run(m: &ArgMatches) -> i32 {
    let solve = crate::cmdline::command();
    let solve = solve.find_subcommand("solve").expect("solve is registered");
    for arg in solve.get_arguments() {
        let id = arg.get_id().as_str();
        if TAKES.contains(&id) || m.value_source(id) != Some(ValueSource::CommandLine) {
            continue;
        }
        let name = arg
            .get_long()
            .map(|l| format!("--{l}"))
            .unwrap_or_else(|| id.to_string());
        eprintln!(
            "error: {name} is the walk's: the region modes (--minimal, --maximal, --single, \
             --region-budget) do not take it until S1e2.1a.5 says what it means there"
        );
        return 2;
    }
    let file = m.get_one::<String>("file").expect("required").clone();
    let mut opts = explore::Options::new(mode_of(m));
    if let Some(&b) = m.get_one::<i64>("region-budget") {
        opts.budget = b as u64;
    }
    opts.limit = (*m.get_one::<i64>("region-limit").unwrap_or(&16)).clamp(0, 63) as u32;
    opts.jobs = crate::solve::resolve_jobs(*m.get_one::<i64>("jobs").unwrap_or(&1));
    let max_time = m.get_one::<f64>("max-time").copied();
    opts.deadline = max_time.map(|s| Instant::now() + Duration::from_secs_f64(s.max(0.0)));

    // `--dump-answer` makes its directory before the run, as the walk's
    // does, so a path that cannot be written fails before the work.
    let dump = m.get_one::<String>("dump-answer").cloned();
    if let Some(dir) = dump.as_deref()
        && let Err(e) = std::fs::create_dir_all(dir)
    {
        crate::solve::artefact_failed("dump-answer", dir, e);
        return 1;
    }

    let mut ast = Ast::new();
    let mut terms = Terms::new();
    let Some(mut kb) = crate::common::load_any_or_exit(&mut ast, &mut terms, Path::new(&file))
    else {
        return 1;
    };
    let r = match read_out::run(&mut kb, &mut terms, &ast, &opts) {
        Ok(r) => r,
        Err(e) => {
            eprintln!("{e}");
            return 1;
        }
    };
    let code = print(
        &r,
        "solve",
        &file,
        opts.budget,
        max_time,
        m.get_flag("stats"),
    );
    if code != 2
        && let Some(dir) = dump.as_deref()
        && let Err(e) = write_answer_txt(&r, Path::new(dir), &file, opts.budget)
    {
        crate::solve::artefact_failed("dump-answer", dir, e);
        return 1;
    }
    code
}

/// `DIR/answer.txt`: the read-out with no listing cut — where stdout's cut
/// says the rest is (the user's ruling of 2026-10-16, the diff's § 4.1).
pub fn write_answer_txt(r: &ReadOut, dir: &Path, file: &str, budget: u64) -> std::io::Result<()> {
    let argv = header_argv("solve");
    let text = ein_render::read_out::render(
        r,
        &ein_render::read_out::Options {
            command: "solve",
            argv: &argv,
            file,
            budget,
            uncut: true,
        },
    );
    std::fs::write(dir.join("answer.txt"), text.stdout)
}

/// Print `r`, write its stderr lines, and return its exit code. `-T` aborts
/// with 2 before anything is printed.
pub fn print(
    r: &ReadOut,
    command: &str,
    file: &str,
    budget: u64,
    max_time: Option<f64>,
    stats: bool,
) -> i32 {
    if r.answer.stopped == Some(Stopped::Deadline) {
        eprintln!(
            "** aborted: --max-time {} s reached before the question was decided **",
            max_time.unwrap_or(0.0)
        );
        return 2;
    }
    let argv = header_argv(command);
    let text = ein_render::read_out::render(
        r,
        &ein_render::read_out::Options {
            command,
            argv: &argv,
            file,
            budget,
            uncut: false,
        },
    );
    print!("{}", text.stdout);
    if stats {
        println!();
        println!(
            "  {:<18}{:.1} ms",
            "wall",
            r.answer.wall.as_secs_f64() * 1e3
        );
    }
    for l in &text.stderr {
        eprintln!("{l}");
    }
    text.code
}
