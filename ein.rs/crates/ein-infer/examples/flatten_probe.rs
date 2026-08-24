//! T1a.7.2.0 — what coalescing root's layer stack at the layer barrier costs
//! and buys.
//!
//! ```sh
//! cargo run --release --features counters -p ein-infer --example flatten_probe
//! ```
//!
//! One process, one file at a time, [`SolveOptions::coalesce_root_at`] `None`
//! against the shipping threshold and two others. Reports, per setting: the
//! entering count, root's depth at exit, how many barriers flattened and how
//! many facts they copied, and the wall clock.
//!
//! **Read the entering column first.** This is not
//! [`defer_probe`](defer_probe.rs): a deferral buys the same depth collapse
//! by holding a layer's root writes back, and pays for it in prunes that
//! arrive late — 101 → 521 enterings on `zebra2 -e`. A flatten defers nothing.
//! Every writeback lands exactly when it does today and only root's
//! *representation* is rebuilt, so the entering count is identical in every
//! column and the whole of the difference is the read path. If a column here
//! ever shows a different count, the flatten has stopped being answer-neutral
//! and `tests/search_invariants.rs` is the test that should have said so first.
//!
//! The `flat` and `copied` columns are the cost side, and they are why the
//! setting is a threshold rather than a `bool`: [`Kb::materialise`] is
//! O(facts), so a search whose layers are cheap and whose root is large can
//! pay more than it saves.
//!
//! Numbers:
//! [scaling.md §6](../../../../docs/history/m1a_rust/measurements/scaling.md).

use ein_core::Terms;
use ein_infer::Events;
use ein_infer::solve::{NoDumper, SolveOptions, solve};
use ein_ir::{Ast, load_file};
use std::path::PathBuf;
use std::time::Instant;

/// The measurement set — the four workloads P1a.7's scaling target names —
/// plus the zebra family, which is where the writebacks are dense and the
/// roots are small, and one big-root control.
const FILES: &[&str] = &[
    "examples/branching/07_lookahead_off.ein",
    "examples/branching/06_lookahead_on.ein",
    "examples/saturation/square-bwd/houses.ein",
    "examples/features/01_not_and_absent.ein",
    "examples/zebra.ein",
    "examples/zebra2.ein",
    "examples/zebra2-hints.ein",
    "examples/branching/04_two_levels.ein",
];

/// `None` is the pre-T1a.7.2.0 engine; `Some(2)` flattens at every barrier
/// (a fork seals root's top, so 2 is the floor a layer can leave); `Some(3)`
/// is "a writeback happened"; `Some(20)` only pays on a deep stack.
const SETTINGS: &[Option<usize>] = &[None, Some(2), Some(3), Some(20)];

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
    print!("{:<46}", "workload");
    for s in SETTINGS {
        let label = match s {
            None => "off".to_string(),
            Some(n) => format!("at depth {n}"),
        };
        print!(" {label:>34}");
    }
    println!();
    for rel in files {
        print!("{rel:<46}");
        for &setting in SETTINGS {
            let mut ast = Ast::new();
            let mut terms = Terms::new();
            let mut kb = load_file(&mut ast, &mut terms, &root.join(rel)).expect("loads");
            let opts = SolveOptions {
                stop_after: None,
                coalesce_root_at: setting,
                ..SolveOptions::default()
            };
            let mut events = Events::off();
            ein_core::counters::reset();
            let started = Instant::now();
            let Ok(solved) = solve(&mut kb, &mut terms, &ast, &mut events, &mut NoDumper, &opts)
            else {
                print!(" {:>34}", "(no verdict)");
                continue;
            };
            let ms = started.elapsed().as_secs_f64() * 1000.0;
            let c = ein_core::counters::snapshot();
            print!(
                " {:>7} d{:<3} {:>3} flat {:>7} copied {:>6.0}ms",
                solved.stats.base.enterings_total,
                kb.depth(),
                c.flatten,
                c.flatten_facts,
                ms
            );
        }
        println!();
    }
}
