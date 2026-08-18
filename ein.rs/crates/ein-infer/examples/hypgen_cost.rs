//! What one hypothesis-generation pass costs — the S1a.4.1 figure.
//!
//! The companion to `engine_cost`, and measured the same way: the numbers
//! printed alongside the wall clock say what work was done, because a ratio
//! against ein.py is only meaningful if both sides build the same candidates
//! and drop them at the same filters — which `tests/hypgen_parity.rs` proves
//! and this prints.
//!
//! ```sh
//! cargo run --release -p ein-infer --example hypgen_cost
//! ```
//!
//! One pass is timed rather than a whole solve: the search loop that calls it
//! ~100 times is [S1a.4.5](../../../../plans/m1a_rust/p1a.4_search_layer/s1a.4.5_solve_loop.md)'s.
//! Rounds after the first do identical work — the only state a pass leaves
//! behind is the kill cache's `(not h)` facts, and writing one is idempotent —
//! so the mean over `ROUNDS` is the steady-state cost.
//!
//! **Both generation modes are measured**, because they share no code beyond
//! the filter pipeline: `zebra` and `zebra2` declare an `(hrule …)`, so the
//! blind enumerator never runs on them, and `terminus` is the largest corpus
//! file that has none. The lookahead lever is the other axis, and it is by
//! far the larger one.

use std::time::Instant;

use ein_core::Terms;
use ein_infer::{Events, HypGenStats, Saturator, Session, SharedMemo};
use ein_ir::{Ast, load_file};

fn main() {
    let root = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../..")
        .canonicalize()
        .expect("repo root");
    for rel in [
        "examples/zebra2.ein",
        "examples/zebra.ein",
        "examples/saturation/square-unique/terminus.ein",
    ] {
        for lookahead in [true, false] {
            let mut ast = Ast::new();
            let mut terms = Terms::new();
            let mut kb = load_file(&mut ast, &mut terms, &root.join(rel)).expect("loads");
            let mut cfg = kb.program().config.clone().unwrap_or_default();
            cfg.enable_pre_branch_lookahead = lookahead;
            kb.program_mut().config = Some(cfg);
            let mut events = Events::off();
            let mut s = Session {
                kb: &mut kb,
                terms: &mut terms,
                ast: &ast,
                events: &mut events,
                memo: SharedMemo::default(),
            };
            let mut sat = Saturator::new(&mut s).expect("compiles");
            sat.saturate(&mut s, None, &mut |_| {}).expect("saturates");

            const ROUNDS: u32 = 100;
            let mut stats = HypGenStats::new();
            let start = Instant::now();
            for _ in 0..ROUNDS {
                stats = HypGenStats::new();
                ein_infer::generate(&mut s, &mut stats, &mut |_| {
                    std::ops::ControlFlow::Continue(())
                })
                .expect("generates");
            }
            let per_pass = start.elapsed() / ROUNDS;
            let on = if lookahead { "on " } else { "off" };
            let mode = if s.kb.program().hrules.is_empty() {
                "blind"
            } else {
                "hrule"
            };
            println!(
                "{rel} [{mode}] lookahead={on}: raw {} → emitted {} — {per_pass:?}/pass",
                stats.raw, stats.emitted
            );
        }
    }
}
