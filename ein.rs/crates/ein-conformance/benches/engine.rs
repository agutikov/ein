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

/// A loaded zebra2, and the same one saturated to its root fixpoint.
fn zebra2_root() -> (ein_ir::Ast, ein_core::Terms, ein_core::Kb) {
    let root = repo_root();
    let mut ast = ein_ir::Ast::new();
    let mut terms = ein_core::Terms::new();
    let mut kb =
        ein_ir::load_file(&mut ast, &mut terms, &root.join("examples/zebra2.ein")).expect("loads");
    let mut events = ein_infer::Events::off();
    let mut s = ein_infer::Session {
        kb: &mut kb,
        terms: &mut terms,
        ast: &ast,
        events: &mut events,
    };
    let mut sat = ein_infer::Saturator::new(&mut s).expect("compiles");
    sat.saturate(&mut s, None, &mut |_| {}).expect("saturates");
    (ast, terms, kb)
}

fn deductive(c: &mut Criterion) {
    // Root saturation from a freshly loaded KB — ein.py: 90 ms (S1a.3.3).
    let mut group = c.benchmark_group("saturate_root");
    group.bench_function("zebra2", |b| {
        let path = repo_root().join("examples/zebra2.ein");
        // Batched, so the load is setup and only the saturation is timed —
        // the same split `utils/bench_baseline.py` reports on the Python side.
        b.iter_batched(
            || {
                let mut ast = ein_ir::Ast::new();
                let mut terms = ein_core::Terms::new();
                let kb = ein_ir::load_file(&mut ast, &mut terms, &path).expect("loads");
                (ast, terms, kb)
            },
            |(ast, mut terms, mut kb)| {
                let mut events = ein_infer::Events::off();
                let mut s = ein_infer::Session {
                    kb: &mut kb,
                    terms: &mut terms,
                    ast: &ast,
                    events: &mut events,
                };
                let mut sat = ein_infer::Saturator::new(&mut s).expect("compiles");
                let n = sat.saturate(&mut s, None, &mut |_| {}).expect("saturates");
                std::hint::black_box(n);
            },
            criterion::BatchSize::SmallInput,
        )
    });
    group.finish();

    // `match::run` over the saturated root, every plan. 46 % of ein.py's self
    // time, and the bench the register matcher (design/05) has to move. The
    // comparable *work* is the call counts, not the wall clock: ein.py makes
    // 6.0 M `_bind_arg` and 4.6 M `_bind_args` calls on the exhaustive solve
    // this root feeds.
    let (ast, terms, kb) = zebra2_root();
    let rules: Vec<_> = kb.program().rules.values().cloned().collect();
    let mut terms_mut = terms;
    let mut plans = Vec::new();
    for rule in &rules {
        for activator in ein_infer::activators_for(&kb, &terms_mut, rule) {
            plans.push(
                ein_infer::compile_rule(&ast, &mut terms_mut, rule, activator).expect("compiles"),
            );
        }
    }
    let terms = terms_mut;
    let mut group = c.benchmark_group("match_hot");
    group.bench_function("zebra2", |b| {
        let mut m = ein_infer::Matcher::new();
        b.iter(|| {
            let mut n = 0usize;
            for plan in &plans {
                m.run(&kb, &terms, &ast, plan, &mut |mt| {
                    n += mt.premises().len();
                    std::ops::ControlFlow::Continue(())
                });
            }
            std::hint::black_box(n);
        })
    });
    group.finish();

    // The boundary. 72 % of ein.py's exhaustive profile sits under
    // `_admit_from_boundary` (design/06 § Boundary), and **80 % of a `zebra`
    // root saturation** in ein.rs — which is why the workload is that
    // saturation rather than a single round: a round is not repeatable
    // without rebuilding the state that produced it, and the state is the
    // saturation. `examples/engine_cost.rs` reports the split, so the share
    // this bench measures is a number rather than an assumption.
    let mut group = c.benchmark_group("boundary");
    group.bench_function("zebra", |b| {
        let path = repo_root().join("examples/zebra.ein");
        b.iter_batched(
            || {
                let mut ast = ein_ir::Ast::new();
                let mut terms = ein_core::Terms::new();
                let kb = ein_ir::load_file(&mut ast, &mut terms, &path).expect("loads");
                (ast, terms, kb)
            },
            |(ast, mut terms, mut kb)| {
                let mut events = ein_infer::Events::off();
                let mut s = ein_infer::Session {
                    kb: &mut kb,
                    terms: &mut terms,
                    ast: &ast,
                    events: &mut events,
                };
                let mut sat = ein_infer::Saturator::new(&mut s).expect("compiles");
                sat.saturate(&mut s, None, &mut |_| {}).expect("saturates");
                std::hint::black_box(sat.boundary_nanos);
            },
            criterion::BatchSize::SmallInput,
        )
    });
    group.finish();
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
