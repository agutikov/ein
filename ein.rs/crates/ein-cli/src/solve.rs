//! `ein solve` — the one solver command.
//!
//! The Rust half of `ein/cli/solve.py`. The verdict is *read from the result*
//! (`k = 0 / 1 / >1`), never chosen by a flag, and every diagnostic is opt-in
//! so the default output stays the answer and nothing else.

use std::io::Write;
use std::path::{Path, PathBuf};
use std::time::Instant;

use clap::ArgMatches;
use ein_core::{Kb, SolverConfig, Terms};
use ein_infer::events::{Events, Level};
use ein_infer::solve::{Dumper, MonotonicStats, NoDumper, SolveError, SolveOptions, Solved, solve};
use ein_infer::verdict::{Answer, Verdict};
use ein_ir::Ast;
use ein_render::dump::{MonotonicDumper, ProgressDumper};
use ein_render::render_solution_table;

use crate::common::read_text_or_crash;
use crate::factdump::{self, hypothesis_target_relations, print_final_state, print_unsat_core};
use crate::printers::{self, Phases, TimingDumper};

/// `_timed_load` — parse + build the KB, timing each phase.
///
/// Replaces the shared loader so `--timing` can split parse from kb-load
/// without re-doing the work. `n_forms` counts the *parsed* top-level forms,
/// before import resolution flattens them.
fn timed_load(
    ast: &mut Ast,
    terms: &mut Terms,
    path: &Path,
    query: usize,
) -> Option<(Kb, f64, f64, usize)> {
    // A `.einb` has no parse phase to time and no top-level forms to count:
    // its whole open is the load, which is the point of it (T1a.8.1.7).
    #[cfg(feature = "einb")]
    if ein_einb::is_einb(&crate::common::read_bytes_or_crash(path)) {
        let t = Instant::now();
        let kb = crate::common::load_any_query_or_exit(ast, terms, path, query)?;
        return Some((kb, 0.0, t.elapsed().as_secs_f64() * 1000.0, 0));
    }
    let text = read_text_or_crash(path);
    let t = Instant::now();
    let forms = match ein_ir::parse(ast, &text, path.to_str()) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("{e}");
            return None;
        }
    };
    let parse_ms = t.elapsed().as_secs_f64() * 1000.0;
    let t = Instant::now();
    match ein_ir::load_query(ast, terms, &forms, path.parent(), query) {
        Ok(kb) => {
            let load_ms = t.elapsed().as_secs_f64() * 1000.0;
            Some((kb, parse_ms, load_ms, forms.len()))
        }
        Err(e) => {
            eprintln!("kb load error: {e}");
            None
        }
    }
}

/// `_resolved_config` — `kb.config` (or defaults) plus the CLI overrides.
fn resolved_config(kb: &Kb, m: &ArgMatches, seed: Option<i64>) -> SolverConfig {
    let mut cfg = kb.program().config.clone().unwrap_or_default();
    if m.get_flag("no-lookahead") {
        cfg.enable_pre_branch_lookahead = false;
    }
    if m.get_flag("no-kill-cache") {
        cfg.enable_lookahead_kill_cache = false;
    }
    if m.get_flag("lattice-sanity-check") {
        cfg.lattice_sanity_check = true;
    }
    if let Some(order) = m.get_one::<String>("lattice-order") {
        cfg.lattice_order = order.clone();
    }
    if m.get_flag("shuffle") {
        cfg.lattice_order_seed = seed;
    }
    cfg
}

/// The lifecycle dumper, chosen the way `_make_dumper` chooses it.
///
/// `--verbose` wins and streams the live view — and because `ProgressDumper`
/// captures `t0/t_root/t_end/root_facts` it *also* feeds `--timing`, so
/// `-v -t` shows both the search and the phase table.
enum Chosen {
    Progress(Box<ProgressDumper>),
    Timing(TimingDumper),
    Monotonic(Box<MonotonicDumper>),
    None(NoDumper),
}

impl Chosen {
    fn as_dumper(&mut self) -> &mut dyn Dumper {
        match self {
            Chosen::Progress(d) => d.as_mut(),
            Chosen::Timing(d) => d,
            Chosen::Monotonic(d) => d.as_mut(),
            Chosen::None(d) => d,
        }
    }

    fn phases(&self) -> Phases {
        match self {
            Chosen::Progress(d) => Phases {
                t0: d.t0,
                t_root: d.t_root,
                t_end: d.t_end,
                root_facts: d.root_facts,
            },
            Chosen::Timing(d) => Phases {
                t0: d.t0,
                t_root: d.t_root,
                t_end: d.t_end,
                root_facts: d.root_facts,
            },
            _ => Phases {
                t0: Instant::now(),
                t_root: None,
                t_end: None,
                root_facts: 0,
            },
        }
    }
}

