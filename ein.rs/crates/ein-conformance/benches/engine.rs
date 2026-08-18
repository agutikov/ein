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

use std::path::PathBuf;

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

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../..")
        .canonicalize()
        .expect("repo root")
}

/// `parse` — zebra2.ein, zebra.ein and the seven stdlib modules, in one
/// measurement, because that is the unit `utils/bench_baseline.py` times on
/// the Python side (S1a.1.1).
fn frontend(c: &mut Criterion) {
    let root = repo_root();
    let mut sources: Vec<(String, String)> = Vec::new();
    for rel in ["examples/zebra2.ein", "examples/zebra.ein"] {
        let path = root.join(rel);
        sources.push((rel.to_string(), std::fs::read_to_string(&path).expect(rel)));
    }
    let stdlib = root.join("stdlib");
    let mut modules: Vec<PathBuf> = std::fs::read_dir(&stdlib)
        .expect("stdlib")
        .flatten()
        .map(|e| e.path())
        .filter(|p| p.extension().is_some_and(|x| x == "ein"))
        .collect();
    modules.sort();
    for path in modules {
        let name = path
            .file_name()
            .expect("name")
            .to_string_lossy()
            .to_string();
        sources.push((name, std::fs::read_to_string(&path).expect("module")));
    }

    let mut group = c.benchmark_group("parse");
    group.bench_function("corpus", |b| {
        b.iter(|| {
            let mut ast = ein_ir::Ast::new();
            for (name, text) in &sources {
                ein_ir::parse(&mut ast, text, Some(name)).expect("parses");
            }
            std::hint::black_box(&ast);
        })
    });
    group.bench_function("zebra2", |b| {
        let text = &sources[0].1;
        b.iter(|| {
            let mut ast = ein_ir::Ast::new();
            let forms = ein_ir::parse(&mut ast, text, Some("zebra2.ein")).expect("parses");
            std::hint::black_box(forms.len());
        })
    });
    group.bench_function("zebra2_resolve", |b| {
        // P1a.1's acceptance number: parse **plus** import resolution and
        // macro expansion of zebra2, which pulls three stdlib modules in.
        let text = &sources[0].1;
        let base = root.join("examples");
        b.iter(|| {
            let mut ast = ein_ir::Ast::new();
            let forms = ein_ir::parse(&mut ast, text, Some("zebra2.ein")).expect("parses");
            let resolved = ein_ir::imports::Resolver::new()
                .resolve_imports(&mut ast, &forms, Some(&base))
                .expect("resolves");
            let macros = ein_ir::macros::collect_macros(&ast, &resolved);
            let expanded =
                ein_ir::macros::expand_rule_clauses(&mut ast, &resolved, &macros).expect("expands");
            std::hint::black_box(expanded.len());
        })
    });
    group.finish();

    // parse + import resolution + macro expansion + index build (S1a.2.3).
    let mut group = c.benchmark_group("load");
    group.bench_function("zebra2", |b| {
        let path = root.join("examples/zebra2.ein");
        b.iter(|| {
            let mut ast = ein_ir::Ast::new();
            let mut terms = ein_core::Terms::new();
            let kb = ein_ir::load_file(&mut ast, &mut terms, &path).expect("loads");
            std::hint::black_box(kb.n_facts());
        })
    });
    group.finish();
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
    // which is a different question from "is one fork fast" (S1a.2.2).
    let root = repo_root();
    let mut ast = ein_ir::Ast::new();
    let mut terms = ein_core::Terms::new();
    let mut kb =
        ein_ir::load_file(&mut ast, &mut terms, &root.join("examples/zebra2.ein")).expect("loads");
    let rel = terms.intern_text("__bench__").expect("room");
    let args = [
        terms.value_text("a").expect("room"),
        terms.value_text("b").expect("room"),
    ];
    let mut group = c.benchmark_group("fork");
    group.bench_function("zebra2", |b| {
        b.iter(|| {
            // Each fork is fresh, so the same fact is new every time — the
            // shape `utils/bench_baseline.py::_fork_thunk` measures. Its root
            // is *saturated*; this one is merely loaded, until P1a.3.
            let mut child = kb.fork();
            child
                .add_and_index_fact(&mut terms, rel, &args, None)
                .expect("room");
            std::hint::black_box(child.n_facts());
        })
    });
    group.finish();
}

fn search(c: &mut Criterion) {
    pending(c, "solve_fast", "P1a.4");
    pending(c, "solve_exhaustive", "P1a.4");
}

criterion_group!(benches, frontend, deductive, search);
criterion_main!(benches);
