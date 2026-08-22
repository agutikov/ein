//! S1a.7.1 T1a.7.1.0 — **how hard the shared state is actually hit.**
//!
//! ```sh
//! cargo run --release -p ein-infer --features counters --example shared_state_probe
//! ```
//!
//! [design/08 §6](../../../../plans/m1a_rust/design/08_parallelism.md#6-what-must-be-sync-and-how)
//! names four structures a worker shares and a strategy for each — a sharded
//! `RwLock` interner, a lock-free segmented fact store, a locked plan memo,
//! an immutable `KbCore`. Three of those are *write* strategies, and nobody
//! had measured the write rate. This does.
//!
//! Two questions, one per column group:
//!
//! - **does an intern table grow while it is shared?** The interner and the
//!   integer pool hand out `&str` borrowed from an arena, which no lock can
//!   do, so a shareable table is one that does not grow. Each file is loaded,
//!   saturated at root, marked, and then solved; the `syms`/`ints` columns are
//!   the growth *after* the mark, and the names are printed when there is any.
//! - **and how does the fact store's read rate compare to its write rate?**
//!   `reads` is the borrow-returning path (`rel`/`args`/`row`/`get`),
//!   `intern` is every interning call and `new` the ids they assigned. It is
//!   the *ratio* that chooses the strategy.
//!
//! The counters need `--features counters`; without it the fact-store columns
//! read zero and the interner ones still work, since those are `len()` and not
//! a counter. The wall clock is deliberately **not** printed: a build with a
//! counter on `FactStore::rel` is not the build anything ships, and the whole
//! point of this probe is the counts.
//!
//! Numbers:
//! [shared_state.md](../../../../plans/m1a_rust/p1a.7_parallelism/shared_state.md).

use ein_core::{Symbol, Terms, counters};
use ein_infer::Events;
use ein_infer::solve::{NoDumper, SolveOptions, solve};
use ein_ir::{Ast, load_file};
use std::path::PathBuf;

/// The phase's measurement set (the four entries with a search worth
/// parallelising) plus the two zebras, which are where the writeback — and so
/// the interning — is densest.
const FILES: &[&str] = &[
    "examples/zebra.ein",
    "examples/zebra2.ein",
    "examples/branching/06_lookahead_on.ein",
    "examples/branching/07_lookahead_off.ein",
    "examples/saturation/square-bwd/houses.ein",
    "examples/features/01_not_and_absent.ein",
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
    if !cfg!(feature = "counters") {
        eprintln!("note: built without --features counters; the fact-store columns are zero");
    }
    println!(
        "{:<44} {:>7} {:>7} {:>5} {:>5} {:>12} {:>11} {:>9} {:>7}",
        "workload", "enter", "facts", "syms+", "ints+", "reads", "intern", "probe", "new"
    );
    for rel in &files {
        let mut ast = Ast::new();
        let mut terms = Terms::new();
        let mut kb = load_file(&mut ast, &mut terms, &root.join(rel)).expect("loads");
        let mut events = Events::off();

        // Root saturation first, then the mark: everything after it is the
        // search, which is the only region a worker shares anything across.
        ein_infer::saturate_events(&ast, &mut terms, &mut kb).expect("root saturates");
        let (syms, ints, facts) = (terms.syms.len(), terms.ints.len(), terms.facts.len());
        counters::reset();

        let opts = SolveOptions {
            stop_after: None,
            ..SolveOptions::default()
        };
        let solved =
            solve(&mut kb, &mut terms, &ast, &mut events, &mut NoDumper, &opts).expect("solves");
        let c = counters::snapshot();
        println!(
            "{:<44} {:>7} {:>7} {:>5} {:>5} {:>12} {:>11} {:>9} {:>7}",
            rel,
            solved.stats.base.enterings_total,
            terms.facts.len() - facts,
            terms.syms.len() - syms,
            terms.ints.len() - ints,
            c.fact_read,
            c.fact_intern,
            c.fact_probe,
            c.fact_new,
        );
        for i in syms..terms.syms.len() {
            println!(
                "      interned during the search: {:?}",
                terms.sym(Symbol(i as u32))
            );
        }
    }
}
