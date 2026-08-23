//! T1a.7.5.3 — **the `--jobs` contract, as a sweep**: the whole corpus, every
//! observable surface, at one job count and at several, and nothing may move.
//!
//! This is the third sweep over [`corpus_ops`](corpus_ops), beside
//! `corpus_shapes` (digest once) and `id_order_invariance` (twice, ids
//! permuted). It is what
//! [P1a.7's acceptance](../../../../plans/m1a_rust/p1a.7_parallelism/README.md#the-acceptance-restated)
//! names in place of `ein-conformance --tier T3`, which ran two *processes*
//! per corpus cell and left with the second engine at
//! [P1a.10](../../../../plans/m1a_rust/p1a.10_single_implementation/README.md).
//!
//! ## What it claims, and it is stronger than the contract
//!
//! The contract ([design/08](../../../../plans/m1a_rust/design/08_parallelism.md)
//! §1) is that `--jobs N` is *the same computation* as `--jobs 1` — same
//! verdict, same models, same unsat core, same counters — and no wider in
//! narration than a permuted id space already is, which is
//! `id_order_invariance`'s 51-of-3160.
//!
//! **Today it is wider than that in neither direction: nothing moves at all.**
//! A worker builds its narration into its own buffer with a hole where the
//! event ordinal goes, and `Events::replay` fills it in at the ordered commit
//! ([S1a.7.2](../../../../plans/m1a_rust/p1a.7_parallelism/s1a.7.2_parallel_enterings.md)
//! T1a.7.2.2), so the bytes are the bytes. This test therefore asserts **byte
//! equality** and reports any difference *through the cut*, so that the day
//! one appears the failure says which half it is:
//!
//! - an **answer** difference is a contract violation and always has been;
//! - a **narration** difference is admitted by the contract and has never
//!   happened, so it fails here too, with a message that says so. Relaxing
//!   that is a deliberate edit to this file and not something a run can do
//!   quietly.
//!
//! ## Why it is cheap enough to be in `cargo test`
//!
//! `ein-conformance --tier T3` cost 738 s for a full sweep because it ran two
//! processes per cell. This runs in-process, like its two siblings, and the
//! job counts it sweeps are a **measurement decision**: `EIN_JOBS_SWEEP`
//! overrides them, and the default is `2` because two threads is where a
//! fan-out either commits in order or does not. `EIN_JOBS_SWEEP=2,4,8,16` is
//! the acceptance's full matrix and is what a release run should use.
//!
//! ```text
//! cargo test -p ein-render --test jobs_invariance
//! EIN_JOBS_SWEEP=2,4,8,16 cargo test -p ein-render --test jobs_invariance
//! ```

mod corpus_ops;

use corpus_ops::{Op, ops, run_with};
use ein_core::Terms;
use ein_corpus::{corpus_files, repo_root};

/// The job counts to compare `--jobs 1` against.
///
/// Two by default, and the reason is the same one `EIN_ID_SEEDS` defaults to
/// one permutation: a second job count is more of the same question, not a
/// different one. What makes a fan-out wrong is committing out of order or
/// letting a worker see a root the sequential engine would not have given it,
/// and both are already reachable with two threads. The counts that cost
/// something are the ones that *schedule* differently — 16 on an 8-core box —
/// and those belong to a release sweep rather than to every `cargo test`.
fn job_counts() -> Vec<usize> {
    let Ok(spec) = std::env::var("EIN_JOBS_SWEEP") else {
        return vec![2];
    };
    let out: Vec<usize> = spec
        .split(',')
        .filter_map(|s| s.trim().parse().ok())
        .filter(|&n| n > 1)
        .collect();
    assert!(
        !out.is_empty(),
        "EIN_JOBS_SWEEP={spec:?} names no job count above 1 — a sweep that \
         compares --jobs 1 against itself is not a sweep"
    );
    out
}

/// Ops that can possibly differ by job count: the ones that run a solve.
///
/// Not used to *skip* anything — every op is swept, because an op that starts
/// solving later should be covered without anyone remembering to add it here.
/// It is used to assert **coverage**: if the number of solving pairs ever
/// collapses, this sweep has stopped asking its question and would still be
/// green.
fn solves(op: Op) -> bool {
    matches!(
        op,
        Op::Solve(_) | Op::Lattice | Op::Trace(_) | Op::Dump(_) | Op::Dot(_)
    )
}