fn make_dumper(m: &ArgMatches) -> Result<Chosen, String> {
    let out_dir = m.get_one::<String>("dump-states").map(PathBuf::from);
    let every = *m.get_one::<i64>("progress-every").unwrap_or(&100);
    if m.get_flag("verbose") {
        let d = ProgressDumper::new(
            out_dir.as_deref(),
            Box::new(std::io::stderr()),
            every.max(0) as u64,
            "",
        )
        .map_err(|e| e.to_string())?;
        return Ok(Chosen::Progress(Box::new(d)));
    }
    if m.get_flag("timing") {
        return Ok(Chosen::Timing(TimingDumper::new()));
    }
    if out_dir.is_some() {
        let d = MonotonicDumper::new(out_dir.as_deref()).map_err(|e| e.to_string())?;
        return Ok(Chosen::Monotonic(Box::new(d)));
    }
    Ok(Chosen::None(NoDumper))
}

/// `--shuffle` without `--seed`: `random.randrange(1, 2**31)`.
///
/// A fresh seed each run, echoed on stderr — the value only permutes the
/// traversal, and the verdict is shuffle-invariant, so the entropy source does
/// not have to be CPython's. What *does* have to be CPython's is the
/// permutation the seed drives, and that is `mt19937` (Q-M1a.5).
fn fresh_seed() -> i64 {
    use std::time::{SystemTime, UNIX_EPOCH};
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.subsec_nanos() as u64 ^ d.as_secs())
        .unwrap_or(0);
    let mixed = nanos
        .wrapping_mul(6364136223846793005)
        .wrapping_add(std::process::id() as u64);
    1 + (mixed % ((1u64 << 31) - 1)) as i64
}

/// `--print-final-*`: the model facts per solution, or the unsat core.
fn print_final(ast: &Ast, terms: &Terms, kb: &Kb, answer: &Answer, m: &ArgMatches) {
    let modes: Vec<factdump::Mode> = [
        ("print-final-state", factdump::Mode::All),
        ("print-final-positive", factdump::Mode::Positive),
        ("print-final-hfacts", factdump::Mode::Hfacts),
    ]
    .into_iter()
    .filter(|(flag, _)| m.get_flag(flag))
    .map(|(_, mode)| mode)
    .collect();
    if modes.is_empty() {
        return;
    }
    let Answer::Verdict(v) = answer else { return };
    if let Verdict::Contradiction { unsat_core } = v {
        print_unsat_core(terms, unsat_core);
        return;
    }
    let targets = hypothesis_target_relations(ast, terms, kb);
    let branches: Vec<&ein_infer::verdict::Solution> = match v {
        Verdict::Ambiguity(bs) => bs.iter().collect(),
        Verdict::Solution(s) => vec![s],
        Verdict::Contradiction { .. } => unreachable!(),
    };
    let n = branches.len();
    for (i, branch) in branches.iter().enumerate() {
        if n > 1 {
            println!();
            println!("── solution {}/{n} ──", i + 1);
        }
        for mode in &modes {
            print_final_state(terms, &branch.kb, *mode, &targets);
        }
    }
}

/// `-r/--trace` — the self-contained markdown derivation trace, to a file.
fn write_trace(ast: &Ast, terms: &Terms, root: &Kb, solved: &Solved, path: &str, m: &ArgMatches) {
    let diagrams = !m.get_flag("no-diagrams");
    let trace = ein_render::linearize(
        ast,
        terms,
        root,
        solved,
        ein_render::LinearizeOpts {
            diagrams,
            full_kb_snapshots: m.get_flag("full-kb-snapshots"),
            relevant: m.get_flag("relevant"),
        },
    );
    let md = ein_render::render_markdown(
        &trace,
        if m.get_flag("reorder") {
            ein_render::Mode::Reorder
        } else {
            ein_render::Mode::Engine
        },
        diagrams,
    );
    if let Err(e) = std::fs::write(path, md) {
        eprintln!("{e}");
        return;
    }
    eprintln!(
        "wrote {path} ({} steps, {} refuted)",
        trace.steps.len(),
        trace.reductios.len()
    );
}

