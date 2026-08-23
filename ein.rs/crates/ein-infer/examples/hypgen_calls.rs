//! What one hypothesis-generation *call* costs, split by what it is for —
//! [S1a.6.4](../../../../docs/history/m1a_rust/README.md#s1a64--hypgen-and-lattice-hot-paths)'s
//! acceptance number.
//!
//! ```sh
//! cargo run --release -p ein-infer --example hypgen_calls
//! ```
//!
//! `hypgen_cost` times a *pass* and reports the mean over 100 rounds. That
//! number is dominated by round 1 — the round whose lookahead probes fill the
//! kill cache, after which every later candidate dies at `negated_fact` for a
//! bit test — so it answers "what does the first enumeration cost", which is
//! not the question the search asks. The search asks two others, and they have
//! very different shapes:
//!
//! - **`complete()`** — is there *any* undecided candidate? Short-circuits on
//!   the first survivor (S1.9.E16), so its cost is the fixed setup plus one
//!   candidate. Called once per alive entering.
//! - **`open_hypotheses()`** — the whole open set. Runs the pipeline over every
//!   candidate. Called once per layer.
//!
//! The **setup** is the third column and the reason this example exists: every
//! call rebuilds a fresh [`Lookahead`], and a fresh `Lookahead` walks
//! `rules × activators` through a fresh [`Engine`](ein_infer::Engine), which is
//! ~120 compile-cache keys on `zebra2` — against 125 raw candidates for the
//! whole pass. A `complete()` that short-circuits on candidate #1 still pays
//! all of it.
//!
//! Steady state, not first call: the KB is saturated and one warm-up pass has
//! already written the kill cache, which is the state every call inside the
//! search loop but the very first sees.

use std::time::Instant;

use ein_core::Terms;
use ein_infer::{Events, HypGenStats, Lookahead, Saturator, Session, SharedMemo};
use ein_ir::{Ast, load_file};

const ROUNDS: u32 = 200;

fn main() {
    let root = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../..")
        .canonicalize()
        .expect("repo root");
    println!(
        "{:<46} {:>10} {:>12} {:>10} {:>9}",
        "workload [mode]", "complete()", "open_hyp()", "setup", "raw"
    );
    for rel in [
        "examples/zebra2.ein",
        "examples/zebra.ein",
        "examples/saturation/square-unique/terminus.ein",
        "examples/features/05_stdlib_domain_elim.ein",
    ] {
        let mut ast = Ast::new();
        let mut terms = Terms::new();
        let mut kb = load_file(&mut ast, &mut terms, &root.join(rel)).expect("loads");
        let mut events = Events::off();
        // One memo for the whole cell, as a run has: `Lookahead::new`'s
        // `compile_all` then hits the memo on every call but the first, which
        // is what makes the setup column the *engine walk* rather than the
        // compiler.
        let memo = SharedMemo::default();
        let mut s = Session {
            kb: &mut kb,
            terms: &mut terms,
            ast: &ast,
            events: &mut events,
            memo: memo.clone(),
        };
        let mut sat = Saturator::new(&mut s).expect("compiles");
        sat.saturate(&mut s, None, &mut |_| {}).expect("saturates");

        // Warm-up: the kill cache is written by the first pass, and every call
        // the search makes after the first sees it already written.
        let mut warm = HypGenStats::new();
        ein_infer::generate(&mut s, &mut warm, &mut |_| {
            std::ops::ControlFlow::Continue(())
        })
        .expect("generates");

        let start = Instant::now();
        for _ in 0..ROUNDS {
            std::hint::black_box(ein_infer::complete(&mut s).expect("completes"));
        }
        let per_complete = start.elapsed() / ROUNDS;

        let start = Instant::now();
        for _ in 0..ROUNDS {
            std::hint::black_box(
                ein_infer::open_hypotheses(&mut s)
                    .expect("enumerates")
                    .len(),
            );
        }
        let per_open = start.elapsed() / ROUNDS;

        let start = Instant::now();
        for _ in 0..ROUNDS {
            std::hint::black_box(Lookahead::new(&mut s).expect("compiles"));
        }
        let per_setup = start.elapsed() / ROUNDS;

        let mut stats = HypGenStats::new();
        ein_infer::generate(&mut s, &mut stats, &mut |_| {
            std::ops::ControlFlow::Continue(())
        })
        .expect("generates");
        let mode = if s.kb.program().hrules.is_empty() {
            "blind"
        } else {
            "hrule"
        };
        println!(
            "{:<46} {:>10} {:>12} {:>10} {:>9}",
            format!("{rel} [{mode}]"),
            format!("{per_complete:?}"),
            format!("{per_open:?}"),
            format!("{per_setup:?}"),
            stats.raw,
        );
    }
}
