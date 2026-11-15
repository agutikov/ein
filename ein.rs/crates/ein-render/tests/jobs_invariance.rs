//! T1a.7.5.3 — **the `--jobs` contract, as a sweep**: the whole corpus, every
//! observable surface, at one job count and at several, and nothing may move.
//!
//! This is the third sweep over [`corpus_ops`](corpus_ops), beside
//! `corpus_shapes` (digest once) and `id_order_invariance` (twice, ids
//! permuted). It is what
//! [P1a.7's acceptance](../../../../docs/history/m1a_rust/README.md#p1a7--parallelism)
//! names.
//!
//! ## What it claims, and it is stronger than the contract
//!
//! The contract ([design/08](../../../../docs/history/m1a_rust/design/08_parallelism.md)
//! §1) is that `--jobs N` is *the same computation* as `--jobs 1` — same
//! verdict, same models, same unsat core, same counters — and no wider in
//! narration than a permuted id space already is, which is
//! `id_order_invariance`'s 51-of-3160.
//!
//! **Today it is wider than that in neither direction: nothing moves at all.**
//! A worker builds its narration into its own buffer with a hole where the
//! event ordinal goes, and `Events::replay` fills it in at the ordered commit
//! ([S1a.7.2](../../../../docs/history/m1a_rust/README.md#s1a72--level-1-parallel-enterings)
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
//! ## What it cannot say, and where that is said
//!
//! *These runs agreed*, not *all runs agree* — and the distance between the
//! two is where a scheduling-dependent bug lives. The **mechanism** is written
//! beside `ein_core::Nogoods` and as
//! [design/02 §6a](../../../../docs/history/m1a_rust/design/02_determinism_and_order.md)
//! (M1e S1e.1.2, the review's Q1): the shared clause store is never read
//! during an entering and never written during a fan-out, so a worker cannot
//! observe one arriving — and the store is **frozen** for that window so the
//! premise fails loudly rather than being read off the call graph.
//!
//! ## What it runs, and against what (M1f S1f.6.39)
//!
//! **Only the ops a job count reaches**, [`Op::takes_jobs`](corpus_ops::Op::takes_jobs):
//! `corpus_ops::run_with` dispatches on that predicate, and every other op —
//! the frontend modes, the load, root's saturation and its shapes, the DOT
//! views — goes to a function with no job count to read. Until S1f.6.39 all
//! 41 ops ran at `--jobs 1` and `--jobs 2`, and 27 of them were one
//! computation run twice, as the note on `run_with` said they would be.
//! [`an_op_outside_the_predicate_ignores_jobs`] holds that side.
//!
//! **Against the golden.** `--jobs 1`'s rendering of every pair is what
//! `corpus_shapes` digests and banks (`tests/golden/corpus_shapes.md5`), over
//! the same bytes — narration and all, the checkout made portable — so a
//! `--jobs N` run whose digest is the banked one is a run that rendered the
//! same bytes, and the `--jobs 1` run is not computed again. Only a run whose
//! digest differs, or that the golden lacks, computes `--jobs 1` and reports
//! the first difference through the cut, as before. The premise is that the
//! golden is today's, and `corpus_shapes` fails in the same gate when it is
//! not; a stale golden costs this test time, never a verdict, since a
//! mismatch is what computes the baseline.
//!
//! ## Why it is cheap enough to be in `cargo test`
//!
//! It runs in-process, like its two siblings, on the worker pool
//! (`ein_corpus::pool`, `EIN_TEST_JOBS`), and the job counts it sweeps
//! are a **measurement decision**: `EIN_JOBS_SWEEP`
//! overrides them, and the default is `2` because two threads is where a
//! fan-out either commits in order or does not. `EIN_JOBS_SWEEP=2,4,8,16` is
//! the acceptance's full matrix and is what a release run should use.
//!
//! ```text
//! cargo test -p ein-render --test jobs_invariance
//! EIN_JOBS_SWEEP=2,4,8,16 cargo test -p ein-render --test jobs_invariance
//! ```

