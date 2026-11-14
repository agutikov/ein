//! `ein solve` — the one command for R, `solve_interface.md` § 2.1.
//!
//! Since M1e2 S1e2.1a.5's switch every run is a region run
//! (`crate::modes`): bare `ein solve FILE` describes R, and `--minimal`,
//! `--maximal`, `--single` and `-n N` each ask one question about it. The
//! walk has no CLI path. Its library code is kept, with no caller, until
//! P1e2.1c measures what of it the cheap modes reuse.
//!
//! What is here is the command's own half: the nine retired options, refused
//! with what replaced each, and the helpers `test` and `oracle` share.

use std::io::Write;
use std::path::Path;

use clap::ArgMatches;
use clap::parser::ValueSource;
use ein_core::{Kb, SolverConfig, Terms};
use ein_infer::events::{Events, Level};
use ein_ir::Ast;

/// The nine options the switch retired (`solve_interface.md` § 2.1), each with
/// the sentence its refusal prints: what replaces it. They stay in the parser,
/// hidden, so that this is what a stale argv gets.
pub(crate) const RETIRED: [(&str, &str); 11] = [
    (
        "exhaustive",
        "-e/--exhaustive is retired: `ein solve FILE` describes all of R, which \
         is exhaustive, and `--maximal` gives the maximal line, which is what \
         the walk's models were",
    ),
    (
        "max-set-size",
        "-m/--max-set-size is retired: a depth cap changed answers. `--minimal` \
         gives the least commitments and their sizes, and `--region-budget \
         FACTS` bounds the work",
    ),
    (
        "max-enterings",
        "-E/--max-enterings is retired: `--region-budget FACTS` bounds the work, \
         and a run that reaches it prints what it established",
    ),
    (
        "lattice-order",
        "-o/--lattice-order is retired: each route's order is part of its \
         definition, so no lever changes which states are printed",
    ),
    (
        "shuffle",
        "-z/--shuffle is retired: each route's order is part of its definition, \
         so no lever changes which states are printed",
    ),
    (
        "seed",
        "-d/--seed is retired with --shuffle: each route's order is part of its \
         definition",
    ),
    (
        "dump-states",
        "-D/--dump-states is retired: root is never written, so every snapshot \
         was root's. `--dump-answer DIR` writes each state, and `--dump-lattice \
         DIR` each node visited",
    ),
    (
        "dump-region",
        "--dump-region moved: `ein oracle FILE --dump DIR` writes the cube",
    ),
    (
        "region-states",
        "--region-states moved: `ein oracle FILE --dump DIR --states`",
    ),
    (
        "no-lookahead",
        "-L/--no-lookahead is retired (M1e2 S1e2.1c.5): no route runs the \
         walk's one-step lookahead, and every route saturates each candidate's \
         fork, so the lever changed nothing",
    ),
    (
        "no-kill-cache",
        "-K/--no-kill-cache is retired (M1e2 S1e2.1c.5): no route writes a kill \
         into root, and L1 prunes a dead extension's supersets by construction, \
         so the lever changed nothing",
    ),
];

/// Refuse the first retired option `m` carries, with its sentence. `true`
/// when it refused.
pub(crate) fn refuse_retired(m: &ArgMatches, retired: &[(&str, &str)]) -> bool {
    for (id, why) in retired {
        if m.try_get_raw(id).ok().flatten().is_some()
            && m.value_source(id) == Some(ValueSource::CommandLine)
        {
            eprintln!("error: {why}");
            return true;
        }
    }
    false
}

pub fn run(m: &ArgMatches) -> i32 {
    if refuse_retired(m, &RETIRED) {
        return 2;
    }
    crate::modes::run(m)
}

/// `kb.config` (or defaults) plus the lever the command line sets, `-y`, the
/// one left since M1e2 S1e2.1c.5 retired `-L` and `-K`. A command that does
/// not define it leaves the field as the program set it.
pub(crate) fn resolved_config(kb: &Kb, m: &ArgMatches) -> SolverConfig {
    let mut cfg = kb.program().config.clone().unwrap_or_default();
    let on = |id: &str| m.try_get_one::<bool>(id).ok().flatten().copied() == Some(true);
    if on("lattice-sanity-check") {
        cfg.lattice_sanity_check = true;
    }
    cfg
}

/// The resolved config as the `run` event's `config` object: the two flags
/// a program can set (`ein_core::config::FLAGS`), kebab keys, JSON values, in
/// declaration order.
pub(crate) fn config_json(cfg: &SolverConfig) -> Vec<(&'static str, String)> {
    let b = |v: bool| v.to_string();
    vec![
        ("lattice-sanity-check", b(cfg.lattice_sanity_check)),
        (
            "record-alternative-justifications",
            b(cfg.record_alternative_justifications),
        ),
    ]
}

/// Open the log and emit `run`.
///
/// Shared with `test`, which takes the same two flags for the same reason: a
/// failing expectation is exactly when someone wants the stream.
pub(crate) fn events_start(m: &ArgMatches, file: &str, cfg: &SolverConfig) -> Events {
    let Some(path) = m.get_one::<String>("events") else {
        return Events::off();
    };
    let level = match m.get_one::<String>("events-level").map(String::as_str) {
        Some("verbose") => Level::Verbose,
        _ => Level::Normal,
    };
    let sink: Box<dyn Write + Send> = match std::fs::File::create(path) {
        Ok(f) => Box::new(std::io::BufWriter::new(f)),
        Err(e) => {
            artefact_failed("events", path, e);
            return Events::off();
        }
    };
    let argv: Vec<String> = std::env::args().skip(1).collect();
    let cfg_json = config_json(cfg);
    Events::to_with(sink, level, |l| {
        l.str("impl", "ein.rs");
        l.str("file", file);
        l.owned_strs("argv", argv);
        l.obj_strs("config", &cfg_json);
    })
}

