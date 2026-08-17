//! The M1a benchmark set (T1a.0.4.5) —
//! [design/12](../../../../plans/m1a_rust/design/12_toolchain_and_layout.md) §4.
//!
//! Eight benches, matching `utils/bench_baseline.py` name for name so
//! `design/README.md` § Measured can put the two implementations in adjacent
//! columns and have the comparison mean something.
//!
//! **Every bench is `ignore`d until the engine it measures exists.** They live
//! here from P1a.0 for two reasons that are worth more than the zeros they
//! currently report: the harness compiles and runs in CI from the start, so
//! nobody has to build one under time pressure at
//! [P1a.6](../../../../plans/m1a_rust/p1a.6_performance/README.md); and the
//! *set* is fixed now, before there is any result to be tempted by. A
//! benchmark set chosen after seeing the numbers measures what the
//! implementation happens to be good at.
//!
//! The Python column, for reference (CPython 3.14, dev machine, 2026-08-17):
//!
//! | bench | ein.py |
//! |---|---:|
//! | `solve_exhaustive` (zebra2) | 5 630 ms |
//! | `solve_fast` (zebra2) | 1 870 ms |
//! | parse | 200 ms |
//! | load | 430 ms |
//! | `saturate_root` | 90 ms |
//!
//! Refresh both with one command each:
//! `python3 utils/bench_baseline.py --json …` and `cargo bench`.

use criterion::{Criterion, criterion_group, criterion_main};

/// Marks a bench whose engine has not landed. It reports itself once and
/// measures nothing — a zero would be worse than a message, because a zero
/// in a report looks like a result.
fn pending(c: &mut Criterion, name: &str, lands_in: &str) {
    let mut group = c.benchmark_group(name);
    group.sample_size(10);
    eprintln!("bench {name}: pending — the engine lands in {lands_in}");
    group.finish();
}

fn frontend(c: &mut Criterion) {
    // zebra2.ein, zebra.ein and the seven stdlib modules.
    pending(c, "parse", "P1a.1");
    // parse + import resolution + macro expansion + index build.
    pending(c, "load", "P1a.2");
}

fn deductive(c: &mut Criterion) {
    pending(c, "saturate_root", "P1a.3");
    // `match::run` over the saturated root, per plan. 46 % of ein.py's self
    // time, and the bench the register matcher (design/05) has to move.
    pending(c, "match_hot", "P1a.3");
    // One `_admit_from_boundary` round. 72 % of ein.py's exhaustive profile
    // sits under this call (design/06 § Boundary).
    pending(c, "boundary", "P1a.3");
    // Fork + first delta write. Already free in ein.py (0.003 s / 206 calls)
    // — it is measured because P1a.7 needs hundreds of thousands of them,
    // which is a different question from "is one fork fast".
    pending(c, "fork", "P1a.2");
}

fn search(c: &mut Criterion) {
    pending(c, "solve_fast", "P1a.4");
    pending(c, "solve_exhaustive", "P1a.4");
}

criterion_group!(benches, frontend, deductive, search);
criterion_main!(benches);