mod corpus_ops;

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use corpus_ops::{Loads, Op, digest, golden_digests, ops, pair_name, portable, run_with};
use ein_core::Terms;
use ein_corpus::{corpus_files, pool, repo_root};

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
        "this build has no fan-out: `--jobs N` is inert, so the sweep below \
         compares --jobs 1 against itself. Enable `ein-infer/parallel`."
    );
}

/// What one (file, op, jobs) cell came to.
#[derive(Debug, PartialEq)]
enum Cell {
    /// `--jobs 1` refused the file: nothing to compare, as before.
    Skipped,
    /// The same bytes as `--jobs 1`'s: by the golden's digest, or by the
    /// computed run's.
    Same { by_golden: bool },
    /// `--jobs 1` answered and `--jobs N` refused.
    Refused(String),
    /// The answer moved.
    Answer(String),
    /// Only the narration moved.
    Narration(String),
}

/// **One cell**: `--jobs N`'s run, held to the golden's digest, and only on a
/// mismatch to a computed `--jobs 1` run, through the cut.
///
/// `run` is `corpus_ops::run_with` in the sweep; a test passes a run with a
/// job count leaked into it ([`a_job_count_that_reaches_an_output_is_caught`]).
fn cell(
    rel: &Path,
    op: Op,
    jobs: usize,
    golden: &BTreeMap<String, String>,
    root: &str,
    run: &dyn Fn(Op, usize) -> Option<String>,
) -> Cell {
    let got = run(op, jobs);
    if let Some(g) = &got
        && golden
            .get(&pair_name(rel, op))
            .is_some_and(|d| *d == digest(&portable(root, g)))
    {
        return Cell::Same { by_golden: true };
    }
    let Some(base) = run(op, 1) else {
        return Cell::Skipped;
    };
    let Some(got) = got else {
        return Cell::Refused(format!(
            "{} [{op}] --jobs {jobs}: refused only under a fan-out",
            rel.display()
        ));
    };
    if got == base {
        return Cell::Same { by_golden: false };
    }
    // Something moved. The cut decides which half — the same cut
    // `id_order_invariance` applies between two runs of one engine, and for
    // the same reason: a verdict, a model, an unsat core and a counter are the
    // promise; a firing count, an event ordinal and a dying fork's stopping
    // point are narration.
    let is_answer = ein_narration::blank(&base) != ein_narration::blank(&got)
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
        Cell::Answer(line)
    } else {
        Cell::Narration(line)
    }
}

