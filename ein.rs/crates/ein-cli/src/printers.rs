//! `ein solve`'s diagnostic printers — the counters, the phase table, the
//! resolved config, the root-hypothesis preview, and the timing dumper.
//!
//! Every column position here is ein.py's. The floats go through
//! [`ein_core::pyfmt`] rather than Rust's `{:.2}` because the two disagree on
//! sign and fill even where they agree on digits (Q-M1a.15).

use std::time::Instant;

use ein_core::pyfmt::format_spec;
use ein_core::{Kb, SolverConfig, Terms};
use ein_infer::SharedMemo;
use ein_infer::solve::{Dumper, JobStats, MonotonicStats};
use ein_infer::verdict::Answer;
use ein_ir::Ast;

/// `_TimingDumper` — per-phase wall-clock off the solve loop's hooks, no file
/// I/O. `t0` is set at construction, immediately before `solve`.
pub struct TimingDumper {
    pub t0: Instant,
    pub t_root: Option<Instant>,
    pub t_end: Option<Instant>,
    pub root_facts: usize,
}

impl TimingDumper {
    pub fn new() -> TimingDumper {
        TimingDumper {
            t0: Instant::now(),
            t_root: None,
            t_end: None,
            root_facts: 0,
        }
    }
}

impl Dumper for TimingDumper {
    fn root_initial(&mut self, kb: &Kb, _terms: &Terms) {
        self.t_root = Some(Instant::now());
        self.root_facts = kb.n_facts();
    }

    fn summary(&mut self, _verdict: &Answer, _stats: &MonotonicStats) {
        self.t_end = Some(Instant::now());
    }

    fn close(&mut self) {
        if self.t_end.is_none() {
            self.t_end = Some(Instant::now());
        }
    }
}

/// The four values `--timing` reads off whichever dumper ran.
pub struct Phases {
    pub t0: Instant,
    pub t_root: Option<Instant>,
    pub t_end: Option<Instant>,
    pub root_facts: usize,
}

/// `-s/--stats` — the engine counters.
pub fn print_stats(stats: &MonotonicStats, elapsed_ms: f64) {
    let b = &stats.base;
    println!();
    println!("stats");
    println!("  solutions (k)    {}", stats.solution_nodes);
    println!(
        "  exhausted        {}",
        if stats.exhausted { "true" } else { "false" }
    );
    println!(
        "  enterings        {} (alive={} dead_pre={} dead_post={})",
        b.enterings_total, b.enterings_alive, b.enterings_dead_pre, b.enterings_dead_post
    );
    println!("  layers_explored  {}", b.layers_explored);
    println!("  saturate_count   {}", b.saturate_count);
    println!(
        "  nogoods          emitted={} subsumed={}",
        b.nogoods_emitted, b.nogoods_subsumed
    );
    println!("  wall             {} ms", format_spec(elapsed_ms, ".1f"));
}

/// `-s/--stats` under `--jobs N` — what the fan-out did (T1a.7.2.5).
///
/// **Printed only when `--jobs > 1`**, and that is the whole of why it may
/// live on a surface the invariance sweeps read. `--stats` is already the one
/// block that reports the *run* rather than the answer — it has printed a
/// `wall` since ein.py, and no two runs agree on that — so a job count is at
/// home here and nowhere else. Every number below is deliberately absent from
/// [`MonotonicStats`] and from `--json-summary`, because those are compared
/// exactly between `--jobs 1` and `--jobs N` and these must differ.
///
/// The four rows answer four questions a reader of a scaling number has:
///
/// - **workers** — how many threads a layer actually used, which is
///   `min(--jobs, the batch)` and is `0` when no layer was fanned out at all;
/// - **speculated** — enterings evaluated on a worker, and its three-way
///   split. `wasted` is the one the acceptance bounds: enterings computed on
///   a worker that the run stopped before committing, which can only happen
///   at a `stop_after` / budget cut and is at most one batch (T1a.7.2.4);
/// - **handed_back** — enterings a worker could not finish because it would
///   have had to number a proposition, re-run on the committing thread;
/// - **sequential** — enterings whose *layer* could write a fact to root
///   ([`ein_infer::solve`]'s fan-out predicate), so they ran in order however
///   many jobs were asked for. Amdahl's numerator, per run.
pub fn print_job_stats(jobs: &JobStats, asked: usize) {
    let wasted = jobs
        .speculated
        .saturating_sub(jobs.committed)
        .saturating_sub(jobs.handed_back);
    println!();
    println!("jobs");
    println!("  workers          {} (of {asked} asked)", jobs.workers);
    println!(
        "  speculated       {} (committed={} handed_back={} wasted={})",
        jobs.speculated, jobs.committed, jobs.handed_back, wasted
    );
    println!("  sequential       {}", jobs.sequential);
}

/// `-t/--timing` — the per-phase wall-clock table.
pub fn print_timing(
    p: &Phases,
    parse_ms: f64,
    load_ms: f64,
    n_forms: usize,
    compile_ms: f64,
    n_plans: usize,
    stats: &MonotonicStats,
) {
    let ms = |a: Instant, b: Instant| (b.duration_since(a)).as_secs_f64() * 1000.0;
    let root_ms = p.t_root.map_or(0.0, |r| ms(p.t0, r));
    let end = p.t_end.unwrap_or_else(Instant::now);
    let search_ms = p.t_root.map_or(0.0, |r| ms(r, end));
    let solve_ms = root_ms + search_ms;
    let e2e_ms = parse_ms + load_ms + solve_ms;
    let enterings = stats.base.enterings_total;
    let per_hyp = if enterings > 0 {
        search_ms / enterings as f64
    } else {
        0.0
    };
    let f = |v: f64| format_spec(v, "9.2f");

    println!();
    println!("timing (ms)");
    println!("  parse              {}    ({n_forms} forms)", f(parse_ms));
    println!("  kb load            {}", f(load_ms));
    println!(
        "  root saturation    {}    ({} facts after saturation)",
        f(root_ms),
        p.root_facts
    );
    println!(
        "  hypothesis search  {}    ({enterings} enterings / {} layers / {} root saturations)",
        f(search_ms),
        stats.base.layers_explored,
        stats.base.saturate_count
    );
    println!(
        "    per hypothesis   {}    (avg over enterings)",
        f(per_hyp)
    );
    println!("  {}", "─".repeat(40));
    println!(
        "  solve              {}    (root saturation + search)",
        f(solve_ms)
    );
    println!(
        "  end-to-end         {}    (parse + load + solve)",
        f(e2e_ms)
    );
    println!(
        "  compile            {}    ({n_plans} plans; isolated \
         — the solve compiles these lazily inside saturation)",
        f(compile_ms)
    );
}

