//! What a whole solve costs — the P1a.4 figure.
//!
//! The milestone's baseline is `solve zebra2.ein` end-to-end: 1.87 s under
//! CPython on the default `stop_after = 1` path and 5.69 s exhaustive, with
//! **hypothesis search** the 4.96 s of it. This measures the search alone —
//! parse and load are P1a.1/P1a.2's rows — over the same work, which the
//! printed verdict and entering count say.
//!
//! ```sh
//! cargo run --release -p ein-infer --example solve_cost
//! ```

use std::time::Instant;

use ein_core::Terms;
use ein_infer::solve::{NoDumper, SolveOptions, solve};
use ein_infer::{Events, verdict::Answer};
use ein_ir::{Ast, load_file};

fn main() {
    let root = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../..")
        .canonicalize()
        .expect("repo root");
    for rel in ["examples/zebra2.ein", "examples/zebra.ein"] {
        for (label, stop_after) in [("fast", Some(1)), ("exhaustive", None)] {
            let mut ast = Ast::new();
            let mut terms = Terms::new();
            let mut kb = load_file(&mut ast, &mut terms, &root.join(rel)).expect("loads");
            let mut events = Events::off();
            let opts = SolveOptions {
                stop_after,
                ..SolveOptions::default()
            };
            let start = Instant::now();
            let solved = solve(&mut kb, &mut terms, &ast, &mut events, &mut NoDumper, &opts)
                .expect("solves");
            let took = start.elapsed();
            let verdict = match &solved.answer {
                Answer::Verdict(v) => v.as_str(),
                Answer::Aborted { .. } => "Aborted",
            };
            println!(
                "{rel} {label:10}: {verdict} k={} enterings={} (alive {} dead {}/{}) \
                 saturations={} — {took:?}",
                solved.stats.solution_nodes,
                solved.stats.base.enterings_total,
                solved.stats.base.enterings_alive,
                solved.stats.base.enterings_dead_pre,
                solved.stats.base.enterings_dead_post,
                solved.stats.base.saturate_count,
            );
        }
    }
}
