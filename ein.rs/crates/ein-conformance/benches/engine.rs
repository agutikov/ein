//! The M1a benchmark set (T1a.0.4.5) —
//! [design/12](../../../../plans/m1a_rust/design/12_toolchain_and_layout.md) §4.
//!
//! Eight benches, matching `utils/bench_baseline.py` name for name so
//! `design/README.md` § Measured can put the two implementations in adjacent
//! columns and have the comparison mean something.
//!
//! The set was fixed at P1a.0, before there was any result to be tempted by —
//! a benchmark set chosen after seeing the numbers measures what the
//! implementation happens to be good at — and each row went live as its engine
//! landed. **As of S1a.4.5 none is pending**: the last two, `solve_fast` and
//! `solve_exhaustive`, are the two the whole set exists for.
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

/// The allocator `ein` ships with — see `ein-cli/src/main.rs`. Without this
/// line the bench set would time the system allocator while the binary ran on
/// snmalloc, and §6's table would stop agreeing with §1's.
/// `--no-default-features` is the system-allocator arm.
#[cfg(feature = "snmalloc")]
#[global_allocator]
static GLOBAL: snmalloc_rs::SnMalloc = snmalloc_rs::SnMalloc;
use ein_infer::SharedMemo;

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
        memo: SharedMemo::default(),
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
                    memo: SharedMemo::default(),
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
    // **Both puzzles**, because `utils/bench_baseline.py::boundary` times
    // *zebra2* and this group timed only *zebra*: the two `boundary` rows were
    // put side by side in a comparison table at S1a.6.1 and are not the same
    // workload. Adding the missing case is cheaper than remembering the
    // asymmetry, and `zebra` stays because design/README quotes its 80 % split.
    let mut group = c.benchmark_group("boundary");
    for rel in ["examples/zebra.ein", "examples/zebra2.ein"] {
        let name = rel.trim_start_matches("examples/").trim_end_matches(".ein");
        group.bench_function(name, |b| {
            let path = repo_root().join(rel);
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
                        memo: SharedMemo::default(),
                    };
                    let mut sat = ein_infer::Saturator::new(&mut s).expect("compiles");
                    sat.saturate(&mut s, None, &mut |_| {}).expect("saturates");
                    std::hint::black_box(sat.boundary_nanos);
                },
                criterion::BatchSize::SmallInput,
            )
        });
    }
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

/// The two the whole set exists for: `solve zebra2.ein` on the default
/// `stop_after = 1` path and exhaustively.
///
/// **These measure the search, not the run.** The Python column above is
/// end-to-end, and its own attribution puts parse at 200 ms and load at 430 ms
/// of `solve_fast`'s 1 870; parse and load have their own benches here and are
/// 1 003× and 607× respectively, so folding them in would flatter this row
/// with two others' results. What is timed is what P1a.4 wrote: root
/// saturation plus the hypothesis search.
///
/// The batch reloads the KB every iteration, because a solve mutates root —
/// the singleton `(not h)` writeback and the forced-positive promotions are
/// root writes by design — so a second solve on the same KB is a different
/// problem.
fn search(c: &mut Criterion) {
    let path = repo_root().join("examples/zebra2.ein");
    for (name, stop_after) in [("solve_fast", Some(1)), ("solve_exhaustive", None)] {
        let mut group = c.benchmark_group(name);
        group.sample_size(10);
        group.bench_function("zebra2", |b| {
            b.iter_batched(
                || {
                    let mut ast = ein_ir::Ast::new();
                    let mut terms = ein_core::Terms::new();
                    let kb = ein_ir::load_file(&mut ast, &mut terms, &path).expect("loads");
                    (ast, terms, kb)
                },
                |(ast, mut terms, mut kb)| {
                    let mut events = ein_infer::Events::off();
                    let opts = ein_infer::SolveOptions {
                        stop_after,
                        ..ein_infer::SolveOptions::default()
                    };
                    let solved = ein_infer::solve(
                        &mut kb,
                        &mut terms,
                        &ast,
                        &mut events,
                        &mut ein_infer::NoDumper,
                        &opts,
                    )
                    .expect("solves");
                    std::hint::black_box(solved.stats.base.enterings_total);
                },
                criterion::BatchSize::SmallInput,
            )
        });
        group.finish();
    }
}

criterion_group!(benches, frontend, deductive, search);
criterion_main!(benches);
