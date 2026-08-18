//! What compiling a whole program's plans costs — S1a.3.1's acceptance number.
//!
//! Not a `criterion` bench: the M1a bench set is fixed
//! ([`crates/ein-conformance/benches/engine.rs`](../../ein-conformance/benches/engine.rs)),
//! chosen before there were results to be tempted by, and growing it for one
//! stage's number would undo that. This is the same shape as `ein-ir`'s
//! `load_rss` example — a program that reports one figure, run by hand and
//! quoted in the stage's close-out.
//!
//! ```sh
//! cargo run --release -p ein-infer --example compile_cost
//! ```
//!
//! The Python column, for comparison, comes from the exhaustive-`zebra2`
//! profile in [design/06](../../../../plans/m1a_rust/design/06_saturation.md) §3:
//! **253 440 `compile_for` calls costing 1.45 s**, of which all but 19 are
//! cache hits — the waste Win A removes. What is measured here is the 19: one
//! cold compile of every distinct `(rule, activator)` pair.

use std::time::Instant;

use ein_core::Terms;
use ein_ir::{Ast, load_file};

fn main() {
    let root = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../..")
        .canonicalize()
        .expect("repo root");
    for rel in ["examples/zebra2.ein", "examples/zebra.ein"] {
        let mut ast = Ast::new();
        let mut terms = Terms::new();
        let kb = load_file(&mut ast, &mut terms, &root.join(rel)).expect("loads");
        let rules: Vec<_> = kb.program().rules.values().cloned().collect();

        // The pairs, once, so the measurement is compile time and not the
        // walk that finds the work.
        let pairs: Vec<_> = rules
            .iter()
            .flat_map(|r| {
                ein_infer::activators_for(&kb, &terms, r)
                    .into_iter()
                    .map(move |a| (r, a))
            })
            .collect();

        // Warm: every symbol these plans name is interned by the first round,
        // so later rounds measure compiling rather than interning.
        for (rule, activator) in &pairs {
            ein_infer::compile_rule(&ast, &mut terms, rule, *activator).expect("compiles");
        }

        const ROUNDS: u32 = 1000;
        let start = Instant::now();
        let mut steps = 0usize;
        for _ in 0..ROUNDS {
            for (rule, activator) in &pairs {
                let plan =
                    ein_infer::compile_rule(&ast, &mut terms, rule, *activator).expect("compiles");
                steps += plan.steps.len();
            }
        }
        let each = start.elapsed() / ROUNDS;
        println!(
            "{rel}: {} plans, {} steps — {each:?} per full compile ({:?} per plan)",
            pairs.len(),
            steps / ROUNDS as usize,
            each / pairs.len().max(1) as u32,
        );
    }
}