/// The resolved config as the `run` event's `config` object: kebab keys, JSON
/// values, in declaration order.
pub(crate) fn config_json(cfg: &SolverConfig) -> Vec<(&'static str, String)> {
    let b = |v: bool| v.to_string();
    let s = |v: &str| {
        let mut out = String::new();
        ein_infer::events::push_json_str(&mut out, v);
        out
    };
    vec![
        (
            "enable-pre-branch-lookahead",
            b(cfg.enable_pre_branch_lookahead),
        ),
        (
            "enable-lookahead-kill-cache",
            b(cfg.enable_lookahead_kill_cache),
        ),
        ("hypgen-scoring", s(&cfg.hypgen_scoring)),
        ("hypgen-rel-weight", format!("{:?}", cfg.hypgen_rel_weight)),
        ("hypgen-obj-weight", format!("{:?}", cfg.hypgen_obj_weight)),
        ("print-alive", b(cfg.print_alive)),
        ("warn-derived-naf", b(cfg.warn_derived_naf)),
        ("candidate-order-seed", cfg.candidate_order_seed.to_string()),
        ("lattice-sanity-check", b(cfg.lattice_sanity_check)),
        ("lattice-order", s(&cfg.lattice_order)),
        (
            "lattice-order-seed",
            match cfg.lattice_order_seed {
                Some(v) => v.to_string(),
                None => "null".to_string(),
            },
        ),
        ("enable-path-nogoods", b(cfg.enable_path_nogoods)),
        ("enable-symmetric-mirror", b(cfg.enable_symmetric_mirror)),
        (
            "enable-singleton-writeback",
            b(cfg.enable_singleton_writeback),
        ),
        ("enable-forced-positive", b(cfg.enable_forced_positive)),
        (
            "record-alternative-justifications",
            b(cfg.record_alternative_justifications),
        ),
        ("enable-fail-fast-fork", b(cfg.enable_fail_fast_fork)),
    ]
}

