//! **The order of the obligation tree decides the work and never the
//! answer**, and **the work does not read the order a program states its
//! facts in**: `ein_infer::order`, held two ways on the programs past the
//! cube.
//!
//! - **Every order, one answer.** Each of the four questions the tree
//!   answers, `--single`, `-n 1`, `--minimal` and `--maximal`, on every
//!   corpus program and regression witness that searches, asked under the
//!   order the routes take and under every other order of [`orders`]: the
//!   written order, its reverse, text, seeded permutations of the choice,
//!   each queue, a learned order that starts the tree again as it goes, and
//!   the structure read at other moments than the routes read it, so that a
//!   tree started again at its third node is held as the one that never is.
//!   `ein_infer::order::with` chooses one for the calling thread; no lever
//!   reaches it.
//! - **The two SENDs.** `send_more_money.ein` and
//!   `send_more_money_written.ein` are one program but for the order of
//!   eight facts (`cli_semantics.rs` holds that), and under the order the
//!   routes take `--single` and the maximal line do the same work on both,
//!   to the saturation and the fact, and list the same states. `-n N` and
//!   the minimal line are the questions whose last key is the order the
//!   obligation pass reports, so there the two are two runs, and the test
//!   says so.
//!
//! The oracle holds every order on every cube it enumerates
//! (`explore.rs`, `every_order_of_the_tree_is_the_oracle`); this file holds
//! the programs past the cube, by what a describe or a whole line says.
//!
//! ## What *one answer* means
//!
//! A run **finished** when no budget, limit or deadline stopped it. An order
//! may finish where another is stopped: that is the work, and it is what an
//! order is for.
//!
//! - **Both finished**: one verdict and one `exhausted`. A part of `R` both
//!   list whole is one set of states and one set of minimal commitments. A
//!   listing a question stopped as asked (`-n 1`, a `--single` that found
//!   two) is as many states from each; **which** states is the order's.
//! - **Either stopped**: a verdict both reach is one verdict, and a stopped
//!   run claims no part of `R` whole.
//! - **Every state any run lists is a state of `R`**: of the describe where
//!   it finishes, else of the maximal line where the routes' order lists it
//!   whole, a state on which is in `R`. A listing that is the routes' own
//!   again, state for state, is that run's, which `explore.rs` and
//!   `known_dead.rs` hold; one that is another and that neither a describe
//!   nor a line holds fails the test.
//!
//! The corpus's `slow` entries are read under `EIN_CORPUS_SLOW=1`, under
//! fewer orders; the two zebra puzzles every run. It runs in shards,
//! `EIN_TEST_SHARD=k/n`, each program in one
//! (`ein_corpus::shard`), and the two SENDs' test whole in one of them.
//!
//! ```text
//! cargo test -p ein-infer --profile gate --test order
//! EIN_CORPUS_SLOW=1 EIN_TEST_SHARD=1/4 cargo test -p ein-infer --profile gate --test order
//! ```

use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

use ein_core::{Kb, Terms};
use ein_corpus::repo_root;
use ein_infer::explore::{self, Answer, Ask, Mode, Verdict, Which};
use ein_infer::order::{self, Blame, Choice, Order, Queue, Structure};
use ein_ir::{Ast, load, parse};

// What *a program searches* means, by the determinism sweeps' one
// definition.
#[allow(dead_code)]
#[path = "../../ein-render/tests/corpus_ops/searches.rs"]
mod searching;

fn loaded(path: &Path) -> Option<(Kb, Terms, Ast)> {
    let text = std::fs::read_to_string(path).ok()?;
    let mut ast = Ast::new();
    let mut terms = Terms::new();
    let forms = parse(&mut ast, &text, path.to_str()).ok()?;
    let kb = load(&mut ast, &mut terms, &forms, path.parent()).ok()?;
    Some((kb, terms, ast))
}

fn rel_of(path: &Path) -> String {
    path.strip_prefix(repo_root())
        .unwrap_or(path)
        .to_string_lossy()
        .to_string()
}