/// **The one shape an artefact flag's failure takes** — M1e
/// [S1e.3.5](../../../../docs/history/m1e_review_processing/README.md#s1e35--error-handling-medium),
/// `EH-M1`.
///
/// Five options write a file the caller asked for, and before this stage they
/// failed in four different ways: `--events`, `--json-summary` and `--trace`
/// printed a bare OS error (*Is a directory (os error 21)* — no flag, no
/// path, on a run with three artefact flags), `--json-report` named the path
/// and not the flag, and `--dump-states` was the only one whose failure the
/// exit code carried. A consumer that reads stderr — which is the whole of
/// what the contract gives it — has to be able to tell *which* artefact was
/// lost and *where* it was going.
///
/// **The exit code is deliberately not this function's business.** The four
/// options whose `--help` says *additive: stdout, stderr and the exit code are
/// unchanged* keep saying it: a failed write leaves the run's answer standing,
/// because the answer on stdout is correct and complete and the artefact is
/// paperwork. `--dump-states` is the exception and states why at its call
/// site. The ruling and the reasoning are
/// [`defined_behaviour.md` § 4.4](../../../../docs/kernel/defined_behaviour.md),
/// and whether the additive arm should have an exit code of its own is
/// [Q-M1e.22](../../../../docs/history/m1e_review_processing/open_questions.md).
pub(crate) fn artefact_failed(flag: &str, path: &str, e: impl std::fmt::Display) {
    eprintln!("error: --{flag} {path}: {e}");
}

/// The `load` event — what the loader built, in registry order.
pub(crate) fn events_load(events: &mut Events, terms: &Terms, kb: &Kb) {
    if !events.on() {
        return;
    }
    let p = kb.program();
    let rel_names: Vec<&str> = p.relations.keys().map(|s| terms.sym(s)).collect();
    let rule_names: Vec<&str> = p.rules.keys().map(|s| terms.sym(s)).collect();
    events.emit("load", |l| {
        l.num("relations", p.relations.len() as i64);
        l.num("rules", p.rules.len() as i64);
        l.num("hrules", p.hrules.len() as i64);
        l.num("macros", p.macros.len() as i64);
        l.num("facts", kb.n_facts() as i64);
        l.strs("relation_names", rel_names);
        l.strs("rule_names", rule_names);
    });
}

/// `--jobs N`, or `--jobs auto` — which the parser gives us as `0`.
///
/// **The default is 1 and stays 1** ([S1a.7.5](../../../../docs/history/m1a_rust/README.md#s1a75--the---jobs-contract)
/// T1a.7.5.1): a benchmark, a golden run and the corpus sweep must never
/// silently become a different computation, even one that is provably the
/// same. `auto` is what a user opts into, and it is the *machine's* number:
/// [`std::thread::available_parallelism`] respects a cgroup quota and a
/// `taskset` affinity mask, and counts logical CPUs — SMT siblings and E-cores
/// included, which on a hybrid box is not the fastest setting.
///
/// This lives here and **not in `SolverConfig`**, which is the ruling
/// T1a.7.5.1 owed: every `SolverConfig` field is printed by `--dump-config`
/// and parsed from `(config …)`, so putting a thread count there would let a
/// *puzzle file* set it — a `.ein` that reads differently on an 8-core machine
/// than on a 4-core one, through a field the KB-shape digest renders. Jobs is
/// an execution knob; `SolverConfig` is the semantics.
pub(crate) fn resolve_jobs(n: i64) -> usize {
    if n == 0 {
        return std::thread::available_parallelism().map_or(1, |n| n.get());
    }
    n.max(1) as usize
}

/// The program `--dump-answer` writes at the head of every file: the source
/// with its imports inlined, by the three calls `stdlib/README.md` documents
/// and `imports_semantics::the_inlining_route_the_stdlib_readme_documents_round_trips`
/// holds to the same program; or, on a `.einb`, the program it stores, already
/// resolved and import-flattened (M1f S1f.5.12, W43). The program as loaded,
/// whole: since M1f S1f.6.35 an import brings its module whole and nothing is
/// shaken out (E-41), so the file declares every full name the run had. The file has already
/// loaded, so a failure here is a defect, and it is written into the file
/// rather than swallowed.
pub(crate) fn inlined_program(file: &str) -> String {
    let path = Path::new(file);
    let p = match crate::common::program_text(path) {
        Ok(p) => p,
        Err(e) => return format!(";; (the source could not be read for inlining: {e})\n"),
    };
    let text = p.text;
    let mut ast = Ast::new();
    let forms = match ein_ir::parse(&mut ast, &text, p.name.as_deref()) {
        Ok(f) => f,
        Err(e) => return format!(";; (the source did not parse for inlining: {e})\n{text}"),
    };
    match ein_ir::resolve_imports(&mut ast, &forms, p.base.as_deref()) {
        Ok(inlined) => ein_ir::dump_canonical(&ast, &inlined),
        Err(e) => format!(";; (imports could not be inlined: {e})\n{text}"),
    }
}
