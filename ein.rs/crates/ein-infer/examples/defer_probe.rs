//! S1a.7.0 T1a.7.0.5 — what deferring a layer's integration costs.
//!
//! ```sh
//! cargo run --release -p ein-infer --example defer_probe
//! ```
//!
//! One process, one file at a time, three integration policies:
//! [`SolveOptions::integrate_every`] `None` (the sequential engine), a barrier
//! every 20 enterings, and one barrier per layer. Reports the entering count,
//! **root's layer depth at exit** and the wall clock.
//!
//! The depth column is the one to read. Every root write seals another layer —
//! `Kb::fork` seals the top so the parent's later appends land in a new one —
//! and every fork inherits the whole stack. A barrier coalesces a layer's
//! writes into one burst, and on `branching/07 -e` that is the difference
//! between depth 164 and depth 3, which is 2.8× of the run.
//!
//! **That win no longer needs this mode, which is why every column here sets
//! [`SolveOptions::coalesce_root_at`] to `None`.** T1a.7.2.0 read the row
//! below and took the depth directly — `Kb::flatten` at the layer barrier,
//! integration still immediate — so with the shipping default on, all three
//! columns are shallow and the probe measures nothing. Switching it off is
//! what keeps [scaling.md §4](../../../../docs/history/m1a_rust/measurements/scaling.md)
//! reproducible; the flatten's own numbers are `flatten_probe` and §6.
//!
//! That the *answer* is unchanged is not checked here: it is
//! `tests/search_invariants.rs`'s job, and it is checked on more files than
//! this probe times.
//!
//! Numbers:
//! [scaling.md §4](../../../../docs/history/m1a_rust/measurements/scaling.md).

use ein_core::Terms;
use ein_infer::Events;
use ein_infer::solve::{NoDumper, SolveOptions, solve};
use ein_ir::{Ast, load_file};
use std::path::PathBuf;
use std::time::Instant;

/// The two zebras (where the singleton writeback prunes hard), a deep search
/// with no writebacks at all, one with 162 that prune nothing, and two small
/// controls.
const FILES: &[&str] = &[
    "examples/zebra2.ein",
    "examples/zebra.ein",
    "examples/zebra2-hints.ein",
    "examples/branching/04_two_levels.ein",
    "examples/branching/06_lookahead_on.ein",
    "examples/saturation/square-bwd/houses.ein",
    "examples/branching/07_lookahead_off.ein",
];

fn main() {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../..")
        .canonicalize()
        .expect("repo root");
    let args: Vec<String> = std::env::args().skip(1).collect();
    let files: Vec<&str> = if args.is_empty() {
        FILES.to_vec()
    } else {
        args.iter().map(|s| s.as_str()).collect()
    };
    println!(
        "{:<46} {:>28} {:>28} {:>28}",
        "workload", "sequential", "barrier every 20", "one barrier per layer"
    );
    for rel in files {
        print!("{rel:<46}");
        for batch in [None, Some(20usize), Some(usize::MAX)] {
            let mut ast = Ast::new();
            let mut terms = Terms::new();
            let mut kb = load_file(&mut ast, &mut terms, &root.join(rel)).expect("loads");
            let opts = SolveOptions {
                stop_after: None,
                integrate_every: batch,
                // See the module doc: this probe's subject is the deferral,
                // and the shipping barrier would collapse every column.
                coalesce_root_at: None,
                ..SolveOptions::default()
            };
            let mut events = Events::off();
            let started = Instant::now();
            let solved = solve(&mut kb, &mut terms, &ast, &mut events, &mut NoDumper, &opts)
                .expect("solves");
            print!(
                " {:>12} d{:<4} {:>6.0}ms",
                solved.stats.base.enterings_total,
                kb.depth(),
                started.elapsed().as_secs_f64() * 1000.0
            );
        }
        println!();
    }
}
