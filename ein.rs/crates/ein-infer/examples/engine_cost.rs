//! What a root saturation costs, and how much work `match_hot` measures.
//!
//! The companion to `compile_cost`: a program that reports the figures the
//! stage's acceptance quotes, so the comparison against ein.py is against the
//! **same work** rather than against a wall clock alone. `match_hot` in
//! particular is only a meaningful ratio if both sides enumerate the same
//! matches and consume the same premises — which the parity tests prove and
//! this prints.
//!
//! ```sh
//! cargo run --release -p ein-infer --example engine_cost
//! ```
//!
//! The Python column, measured the same way on the dev machine:
//!
//! | | ein.py | ein.rs |
//! |---|---:|---:|
//! | root saturation, `zebra2` | 90 ms | see below |
//! | `match_hot` (every plan, saturated root) | 2 110 µs | see below |

use std::time::Instant;

use ein_core::Terms;
use ein_infer::{Events, Matcher, Saturator, Session};
use ein_ir::{Ast, load_file};

fn main() {
    let root = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../..")
        .canonicalize()
        .expect("repo root");
    for rel in ["examples/zebra2.ein", "examples/zebra.ein"] {
        let mut ast = Ast::new();
        let mut terms = Terms::new();
        let mut kb = load_file(&mut ast, &mut terms, &root.join(rel)).expect("loads");
        let mut events = Events::off();
        let start = Instant::now();
        let (firings, plans, rounds, guard_evals, monotone, boundary) = {
            let mut s = Session {
                kb: &mut kb,
                terms: &mut terms,
                ast: &ast,
                events: &mut events,
            };
            let mut sat = Saturator::new(&mut s).expect("compiles");
            let n = sat.saturate(&mut s, None, &mut |_| {}).expect("saturates");
            (
                n,
                sat.engine.len(),
                sat.naf_rounds,
                sat.guard_evals,
                sat.guard_evals_monotone,
                std::time::Duration::from_nanos(sat.boundary_nanos),
            )
        };
        let saturate = start.elapsed();

        // Every plan, run over the saturated root — the `match_hot` workload.
        let rules: Vec<_> = kb.program().rules.values().cloned().collect();
        let mut compiled = Vec::new();
        for rule in &rules {
            for activator in ein_infer::activators_for(&kb, &terms, rule) {
                compiled
                    .push(ein_infer::compile_rule(&ast, &mut terms, rule, activator).expect("ok"));
            }
        }
        let mut m = Matcher::new();
        let mut premises = 0usize;
        let mut matches = 0usize;
        let start = Instant::now();
        const ROUNDS: u32 = 1000;
        for _ in 0..ROUNDS {
            premises = 0;
            matches = 0;
            for plan in &compiled {
                m.run(&kb, &terms, &ast, plan, &mut |mt| {
                    premises += mt.premises().len();
                    matches += 1;
                    std::ops::ControlFlow::Continue(())
                });
            }
        }
        let match_hot = start.elapsed() / ROUNDS;

        println!(
            "{rel}: {} facts, {firings} firings, {plans} plans — saturate {saturate:?}",
            kb.n_facts()
        );
        println!(
            "  boundary: {rounds} rounds, {guard_evals} guard evaluations \
             ({monotone} monotone), {boundary:?} ({:.0}% of saturation)",
            100.0 * boundary.as_secs_f64() / saturate.as_secs_f64()
        );
        println!(
            "  match_hot: {} plans, {matches} matches, {premises} premises — {match_hot:?}",
            compiled.len()
        );
    }
}