/// **The sweep is not vacuous.** Without `ein-infer/parallel` every layer runs
/// on the committing thread whatever `--jobs` says, so `--jobs 8` and `--jobs
/// 1` are the same code and the whole sweep below passes by construction —
/// 20 712 cells of nothing. S1a.9.3 T1a.9.3.2 made that reachable by accident
/// (`ein-render` stopped taking `ein-infer`'s defaults), so it is a failure
/// here rather than a silent green, the way `dot_wellformed` fails without
/// Graphviz.
#[test]
fn the_sweep_is_not_vacuous() {
    assert!(
        ein_infer::build::features().contains(&"parallel"),
        "this build has no fan-out: `--jobs N` is inert, so the sweep below          compares --jobs 1 against itself. Enable `ein-infer/parallel`."
    );
}

#[test]
fn jobs_does_not_move_any_observable() {
    let ops = ops();
    let files = corpus_files();
    let counts = job_counts();
    let (mut answers, mut narrations) = (Vec::new(), Vec::new());
    let (mut compared, mut solving) = (0usize, 0usize);

    for path in &files {
        let rel = path.strip_prefix(repo_root()).unwrap_or(path);
        for op in &ops {
            let mut base_terms = Terms::new();
            let Some(base) = run_with(&mut base_terms, path, *op, 1) else {
                continue;
            };
            for &jobs in &counts {
                let mut terms = Terms::new();
                let Some(got) = run_with(&mut terms, path, *op, jobs) else {
                    answers.push(format!(
                        "{} [{op}] --jobs {jobs}: refused only under a fan-out",
                        rel.display()
                    ));
                    continue;
                };
                compared += 1;
                if solves(*op) {
                    solving += 1;
                }
                if got == base {
                    continue;
                }
                // Something moved. The cut decides which half — the same cut
                // `id_order_invariance` applies between two runs of one
                // engine, and for the same reason: a verdict, a model, an
                // unsat core and a counter are the promise; a firing count,
                // an event ordinal and a dying fork's stopping point are
                // narration.
                let is_answer = ein_parity::blank(&base) != ein_parity::blank(&got)
                    && match (op.narrow(&base), op.narrow(&got)) {
                        (Some(x), Some(y)) => x != y,
                        _ => false,
                    };
                let line = format!(
                    "{} [{op}] --jobs {jobs}\n{}",
                    rel.display(),
                    first_difference(&base, &got)
                );
                if is_answer {
                    answers.push(line);
                } else {
                    narrations.push(line);
                }
            }
        }
    }

    assert!(
        answers.is_empty(),
        "{} of {compared} (file, op, jobs) cells changed the ANSWER — \
         `--jobs N` is not the same computation as `--jobs 1`:\n\n{}",
        answers.len(),
        answers
            .iter()
            .take(10)
            .cloned()
            .collect::<Vec<_>>()
            .join("\n\n")
    );
    // The contract admits this; the engine has never done it. Failing here is
    // the point — see the module note.
    assert!(
        narrations.is_empty(),
        "{} of {compared} (file, op, jobs) cells moved in NARRATION. The \
         contract admits that — a firing count, an event ordinal, a dying \
         fork's stopping point — but nothing in this engine has ever moved \
         under a job count, because a worker's events get their ordinals at \
         the ordered commit. So this is a finding rather than a tolerance: \
         either the ordinal moved off the commit, or something a worker \
         computes leaked into a rendering. If the change is deliberate, \
         relax this assertion on purpose:\n\n{}",
        narrations.len(),
        narrations
            .iter()
            .take(10)
            .cloned()
            .collect::<Vec<_>>()
            .join("\n\n")
    );
    // A sweep that compared nothing that could differ would be green forever.
    assert!(
        solving >= 1000,
        "only {solving} of {compared} cells ran a solve — the ops that can \
         differ by job count are the ones that search, and this sweep has \
         stopped reaching them"
    );
    eprintln!(
        "jobs invariance: {compared} (file, op, jobs) cells over {} files × \
         {} ops × {:?}, {solving} of them running a solve, 0 moved",
        files.len(),
        ops.len(),
        counts
    );
}

fn first_difference(a: &str, b: &str) -> String {
    for (i, (x, y)) in a.lines().zip(b.lines()).enumerate() {
        if x != y {
            return format!("  line {}\n    --jobs 1: {x}\n    --jobs N: {y}", i + 1);
        }
    }
    format!(
        "  same {} lines, then {} vs {}",
        a.lines().count().min(b.lines().count()),
        a.lines().count(),
        b.lines().count()
    )
}