/// `-c/--dump-config` — each resolved `SolverConfig` field, in declaration
/// order, with the name kebab-cased into a 32-column field.
///
/// Note this is *not* `config::rendered_fields`: that one spells values as
/// `repr` for the KB-shape dump, and `--dump-config` prints `str(v)` with
/// bools lowered — so `'popularity'` there is `popularity` here.
pub fn print_resolved_config(cfg: &SolverConfig) {
    let b = |v: bool| if v { "true" } else { "false" }.to_string();
    let fields: Vec<(&str, String)> = vec![
        (
            "enable-pre-branch-lookahead",
            b(cfg.enable_pre_branch_lookahead),
        ),
        (
            "enable-lookahead-kill-cache",
            b(cfg.enable_lookahead_kill_cache),
        ),
        ("hypgen-scoring", cfg.hypgen_scoring.clone()),
        ("hypgen-rel-weight", format!("{:?}", cfg.hypgen_rel_weight)),
        ("hypgen-obj-weight", format!("{:?}", cfg.hypgen_obj_weight)),
        ("print-alive", b(cfg.print_alive)),
        ("warn-derived-naf", b(cfg.warn_derived_naf)),
        ("candidate-order-seed", cfg.candidate_order_seed.to_string()),
        ("lattice-sanity-check", b(cfg.lattice_sanity_check)),
        ("lattice-order", cfg.lattice_order.clone()),
        (
            "lattice-order-seed",
            match cfg.lattice_order_seed {
                Some(s) => s.to_string(),
                None => "None".to_string(),
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
    ];
    println!("config (resolved)");
    for (name, shown) in fields {
        println!("  {name:<32} {shown}");
    }
}

/// `-H/--hyp-stats` — saturate a fork of root and report what the hypothesis
/// generator would enumerate.
pub fn print_root_hyp_preview(ast: &Ast, terms: &mut Terms, kb: &mut Kb) -> Result<(), String> {
    use ein_infer::events::Events;
    use ein_infer::hypgen::{HypGenStats, generate};
    use ein_infer::saturator::{Saturator, Session};
    use std::ops::ControlFlow;

    let mut preview = kb.fork();
    let mut events = Events::off();
    let mut s = Session {
        kb: &mut preview,
        terms,
        ast,
        events: &mut events,
        memo: SharedMemo::default(),
    };
    ein_infer::closed::emit_closed(&mut s).map_err(crate::common::compile_error_line)?;
    let mut sat = Saturator::new(&mut s).map_err(|e| crate::common::saturate_error_line(&e))?;
    sat.saturate(&mut s, None, &mut |_| {})
        .map_err(|e| crate::common::saturate_error_line(&e))?;

    let mut stats = HypGenStats::new();
    let mut generated: Vec<ein_core::FactId> = Vec::new();
    generate(&mut s, &mut stats, &mut |f| {
        generated.push(f);
        ControlFlow::Continue(())
    })
    .map_err(crate::common::compile_error_line)?;

    // `Counter` over the generated facts' relations: counts in first-seen
    // order, which is what `most_common` breaks ties by.
    let mut order: Vec<ein_core::Symbol> = Vec::new();
    let mut counts: Vec<u64> = Vec::new();
    for f in generated {
        let rel = s.terms.facts.get(f).0;
        match order.iter().position(|&r| r == rel) {
            Some(i) => counts[i] += 1,
            None => {
                order.push(rel);
                counts.push(1);
            }
        }
    }

    if order.is_empty() {
        println!("root hyps        0 candidates");
        // …and *why* there are none, when the ladder is what decided it. A
        // state that owes something it may not branch on prints the same
        // "0 candidates" as one that is finished, and telling them apart is
        // the whole of M1d S1d.2.5's stuck report. Nothing prints here for a
        // program with no obligation rule, so no existing preview moves.
        if stats.rung.mode != ein_infer::oblgen::Mode::Blind {
            for line in stats.report_lines() {
                println!("  {line}");
            }
        }
        return Ok(());
    }
    let total: u64 = counts.iter().sum();
    println!(
        "root hyps        {total} candidates across {} relations",
        order.len()
    );
    // `Counter.most_common()` — by count descending, ties in insertion order,
    // which `sort_by` preserves because it is stable.
    let mut rows: Vec<(usize, u64)> = counts.iter().copied().enumerate().collect();
    rows.sort_by_key(|r| std::cmp::Reverse(r.1));
    for (i, n) in rows {
        let pct = 100.0 * n as f64 / total as f64;
        println!(
            "  {:<24} {:>6}  ({}%)",
            s.terms.sym(order[i]),
            n,
            format_spec(pct, ">5.1f")
        );
    }
    println!("root hyp-gen filter breakdown:");
    for line in stats.report_lines() {
        println!("  {line}");
    }
    Ok(())
}