/// The programs every run reads, `slow` or not.
const INSTANCES: [&str; 2] = ["examples/zebra.ein", "examples/zebra2.ein"];

const SEND: &str = "examples/beyond_arith/send_more_money.ein";
/// The shard (`EIN_TEST_SHARD`) [`the_two_sends_take_the_same_work`] runs
/// whole in.
const SENDS_SHARD: usize = 4;
const SEND_WRITTEN: &str = "examples/beyond_arith/send_more_money_written.ein";

/// Every corpus file and regression witness, once: the `slow` entries that
/// are not [`INSTANCES`] where `slow` is asked, and the others where it is
/// not.
fn files(slow: bool) -> Vec<PathBuf> {
    let slow_set: BTreeSet<PathBuf> = ein_corpus::corpus()
        .entry
        .iter()
        .filter(|e| e.slow && !INSTANCES.contains(&e.path.as_str()))
        .map(|e| repo_root().join(&e.path))
        .collect();
    let mut files = ein_corpus::corpus_files();
    for row in ein_corpus::regressions().case {
        files.push(repo_root().join(&row.witness));
    }
    files.sort();
    files.dedup();
    files.retain(|f| slow_set.contains(f) == slow);
    files
}

/// The four questions the tree answers.
fn questions() -> [(&'static str, Mode); 4] {
    [
        (
            "--single",
            Mode {
                which: Which::All,
                ask: Ask::Single,
            },
        ),
        (
            "-n 1",
            Mode {
                which: Which::All,
                ask: Ask::Find(1),
            },
        ),
        (
            "--minimal",
            Mode {
                which: Which::Minimal,
                ask: Ask::Describe,
            },
        ),
        (
            "--maximal",
            Mode {
                which: Which::Maximal,
                ask: Ask::Describe,
            },
        ),
    ]
}

fn an(choice: Choice, queue: Queue, structure: Structure, restarts: bool) -> Order {
    Order {
        choice,
        queue,
        structure,
        restarts,
    }
}

/// Which of [`orders`] a run takes.
#[derive(Clone, Copy, PartialEq, Eq)]
enum Set {
    /// Those the `slow` programs are read under: the first four.
    Few,
    /// Those every commit reads the other programs under: six, one of each
    /// way an order can differ from the routes' own.
    Commit,
    /// All of them.
    All,
}

/// The orders the routes' own is held against.
fn orders(set: Set) -> Vec<(&'static str, Order)> {
    let node = |after| Structure::Node {
        after,
        one_step: false,
        share: 0,
    };
    let mut v = vec![
        ("the written order", Order::WRITTEN),
        (
            "its reverse",
            an(Choice::Reversed, Queue::BySize, Structure::None, false),
        ),
        (
            "a permutation, seed 1",
            an(Choice::Shuffled(1), Queue::Deepest, Structure::None, false),
        ),
        (
            "the node read at the third saturation, the tree started again",
            an(Choice::CoupledOpen, Queue::BySize, node(3), false),
        ),
    ];
    if set == Set::Few {
        return v;
    }
    v.extend([
        (
            "by the couplings, read at the second saturation within a third of the facts",
            an(
                Choice::Coupled { written: false },
                Queue::BySize,
                Structure::Node {
                    after: 2,
                    one_step: false,
                    share: 3,
                },
                false,
            ),
        ),
        (
            "learned, by the path, the largest state first",
            an(
                Choice::Learned(Blame::Path),
                Queue::Deepest,
                Structure::None,
                true,
            ),
        ),
    ]);
    if set == Set::Commit {
        return v;
    }
    v.extend([
        (
            "the written order without the denials' count",
            an(
                Choice::Written { denials: false },
                Queue::BySize,
                Structure::None,
                false,
            ),
        ),
        (
            "text",
            an(Choice::Text, Queue::BySize, Structure::None, false),
        ),
        (
            "text, the largest state first",
            an(Choice::Text, Queue::Deepest, Structure::None, false),
        ),
        (
            "a permutation, seed 2",
            an(Choice::Shuffled(2), Queue::BySize, Structure::None, false),
        ),
        (
            "a permutation, seed 3, started again as it goes",
            an(Choice::Shuffled(3), Queue::Deepest, Structure::None, true),
        ),
        (
            "the node read at the first tie",
            an(Choice::OpenCoupled, Queue::BySize, node(0), false),
        ),
        (
            "the node first, read at the first choice, the largest state first",
            an(Choice::CoupledOpen, Queue::Deepest, node(0), false),
        ),
        (
            "the edges one application back, read at the fifth saturation",
            an(
                Choice::OpenCoupled,
                Queue::Deepest,
                Structure::Node {
                    after: 5,
                    one_step: true,
                    share: 0,
                },
                false,
            ),
        ),
        (
            "by the couplings, written among equals, read at the fourth, the largest state first",
            an(
                Choice::Coupled { written: true },
                Queue::Deepest,
                node(4),
                false,
            ),
        ),
        (
            "learned, by the branch, started again as it goes",
            an(
                Choice::Learned(Blame::Branch),
                Queue::BySize,
                Structure::None,
                true,
            ),
        ),
    ]);
    v
}

/// One question of `path` at the default budget, under `order` or the
/// routes' own.
fn ask(path: &Path, mode: Mode, order: Option<&Order>) -> Result<Answer, String> {
    let go = || {
        let (mut kb, mut terms, ast) = loaded(path).expect("loaded once already");
        explore::run(&mut kb, &mut terms, &ast, &explore::Options::new(mode))
            .map_err(|e| e.to_string())
    };
    match order {
        Some(o) => order::with(o.clone(), go),
        None => go(),
    }
}

/// A listing as a set of states, each its closure.
fn closures(a: &Answer) -> BTreeSet<Vec<u32>> {
    a.states.iter().map(|s| s.closure.to_vec()).collect()
}

fn minimal(a: &Answer) -> BTreeSet<Vec<u32>> {
    a.minimal.iter().map(|(g, _)| g.to_vec()).collect()
}

#[derive(Default)]
struct Tally {
    failures: Vec<String>,
    /// Cells asked under another order than the routes'.
    cells: usize,
    /// Of them, answered by a tree that branched on an instance.
    trees: usize,
    /// Cells whose work is another under the other order.
    other_work: usize,
    /// Cells whose listing, stopped as asked both ways, holds other states.
    other_states: usize,
    /// Cells one order finished and the other did not.
    one_stopped: usize,
    /// A listing no finished describe and no whole maximal line holds.
    unheld: Vec<String>,
}

/// What one program that searches answers under the routes' order, which
/// every other order's answers are held to: the four questions, R as a
/// describe lists it where it finishes, and the maximal line where the
/// routes' order lists it whole.
struct Base {
    rel: String,
    r: Option<BTreeSet<Vec<u32>>>,
    line: Option<BTreeSet<Vec<u32>>>,
    asked: Vec<(&'static str, Mode, Result<Answer, String>)>,
    /// The routes' own listing, where the describe does not hold it.
    failures: Vec<String>,
}

/// [`Base`] of `path`, or `None` where it does not load or does not search.
fn base(path: &Path) -> Option<Base> {
    let (mut kb, mut terms, ast) = loaded(path)?;
    if !searching::searches(&ast, &mut terms, &mut kb) {
        return None;
    }
    let rel = rel_of(path);
    // R as a describe lists it, where it finishes: what every state a run
    // lists is held to. Where it does not, the maximal line under the
    // routes' order, listed whole.
    let describe = Mode {
        which: Which::All,
        ask: Ask::Describe,
    };
    let r: Option<BTreeSet<Vec<u32>>> = match ask(path, describe, None) {
        Ok(a) if a.exhausted && a.stopped.is_none() => a
            .described
            .as_ref()
            .map(|d| d.states.iter().map(|s| s.closure.to_vec()).collect()),
        _ => None,
    };
    let mut line: Option<BTreeSet<Vec<u32>>> = None;
    let mut failures = Vec::new();
    let mut asked: Vec<(&'static str, Mode, Result<Answer, String>)> = Vec::new();
    for (flags, mode) in questions() {
        let a = ask(path, mode, None);
        if let Ok(a) = &a
            && mode.which == Which::Maximal
            && a.exhausted
            && a.stopped.is_none()
        {
            line = Some(closures(a));
        }
        // The routes' own listing is held to the describe as every other
        // order's is.
        if let (Ok(a), Some(r)) = (&a, &r)
            && !closures(a).is_subset(r)
        {
            failures.push(format!(
                "{rel} {flags}, the routes' order: lists a state the describe does not"
            ));
        }
        asked.push((flags, mode, a));
    }
    Some(Base {
        rel,
        r,
        line,
        asked,
        failures,
    })
}

/// One program's four questions under one other order, held to its
/// [`Base`] by the module's definition of *one answer*.
fn held(path: &Path, base: &Base, name: &'static str, o: &Order) -> Tally {
    let mut t = Tally {
        failures: base.failures.clone(),
        ..Tally::default()
    };
    let rel = &base.rel;
    for (flags, mode, a) in &base.asked {
        let at = format!("{rel} {flags}, {name}");
        let (a, b) = match (a, ask(path, *mode, Some(o))) {
            (Ok(a), Ok(b)) => (a, b),
            (Err(x), Err(y)) if *x == y => continue,
            (x, y) => {
                t.failures.push(format!(
                    "{at}: {:?} against {:?}",
                    x.as_ref().err(),
                    y.err()
                ));
                continue;
            }
        };
        t.cells += 1;
        t.trees += usize::from(b.route == explore::Route::Tree);
        t.other_work += usize::from((a.saturations, a.facts) != (b.saturations, b.facts));
        if a.verdict.is_some() && b.verdict.is_some() && a.verdict != b.verdict {
            t.failures
                .push(format!("{at}: {:?} against {:?}", b.verdict, a.verdict));
        }
        for x in [a, &b] {
            if x.stopped.is_some() && x.exhausted {
                t.failures
                    .push(format!("{at}: a stopped run says it listed a part whole"));
            }
            if x.stopped.is_some()
                && mode.ask == Ask::Single
                && !matches!(x.verdict, None | Some(Verdict::Ambiguity))
            {
                t.failures
                    .push(format!("{at}: a stopped --single says {:?}", x.verdict));
            }
        }
        match (a.stopped.is_some(), b.stopped.is_some()) {
            (false, false) => {
                if (a.verdict, a.exhausted) != (b.verdict, b.exhausted) {
                    t.failures.push(format!(
                        "{at}: {:?}, exhausted {}, against {:?}, exhausted {}",
                        b.verdict, b.exhausted, a.verdict, a.exhausted
                    ));
                } else if a.exhausted {
                    if closures(a) != closures(&b) || minimal(a) != minimal(&b) {
                        t.failures
                            .push(format!("{at}: one part of R, listed whole, two ways"));
                    }
                } else if a.states.len() != b.states.len() {
                    t.failures.push(format!(
                        "{at}: {} states listed against {}",
                        b.states.len(),
                        a.states.len()
                    ));
                } else if closures(a) != closures(&b) {
                    t.other_states += 1;
                }
            }
            (true, true) => {}
            _ => t.one_stopped += 1,
        }
        match &base.r {
            Some(r) => {
                if !closures(&b).is_subset(r) {
                    t.failures
                        .push(format!("{at}: lists a state the describe does not"));
                }
            }
            None => {
                let states = closures(&b);
                if states != closures(a)
                    && !states.is_empty()
                    && !base.line.as_ref().is_some_and(|l| states.is_subset(l))
                {
                    t.unheld.push(at);
                }
            }
        }
    }
    t
}

/// **Every order gives one answer**, on every corpus program and register
/// witness that searches: the module's definition, cell by cell.
#[test]
fn every_order_gives_one_answer() {
    let slow = std::env::var("EIN_CORPUS_SLOW").as_deref() == Ok("1");
    // By default every program that is not `slow`, under six orders.
    // `EIN_CORPUS_SLOW=1` reads those under every order, and the `slow` ones
    // under the first four.
    let (few, most) = if slow {
        (orders(Set::Few), orders(Set::All))
    } else {
        (Vec::new(), orders(Set::Commit))
    };
    let mut units: Vec<(PathBuf, bool)> = files(false).into_iter().map(|f| (f, false)).collect();
    if slow {
        units.extend(files(true).into_iter().map(|f| (f, true)));
    }
    // `EIN_TEST_SHARD` runs this in shards, a program in one of them.
    let units = ein_corpus::shard::keep(units, |(p, _)| rel_of(p));
    let orders = &most;
    // Each program's answers under the routes' order, then one unit per
    // program and other order, so that a long program's orders run side by
    // side.
    let bases = ein_corpus::pool::map(
        "order::every_order_gives_one_answer::base",
        &units,
        |(p, _)| (rel_of(p), String::new()),
        |(p, _)| base(p),
    );
    let mut pairs: Vec<(usize, usize)> = Vec::new();
    for (i, (b, (_, is_slow))) in bases.iter().zip(&units).enumerate() {
        if b.is_some() {
            let n = if *is_slow { few.len() } else { most.len() };
            pairs.extend((0..n).map(|j| (i, j)));
        }
    }
    let order_of = |i: usize, j: usize| if units[i].1 { &few[j] } else { &most[j] };
    let results = ein_corpus::pool::map(
        "order::every_order_gives_one_answer",
        &pairs,
        |&(i, j)| (rel_of(&units[i].0), order_of(i, j).0.to_string()),
        |&(i, j)| {
            let (name, o) = order_of(i, j);
            held(&units[i].0, bases[i].as_ref().expect("a base"), name, o)
        },
    );
    let mut all = Tally::default();
    // A routes' listing the describe does not hold is said once, not once an
    // order.
    let mut seen: BTreeSet<String> = BTreeSet::new();
    for t in results {
        let failures: Vec<String> = t
            .failures
            .into_iter()
            .filter(|f| {
                !f.ends_with("the routes' order: lists a state the describe does not")
                    || seen.insert(f.clone())
            })
            .collect();
        all.failures.extend(failures);
        all.cells += t.cells;
        all.trees += t.trees;
        all.other_work += t.other_work;
        all.other_states += t.other_states;
        all.one_stopped += t.one_stopped;
        all.unheld.extend(t.unheld);
    }
    eprintln!(
        "{} orders, {} cells, {} answered by a tree: {} do other work than under the routes' \
         order, {} list other states in a listing stopped as asked, {} finish under one order \
         and not the other; held to no finished describe or whole maximal line: {:?}",
        orders.len(),
        all.cells,
        all.trees,
        all.other_work,
        all.other_states,
        all.one_stopped,
        all.unheld
    );
    assert!(
        all.failures.is_empty(),
        "{} cell(s) answer otherwise under another order:\n{}",
        all.failures.len(),
        all.failures.join("\n")
    );
    assert!(
        all.unheld.is_empty(),
        "a listing under another order is held to R by no finished describe or whole maximal \
         line: {:?}",
        all.unheld
    );
    // Not vacuous: trees are asked, and the orders are orders, with other
    // work and other states listed.
    assert!(
        all.cells >= ein_corpus::shard::share(4 * 150 * orders.len())
            && all.trees >= ein_corpus::shard::share(60 * orders.len()),
        "{} cells, {} trees",
        all.cells,
        all.trees
    );
    assert!(
        all.other_work >= ein_corpus::shard::share(30 * orders.len())
            && all.other_states >= ein_corpus::shard::share(10),
        "{} cells with other work, {} with other states",
        all.other_work,
        all.other_states
    );
}

/// **The work does not read the order a program states its facts in.**
/// The two SENDs differ in the order of eight facts and in nothing else, and
/// under the routes' order `--single` and the maximal line saturate the same
/// number of nodes, derive the same number of facts, list the same states
/// and read the same five couplings on both: `--single` answers *Solution*
/// on each.
///
/// **What still reads the written order, and why.** `-n N`, whose last key,
/// among instances equal in coupling and in count, is the first the
/// obligation pass reports: it finds the one state on both, by a first tree
/// that follows the declaration until the couplings are read, which is other
/// work on the two files, and that work is not compared. And the minimal
/// line, whole or under `--single`, which reads no coupling and takes the
/// written order: the budget stops it on both, after some seconds, so it is
/// read with the `slow` entries, and there the two are two runs.
#[test]
fn the_two_sends_take_the_same_work() {
    // One computation, whole in one shard (`EIN_TEST_SHARD`).
    if !ein_corpus::shard::whole_in(SENDS_SHARD) {
        return;
    }
    let slow = std::env::var("EIN_CORPUS_SLOW").as_deref() == Ok("1");
    let (send, written) = (repo_root().join(SEND), repo_root().join(SEND_WRITTEN));
    let texts = |x: &Answer| -> Vec<Vec<String>> {
        x.states
            .iter()
            .map(|s| {
                s.closure
                    .iter()
                    .map(|&c| x.candidates[c as usize].text.clone())
                    .collect()
            })
            .collect()
    };
    for (flags, mode) in questions() {
        if flags == "--minimal" {
            if !slow {
                continue;
            }
            // The written order on both, no coupling read, and the budget
            // stops the line on both: what it does is the declaration's.
            for path in [&send, &written] {
                let a = ask(path, mode, None).expect("answers");
                assert!(
                    a.stopped.is_some() && !a.exhausted,
                    "{flags}: {:?}",
                    a.stopped
                );
                let order = a.order.expect("a tree's order");
                assert!(order.written && order.couplings.is_none(), "{order:?}");
            }
            continue;
        }
        let a = ask(&send, mode, None).expect("answers");
        let b = ask(&written, mode, None).expect("answers");
        if flags != "-n 1" {
            assert_eq!(
                (a.saturations, a.facts),
                (b.saturations, b.facts),
                "{flags}: the work, by column and as written"
            );
            assert_eq!(a.order, b.order, "{flags}: the order each run took");
        }
        assert_eq!(
            (a.verdict, a.exhausted, a.stopped.is_some()),
            (b.verdict, b.exhausted, b.stopped.is_some()),
            "{flags}"
        );
        assert_eq!(texts(&a), texts(&b), "{flags}: the states listed");
        assert!(a.stopped.is_none(), "{flags}: {SEND} is answered");
        assert_eq!(a.states.len(), 1, "{flags}: the one state");
        // The couplings were read, and are SEND's columns.
        let order = a.order.expect("a tree's order");
        assert_eq!(order.couplings.map(|c| c.0), Some(5), "{flags}: {order:?}");
        if mode.ask == Ask::Single {
            assert_eq!(a.verdict, Some(Verdict::Solution), "{SEND}");
            assert!(a.exhausted, "{SEND} is decided");
        }
    }
    // The control: under the written order the two are two runs, and the
    // one declared as the puzzle is written is not decided.
    let single = questions()[0].1;
    let a = ask(&send, single, Some(&Order::WRITTEN)).expect("answers");
    assert_eq!(a.verdict, Some(Verdict::Solution));
    if slow {
        let b = ask(&written, single, Some(&Order::WRITTEN)).expect("answers");
        assert!(
            b.stopped.is_some() && b.verdict.is_none(),
            "under the written order {SEND_WRITTEN} was stopped: {:?}",
            b.verdict
        );
    }
}