/// `_events.start` — open the log and emit `run`.
fn events_start(m: &ArgMatches, file: &str, cfg: &SolverConfig) -> Events {
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
            eprintln!("{e}");
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

/// `_events.load` — what the loader built, in registry order.
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

/// `_events.verdict` — the answer plus every counter.
fn events_verdict(events: &mut Events, terms: &Terms, answer: &Answer, stats: &MonotonicStats) {
    if !events.on() {
        return;
    }
    let mut core: Vec<String> = Vec::new();
    let mut models: Vec<Vec<String>> = Vec::new();
    if let Answer::Verdict(v) = answer {
        match v {
            Verdict::Contradiction { unsat_core } => {
                core = ein_infer::events::sexpr_facts(terms, unsat_core);
                core.sort();
            }
            Verdict::Solution(s) => models.push(sorted_model(terms, &s.kb)),
            Verdict::Ambiguity(bs) => {
                models = bs.iter().map(|b| sorted_model(terms, &b.kb)).collect();
            }
        }
    }
    models.sort();
    let b = &stats.base;
    let ty = answer.as_str();
    events.emit("verdict", |l| {
        l.str("type", ty);
        l.num("k", stats.solution_nodes as i64);
        l.bool("exhausted", stats.exhausted);
        l.obj_strs(
            "counters",
            &[
                ("enterings_total", b.enterings_total.to_string()),
                ("enterings_alive", b.enterings_alive.to_string()),
                ("enterings_dead_pre", b.enterings_dead_pre.to_string()),
                ("enterings_dead_post", b.enterings_dead_post.to_string()),
                ("facts_merged", b.facts_merged.to_string()),
                ("forced_positives", b.forced_positives.to_string()),
                ("saturate_count", b.saturate_count.to_string()),
                ("layers_explored", b.layers_explored.to_string()),
                ("nogoods_emitted", b.nogoods_emitted.to_string()),
                ("nogoods_subsumed", b.nogoods_subsumed.to_string()),
                ("solution_nodes", stats.solution_nodes.to_string()),
                ("exhausted", stats.exhausted.to_string()),
            ],
        );
        l.owned_strs("core", core);
        l.str_lists("models", &models);
    });
}

fn sorted_model(terms: &Terms, kb: &Kb) -> Vec<String> {
    let mut out: Vec<String> = kb
        .facts()
        .map(|f| ein_infer::events::sexpr(terms, f))
        .collect();
    out.sort();
    out
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
/// than on a 4-core one, through a field the parity contract compares. Jobs is
/// an execution knob; `SolverConfig` is the semantics.
fn resolve_jobs(n: i64) -> usize {
    if n == 0 {
        return std::thread::available_parallelism().map_or(1, |n| n.get());
    }
    n.max(1) as usize
}

/// The artefact flags: each names **one** path, so each is incompatible with a
/// file that asks more than one question.
const ONE_PATH_FLAGS: [&str; 4] = ["events", "trace", "json-summary", "dump-states"];

pub fn run(m: &ArgMatches) -> i32 {
    let file = m.get_one::<String>("file").expect("required").clone();
    let (mut rc, mut index) = (0, 0usize);
    loop {
        let (code, n_queries) = run_query(m, &file, index);
        rc = rc.max(code);
        index += 1;
        // A load failure reports `n_queries = 0` and stops the loop: the next
        // query would fail to load in exactly the same way, and saying so
        // twice is not a second finding.
        if code == 1 && n_queries == 0 || index >= n_queries {
            break;
        }
    }
    rc
}

/// One query of the file: load it, solve it, print it, check its `:expect`.
///
/// Returns the exit code and **how many** `(query …)` blocks the file has, so
/// [`run`] can come back for the rest. Loading once per query is not a
/// concession: `:hypothesis-relations` and `:hrules` are per-query, so two
/// queries over one KB are two genuinely different searches, and sharing a
/// loaded `Kb` between them would be an optimisation rather than a semantics.
fn run_query(m: &ArgMatches, file: &str, index: usize) -> (i32, usize) {
    let mut ast = Ast::new();
    let mut terms = Terms::new();
    let Some((mut kb, parse_ms, load_ms, n_forms)) =
        timed_load(&mut ast, &mut terms, Path::new(file), index)
    else {
        return (1, 0);
    };
    let n_queries = kb.program().queries.len();
    if n_queries > 1 {
        if let Some(flag) = ONE_PATH_FLAGS
            .iter()
            .find(|f| m.get_one::<String>(f).is_some())
        {
            eprintln!(
                "error: --{flag} names one path and this file asks {n_queries} \
                 questions — split the queries, or drop the flag"
            );
            return (2, 0);
        }
        println!("query {} of {n_queries}", index + 1);
    }

    // --shuffle randomises the within-layer commitment order. Traversal-only —
    // the verdict is shuffle-invariant (S1.5b.31); a fresh seed each run
    // unless --seed pins it, echoed below.
    let shuffle = m.get_flag("shuffle");
    let mut seed = m.get_one::<i64>("seed").copied();
    if shuffle && seed.is_none() {
        seed = Some(fresh_seed());
    }
    let config = resolved_config(&kb, m, seed);
    if shuffle {
        eprintln!("shuffle seed: {}", seed.unwrap_or(0));
    }

    if m.get_flag("dump-config") {
        printers::print_resolved_config(&config);
    }
    if m.get_flag("hyp-stats")
        && let Err(e) = printers::print_root_hyp_preview(&ast, &mut terms, &mut kb)
    {
        eprintln!("{e}");
        return (1, n_queries);
    }

    // --timing: isolate the (rule, activator) plan-compilation cost — the real
    // solve does this lazily inside saturation, so measure it standalone.
    let timing = m.get_flag("timing");
    let mut compile_ms = 0.0;
    let mut n_plans = 0usize;
    if timing {
        let mut off = Events::off();
        let t = Instant::now();
        let mut eng = ein_infer::Engine::new();
        if let Err(e) = eng.compile_all(&ast, &mut terms, &kb, &mut off) {
            eprintln!("{}", crate::common::compile_error_line(e));
            return (1, n_queries);
        }
        compile_ms = t.elapsed().as_secs_f64() * 1000.0;
        n_plans = eng.len();
    }

    // Last thing before the solve: the diagnostics above are *about* the run,
    // not part of it, and recording them would make the event stream depend on
    // which other flags were passed.
    let mut events = events_start(m, &file, &config);
    events_load(&mut events, &terms, &kb);

    let mut dumper = match make_dumper(m) {
        Ok(d) => d,
        Err(e) => {
            eprintln!("{e}");
            return (1, n_queries);
        }
    };
    let stop_after = if m.get_flag("exhaustive") {
        None
    } else {
        m.get_one::<i64>("solutions").map(|n| (*n).max(0) as u64)
    };
    let trace_path = m.get_one::<String>("trace").cloned();
    let opts = SolveOptions {
        stop_after,
        max_set_size: (*m.get_one::<i64>("max-set-size").unwrap_or(&5)).max(0) as u32,
        config: Some(config.clone()),
        max_time: m.get_one::<f64>("max-time").copied(),
        max_enterings: m
            .get_one::<i64>("max-enterings")
            .map(|n| (*n).max(0) as u64),
        store_lattice: trace_path.is_some(),
        jobs: resolve_jobs(*m.get_one::<i64>("jobs").unwrap_or(&1)),
        ..SolveOptions::default()
    };

    let t0 = Instant::now();
    let solved = match solve(
        &mut kb,
        &mut terms,
        &ast,
        &mut events,
        dumper.as_dumper(),
        &opts,
    ) {
        Ok(s) => s,
        Err(SolveError::Budget { reason, stats }) => {
            eprintln!("** aborted: {reason} **");
            if let Some(path) = m.get_one::<String>("json-summary") {
                match crate::summary::build_aborted(
                    &ast,
                    &mut terms,
                    &mut kb,
                    &reason,
                    &stats,
                    &config,
                    &file,
                    &mut events,
                ) {
                    Ok(s) => {
                        if let Err(e) = crate::summary::write(path, &s) {
                            eprintln!("{e}");
                        }
                    }
                    Err(e) => eprintln!("{e}"),
                }
            }
            return (2, n_queries);
        }
        Err(SolveError::Compile(e)) => {
            eprintln!("{}", crate::common::compile_error_line(e));
            return (1, n_queries);
        }
        Err(SolveError::Saturate(e)) => {
            eprintln!("{}", crate::common::saturate_error_line(&e));
            return (1, n_queries);
        }
        Err(e) => {
            eprintln!("{e}");
            return (1, n_queries);
        }
    };
    let elapsed_ms = t0.elapsed().as_secs_f64() * 1000.0;

    // Result-driven: the table reports k / verdict / query bindings / rendered
    // query facts / NL result — all text from puzzle templates, nothing
    // hardcoded here.
    // `render_solution_table` is fallible for one reason: ein.py compiles the
    // `:goal` pattern inside it, so a goal the compiler rejects ends the run
    // *after* a successful solve — and only when there is a model to render.
    let table = match render_solution_table(
        &ast,
        &mut terms,
        &kb,
        &solved.answer,
        Some(solved.stats.solution_nodes),
        solved.stats.exhausted,
        Some(&file),
    ) {
        Ok(t) => t,
        Err(line) => {
            eprintln!("{line}");
            return (1, n_queries);
        }
    };
    println!("{table}");
    print_final(&ast, &terms, &kb, &solved.answer, m);
    if m.get_flag("stats") {
        printers::print_stats(&solved.stats, elapsed_ms);
        // Only when a fan-out was possible. At the default `--jobs 1` the
        // block would be four rows of zero, and every `--stats` run in the
        // repo would have grown them for nothing.
        if opts.jobs > 1 {
            printers::print_job_stats(&solved.jobs, opts.jobs);
        }
    }
    if timing {
        printers::print_timing(
            &dumper.phases(),
            parse_ms,
            load_ms,
            n_forms,
            compile_ms,
            n_plans,
            &solved.stats,
        );
    }
    if let Some(path) = trace_path.as_deref() {
        write_trace(&ast, &terms, &kb, &solved, path, m);
    }
    // Last: the summary re-saturates a fork of root to fill its `root` block,
    // so it runs after every stdout-producing step rather than between two.
    events_verdict(&mut events, &terms, &solved.answer, &solved.stats);
    if let Some(path) = m.get_one::<String>("json-summary") {
        match crate::summary::build(
            &ast,
            &mut terms,
            &mut kb,
            &solved.answer,
            &solved.stats,
            &config,
            &file,
            &mut events,
        ) {
            Ok(s) => {
                if let Err(e) = crate::summary::write(path, &s) {
                    eprintln!("{e}");
                }
            }
            Err(e) => eprintln!("{e}"),
        }
    }
    // Last of all, because a failing expectation is exactly when someone wants
    // the trace above it: the query's own claim about its answer, checked. A
    // `:expect` that `solve` merely ignored would be worse than no `:expect`,
    // which is the whole reason the keyword is not a comment.
    (check_expectation(&ast, &terms, &kb, &solved.answer), n_queries)
}

/// Evaluate the active query's `:expect`, printing what disagreed.
///
/// Exit 1 on a failure, which is §4's code for "the engine says no": a false
/// claim by the program is a result, not a usage error.
fn check_expectation(ast: &Ast, terms: &Terms, kb: &Kb, answer: &Answer) -> i32 {
    let Some(query) = kb.program().query() else {
        return 0;
    };
    let Some(node) = ein_infer::query_value(ast, query, "expect") else {
        return 0;
    };
    // The loader rejected every shape this can refuse, so the `Err` arm is
    // unreachable from a program and says so rather than inventing a message.
    let Ok(expectation) = ein_ir::expect::parse(ast, node) else {
        eprintln!("internal error: :expect passed the loader and did not parse");
        return 1;
    };
    let report = ein_infer::expect::check(ast, terms, &expectation, answer);
    if report.passed {
        println!("\n  :expect        holds");
        return 0;
    }
    println!("\n  :expect        FAILED");
    for line in &report.lines {
        println!("    {line}");
    }
    1
}
