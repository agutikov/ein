//! The `features.md` lever matrix, regenerated against ein.rs.
//!
//! One cell per engine lever, flipped off against the all-on baseline, in the
//! two modes `utils/feature_matrix.py` uses: `fast` (`stop_after = 1`, the
//! shipped path) and `exhaustive` (`stop_after = None`, where a disabled prune
//! shows its full blow-up).
//!
//! ```sh
//! cargo run --release -p ein-infer --example lever_matrix
//! ```
//!
//! **The wall-clock column is not comparable to ein.py's and the entering
//! column is.** The Python matrix aborts a runaway cell on a *wall-clock*
//! budget, which is machine-dependent and turns "too slow here" into a verdict;
//! this one caps `max_enterings`, so a cell that ein.py could only report as
//! `Aborted (≥90 s)` gets an actual number. What the acceptance asks is that
//! the verdicts and entering counts agree wherever ein.py produced one, and
//! that `enable_singleton_writeback` is still the single load-bearing lever.

use std::time::Instant;

use ein_core::{SolverConfig, Terms};
use ein_infer::Events;
use ein_infer::solve::{NoDumper, OnBudget, SolveOptions, solve};
use ein_ir::{Ast, load_file};

/// A generous cap: `zebra2`'s all-on exhaustive run is 101 enterings, and the
/// one lever that blows up is documented at 3 336+.
const CAP: u64 = 20_000;

fn main() {
    let root = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../..")
        .canonicalize()
        .expect("repo root");
    let path = root.join("examples/zebra2.ein");

    /// One lever, and the patch that turns it off.
    type Cell = (&'static str, fn(&mut SolverConfig));
    let cells: Vec<Cell> = vec![
        ("baseline", |_| {}),
        ("no-lookahead", |c| c.enable_pre_branch_lookahead = false),
        ("no-kill-cache", |c| c.enable_lookahead_kill_cache = false),
        ("no-path-nogoods", |c| c.enable_path_nogoods = false),
        ("no-symmetric-mirror", |c| c.enable_symmetric_mirror = false),
        ("no-singleton-writeback", |c| {
            c.enable_singleton_writeback = false
        }),
        ("no-forced-positive", |c| c.enable_forced_positive = false),
        ("no-fail-fast-fork", |c| c.enable_fail_fast_fork = false),
        ("hypgen-most-constrained", |c| {
            c.hypgen_scoring = "most-constrained".to_string()
        }),
        ("lattice-score-sum", |c| {
            c.lattice_order = "score-sum".to_string()
        }),
    ];

    for (mode, stop_after) in [("fast", Some(1)), ("exhaustive", None)] {
        println!("\n{mode} (zebra2.ein, max_enterings={CAP})");
        println!(
            "  {:<26} {:<14} {:>9}  {:>10}",
            "lever off", "verdict", "enter", "wall"
        );
        let mut base = 0.0f64;
        for (name, patch) in &cells {
            let mut ast = Ast::new();
            let mut terms = Terms::new();
            let mut kb = load_file(&mut ast, &mut terms, &path).expect("loads");
            let mut cfg = kb.program().config.clone().unwrap_or_default();
            patch(&mut cfg);
            kb.program_mut().config = Some(cfg.clone());
            let mut events = Events::off();
            let opts = SolveOptions {
                stop_after,
                config: Some(cfg),
                max_enterings: Some(CAP),
                on_budget: OnBudget::Verdict,
                ..SolveOptions::default()
            };
            // Two runs, and the second is the one reported. The baseline
            // runs first, so a single-shot column would charge it for every
            // cold cache in the process and make each later lever look like
            // an improvement.
            let mut secs = 0.0;
            let mut solved = None;
            for _ in 0..2 {
                let mut ast = Ast::new();
                let mut terms = Terms::new();
                let mut kb = load_file(&mut ast, &mut terms, &path).expect("loads");
                kb.program_mut().config = Some(opts.config.clone().expect("set"));
                let start = Instant::now();
                let s = solve(&mut kb, &mut terms, &ast, &mut events, &mut NoDumper, &opts)
                    .expect("solves");
                secs = start.elapsed().as_secs_f64();
                solved = Some(s);
            }
            let solved = solved.expect("ran");
            if *name == "baseline" {
                base = secs;
            }
            println!(
                "  {:<26} {:<14} {:>9}  {:>8.0} ms  {:.1}x",
                name,
                solved.answer.as_str(),
                solved.stats.base.enterings_total,
                secs * 1e3,
                secs / base
            );
        }
    }
}