#[test]
fn jobs_does_not_move_any_observable() {
    let ops: Vec<Op> = ops().into_iter().filter(|op| op.takes_jobs()).collect();
    let files = corpus_files();
    let counts = job_counts();
    let golden = golden_digests();
    let root = repo_root();
    let root_str = root.display().to_string();
    let (mut answers, mut narrations) = (Vec::new(), Vec::new());
    let (mut compared, mut by_golden) = (0usize, 0usize);

    let n_counts = counts.len();
    let units: Vec<(usize, usize, usize)> = (0..files.len())
        .flat_map(|f| (0..ops.len()).flat_map(move |o| (0..n_counts).map(move |j| (f, o, j))))
        .collect();
    let rel = |f: usize| {
        files[f]
            .strip_prefix(&root)
            .unwrap_or(&files[f])
            .to_path_buf()
    };
    // Each program loaded once, every cell on a copy (M1f S1f.6.39,
    // Q-M1f.292).
    let loads = Loads::new(&files, &vec![ops.len() * n_counts; files.len()]);
    let cells = pool::map(
        "jobs_invariance",
        &units,
        |&(f, o, j)| {
            (
                rel(f).display().to_string(),
                format!("{} --jobs {}", ops[o], counts[j]),
            )
        },
        |&(f, o, j)| {
            let unit = loads.unit(f);
            cell(&rel(f), ops[o], counts[j], &golden, &root_str, &|op, n| {
                unit.run(op, n).0
            })
        },
    );
    for c in cells {
        match c {
            Cell::Skipped => {}
            Cell::Same { by_golden: g } => {
                compared += 1;
                by_golden += usize::from(g);
            }
            Cell::Refused(line) => answers.push(line),
            Cell::Answer(line) => {
                compared += 1;
                answers.push(line);
            }
            Cell::Narration(line) => {
                compared += 1;
                narrations.push(line);
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
    // A sweep that compared nothing that could differ would be green forever:
    // every cell here runs an op a job count reaches.
    assert!(
        compared >= 1000,
        "only {compared} cells ran an op that takes a job count — the ops \
         that can differ by job count are the ones that search, and this \
         sweep has stopped reaching them"
    );
    eprintln!(
        "jobs invariance: {compared} (file, op, jobs) cells over {} files × \
         {} ops that take a job count × {:?}, {by_golden} held to the golden's \
         digest and {} to a computed --jobs 1 run, 0 moved",
        files.len(),
        ops.len(),
        counts,
        compared - by_golden
    );
}

/// **An op outside [`Op::takes_jobs`](corpus_ops::Op::takes_jobs) ignores its
/// job count** (M1f S1f.6.39 T1f.6.39.2): on programs that load, search,
/// refuse and fail to parse, each such op renders the same at `--jobs 1` and
/// at `--jobs 7`, and the predicate holds the fourteen ops that reach a
/// region route, every one of which does take the count.
#[test]
fn an_op_outside_the_predicate_ignores_jobs() {
    let all = ops();
    let taking: Vec<String> = all
        .iter()
        .filter(|o| o.takes_jobs())
        .map(|o| o.to_string())
        .collect();
    assert_eq!(
        taking.len(),
        14,
        "the ops a job count reaches are the five read-outs, the region \
         trace, the states, the lattice, the five core sets and the oracle: \
         {taking:?}"
    );
    let root = repo_root();
    let programs = [
        "examples/saturation/type-exclusivity/pets.ein",
        "examples/branching/02_one_dead_one_alive.ein",
        "examples/lattice/06_friends_tables.ein",
        "tests/stdlib/algebra/01_copiers.ein",
        "examples/broken/absent_arity_high.ein",
    ];
    for p in programs {
        let path = root.join(p);
        assert!(path.is_file(), "{p} is gone: name another program");
        for op in all.iter().filter(|o| !o.takes_jobs()) {
            let (mut a, mut b) = (Terms::new(), Terms::new());
            assert_eq!(
                run_with(&mut a, &path, *op, 1),
                run_with(&mut b, &path, *op, 7),
                "{p} [{op}] renders differently at --jobs 7, and the op is not \
                 one Op::takes_jobs names"
            );
        }
    }
}

/// **A job count that reaches an output is caught**, by the golden's route as
/// by the computed one (M1f S1f.6.39, the stage's probe kept as a test): a
/// run that writes its job count into one op's rendering is an answer that
/// moved, whether or not the golden has the pair.
#[test]
fn a_job_count_that_reaches_an_output_is_caught() {
    let root = repo_root();
    let root_str = root.display().to_string();
    let rel = PathBuf::from("examples/branching/02_one_dead_one_alive.ein");
    let path = root.join(&rel);
    let op = ops()
        .into_iter()
        .find(|o| o.to_string() == "read-out[describe]")
        .expect("the describe read-out");
    let leaky = |op: Op, n: usize| {
        run_with(&mut Terms::new(), &path, op, n).map(|text| format!("{text}--- jobs {n}\n"))
    };
    let honest = |op: Op, n: usize| run_with(&mut Terms::new(), &path, op, n);
    let golden = golden_digests();
    assert!(
        golden.contains_key(&pair_name(&rel, op)),
        "the golden has no line for {}",
        pair_name(&rel, op)
    );
    for (what, g) in [
        ("with the golden", golden.clone()),
        ("without it", BTreeMap::new()),
    ] {
        assert_eq!(
            cell(&rel, op, 2, &g, &root_str, &honest),
            Cell::Same {
                by_golden: what == "with the golden"
            },
            "{what}: the honest run"
        );
        assert!(
            matches!(cell(&rel, op, 2, &g, &root_str, &leaky), Cell::Answer(_)),
            "{what}: a job count in the rendering is not reported as an answer"
        );
    }
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
