//! **Describe's two routes describe `R` alike** — the tree and the classes
//! ([`ein_infer::tree_classes`], the default) and the state route
//! ([`ein_infer::states`]), each forced through `explore::describe_by`.
//!
//! - [`every_field_both_routes_compute_is_one_on_the_corpus`]: on every corpus
//!   program and regression witness the static check calls monotone with an
//!   alive root, every field of `R` or around it that both routes computed is
//!   the same (`Described::differences`), with `N` taken and without; where
//!   the state route stops and the tree and the classes do not, every state
//!   the state route met is one of theirs. The `slow` entries are read under
//!   `EIN_CORPUS_SLOW=1`.
//! - [`both_routes_are_the_oracle_where_it_reaches`]: each against the cube on
//!   every such program within `--region-limit`.
//! - [`the_default_takes_the_tree_where_it_branches`]: describe takes the tree
//!   and the classes exactly where root owes an instance a licence takes, and
//!   the option forces the state route.
//! - [`a_stopped_run_says_what_it_established_and_no_more`]: the tree and the
//!   classes stopped at smaller budgets on programs where they finish at the
//!   default, every claim of the partial description held to the whole one.
//!
//! ```text
//! cargo test -p ein-infer --profile gate --test tree_classes
//! EIN_CORPUS_SLOW=1 cargo test -p ein-infer --profile gate --test tree_classes -- every_field_both_routes_compute_is_one_on_the_corpus
//! ```

use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

use ein_core::{Kb, Terms};
use ein_corpus::repo_root;
use ein_infer::described::Described;
use ein_infer::explore::{self, Ask, DescribeRoute, Mode, Route, Which};
use ein_infer::region;
use ein_infer::tree_classes::Part;
use ein_ir::{Ast, load, parse};

/// Programs on which root owes an instance a licence takes, and which both
/// routes finish at the default budget: one state of `R` or several, merged
/// states, a dead candidate.
const TREE: [&str; 6] = [
    "examples/lattice/07_two_pairs_two_tables.ein",
    "examples/lattice/06_friends_tables_everyone_sits.ein",
    "examples/lattice/14_seats_nothing_proposes.ein",
    "examples/beyond_arith/cryptarithm.ein",
    "examples/binary_logic/knights.ein",
    "tests/stdlib/bool/inputs/01_in_drives.ein",
];

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

fn opts(nogoods: bool, budget: u64) -> explore::Options {
    explore::Options {
        cores: region::Cores::READ_OUT,
        nogoods,
        budget,
        ..explore::Options::new(Mode {
            which: Which::All,
            ask: Ask::Describe,
        })
    }
}

/// `path` described by `route`, forced; `None` where the route does not take
/// the program (flagged, or a dead root for the tree and the classes).
fn by(path: &Path, route: Route, nogoods: bool, budget: u64) -> Option<Described> {
    let (mut kb, mut terms, ast) = loaded(path)?;
    let a = explore::describe_by(&mut kb, &mut terms, &ast, &opts(nogoods, budget), route).ok()?;
    assert_eq!(a.route, route, "{path:?}");
    a.described.map(|d| *d)
}

/// Every corpus file and regression witness, once; the `slow` entries where
/// `slow_too` only.
fn files(slow_too: bool) -> Vec<PathBuf> {
    let slow: BTreeSet<PathBuf> = ein_corpus::corpus()
        .entry
        .iter()
        .filter(|e| e.slow)
        .map(|e| repo_root().join(&e.path))
        .collect();
    let mut files = ein_corpus::corpus_files();
    for row in ein_corpus::regressions().case {
        files.push(repo_root().join(&row.witness));
    }
    files.sort();
    files.dedup();
    files.retain(|f| slow_too || !slow.contains(f));
    files
}

/// The four descriptions [`held`] compares, in its order: `N` taken, then
/// not, each by the state route and then the tree and the classes.
const RUNS: [(bool, Route); 4] = [
    (true, Route::States),
    (true, Route::TreeClasses),
    (false, Route::States),
    (false, Route::TreeClasses),
];

/// What one program's two routes say against each other, from its four
/// descriptions ([`RUNS`], each a unit of the sweep): failures, and whether
/// both finished.
fn held(path: &Path, runs: &[Option<Described>]) -> (Vec<String>, bool) {
    let rel = rel_of(path);
    let mut bad: Vec<String> = Vec::new();
    let mut compared = false;
    for (pair, nogoods) in runs.chunks(2).zip([true, false]) {
        let (Some(s), Some(t)) = (&pair[0], &pair[1]) else {
            return (bad, false);
        };
        match (s.exhausted, t.exhausted) {
            (true, true) => {
                compared = true;
                let d = s.differences(t);
                if !d.is_empty() {
                    bad.push(format!("{rel}, N asked {nogoods}: {d:?} differ"));
                }
            }
            (false, true) => {
                // The state route stopped: what it met of R is the tree's.
                let states: BTreeSet<&[u32]> = t.states.iter().map(|x| &x.closure[..]).collect();
                if let Some(x) = s.states.iter().find(|x| !states.contains(&x.closure[..])) {
                    bad.push(format!(
                        "{rel}: the state route met {:?} in R, which the tree and the classes \
                         did not",
                        x.closure
                    ));
                }
            }
            (true, false) => bad.push(format!(
                "{rel}: the tree and the classes stopped where the state route finished"
            )),
            (false, false) => {
                if t.tree_partial.is_some_and(|p| p.stopped_in > Part::States) {
                    let states: BTreeSet<&[u32]> =
                        t.states.iter().map(|x| &x.closure[..]).collect();
                    if s.states.iter().any(|x| !states.contains(&x.closure[..])) {
                        bad.push(format!(
                            "{rel}: the state route met a state of R the tree's whole R lacks"
                        ));
                    }
                }
            }
        }
    }
    (bad, compared)
}

/// **Every field both routes compute is one, on the corpus** (the module's
/// first item).
#[test]
fn every_field_both_routes_compute_is_one_on_the_corpus() {
    let slow_too = std::env::var("EIN_CORPUS_SLOW").as_deref() == Ok("1");
    let files = files(slow_too);
    // One unit per (program, description), so that a program's four run side
    // by side; folded per program, in the programs' order. Where a route does
    // not take the program, [`held`] reads the first two alone; the other two
    // are asked all the same, so that no pass waits on another, and each ends
    // at the program's root.
    let units: Vec<(usize, usize)> = (0..files.len())
        .flat_map(|f| (0..RUNS.len()).map(move |r| (f, r)))
        .collect();
    let described = ein_corpus::pool::map(
        "tree_classes::both_routes",
        &units,
        |&(f, r)| {
            let (nogoods, route) = RUNS[r];
            (rel_of(&files[f]), format!("{route:?}, N asked {nogoods}"))
        },
        |&(f, r)| {
            let (nogoods, route) = RUNS[r];
            by(
                &files[f],
                route,
                nogoods,
                ein_infer::downset::DEFAULT_BUDGET,
            )
        },
    );
    let results = files
        .iter()
        .zip(described.chunks(RUNS.len()))
        .map(|(p, runs)| held(p, runs));
    let mut bad: Vec<String> = Vec::new();
    let mut compared = 0usize;
    for (b, c) in results {
        bad.extend(b);
        compared += usize::from(c);
    }
    assert!(bad.is_empty(), "{}", bad.join("\n"));
    assert!(
        compared >= 150,
        "both routes finished on {compared} programs: the sweep reads too few"
    );
}

/// **Both routes are the oracle where it reaches**: each against the cube on
/// every monotone program with an alive root within `--region-limit`, the
/// tree and the classes forced where the default would take the state route
/// too.
#[test]
fn both_routes_are_the_oracle_where_it_reaches() {
    let files = files(false);
    let results = ein_corpus::pool::map(
        "tree_classes::oracle",
        &files,
        |p| (rel_of(p), String::new()),
        |p| {
            let rel = rel_of(p);
            let Some((mut kb, mut terms, ast)) = loaded(p) else {
                return (Vec::new(), 0usize);
            };
            let o = region::Options {
                limit: region::DEFAULT_LIMIT,
                keep_states: false,
                jobs: 1,
                deadline: None,
                cores: region::Cores::READ_OUT,
            };
            let Ok(region::Outcome::Region(r)) =
                region::compute(&mut kb, &mut terms, &ast, &o, &mut |_| {})
            else {
                return (Vec::new(), 0);
            };
            let cube = ein_infer::described::of_cube(&r);
            let mut bad = Vec::new();
            let mut read = 0;
            for route in [Route::States, Route::TreeClasses] {
                let Some(d) = by(p, route, true, ein_infer::downset::DEFAULT_BUDGET) else {
                    continue;
                };
                if !d.exhausted {
                    continue;
                }
                read += 1;
                let diff = cube.differences(&d);
                if !diff.is_empty() {
                    bad.push(format!(
                        "{rel} by {route:?}: {diff:?} differ from the oracle's"
                    ));
                }
            }
            (bad, read)
        },
    );
    let mut bad: Vec<String> = Vec::new();
    let mut read = 0usize;
    for (b, r) in results {
        bad.extend(b);
        read += r;
    }
    assert!(bad.is_empty(), "{}", bad.join("\n"));
    assert!(read >= 200, "the oracle held {read} descriptions: too few");
}

/// **The default takes the tree and the classes exactly where root owes an
/// instance a licence takes**, names the other route where there is one, and
/// `--describe-route states` takes the state route.
#[test]
fn the_default_takes_the_tree_where_it_branches() {
    let run = |rel: &str, route: DescribeRoute| {
        let (mut kb, mut terms, ast) = loaded(&repo_root().join(rel)).expect("loads");
        let o = explore::Options {
            describe_route: route,
            ..opts(true, ein_infer::downset::DEFAULT_BUDGET)
        };
        explore::run(&mut kb, &mut terms, &ast, &o).expect("an answer")
    };
    for rel in TREE {
        let a = run(rel, DescribeRoute::Tree);
        assert_eq!(a.route, Route::TreeClasses, "{rel}");
        assert_eq!(a.describe_alt, Some(DescribeRoute::States), "{rel}");
        let a = run(rel, DescribeRoute::States);
        assert_eq!(a.route, Route::States, "{rel}");
        assert_eq!(a.describe_alt, Some(DescribeRoute::Tree), "{rel}");
    }
    // ∅ in R: the tree is root alone, and its walk is the state route's.
    let a = run(
        "examples/lattice/02_genuine_3set_death.ein",
        DescribeRoute::Tree,
    );
    assert_eq!(a.route, Route::States);
    assert_eq!(a.describe_alt, None);
}

/// **A stopped run says what it established and no more**: the tree and the
/// classes at budgets that stop them, on programs they finish at the
/// default, each partial description held to the whole one.
#[test]
fn a_stopped_run_says_what_it_established_and_no_more() {
    let mut stops: BTreeSet<Part> = BTreeSet::new();
    for rel in TREE {
        let path = repo_root().join(rel);
        let whole = by(
            &path,
            Route::TreeClasses,
            false,
            ein_infer::downset::DEFAULT_BUDGET,
        )
        .expect("the tree and the classes take it");
        assert!(whole.exhausted, "{rel} finishes at the default budget");
        let states: Vec<&[u32]> = whole.states.iter().map(|x| &x.closure[..]).collect();
        for budget in (0..12).map(|k| 1u64 << k) {
            let Some(p) = by(&path, Route::TreeClasses, false, budget) else {
                continue;
            };
            if p.exhausted {
                assert!(p.differences(&whole).is_empty(), "{rel} at {budget}");
                continue;
            }
            let part = p.tree_partial.expect("a stopped run says where").stopped_in;
            stops.insert(part);
            let at = format!("{rel} at --region-budget {budget}, stopped in {part:?}");
            // Every state listed is a state of R; past the tree, all of them.
            for s in &p.states {
                assert!(states.contains(&&s.closure[..]), "{at}: a state outside R");
            }
            assert!(p.k_c <= whole.k_c, "{at}: k_c past the whole");
            assert!(p.minimal_commitments.is_empty(), "{at}: a minimal line");
            assert!(p.shape.is_none(), "{at}: a shape");
            assert!(
                p.nogoods_skipped || p.dead.is_empty(),
                "{at}: N as if computed"
            );
            if part > Part::States {
                assert_eq!(p.k_s(), whole.k_s(), "{at}: R's states");
                for (x, y) in p.states.iter().zip(&whole.states) {
                    assert_eq!(x.maximal, y.maximal, "{at}: the maximal line");
                }
                // A class found is the whole run's class; which states are
                // minimal needs every class.
                let found = p.tree_partial.expect("partial").classes as usize;
                for (x, y) in p.states[..found].iter().zip(&whole.states) {
                    assert_eq!(
                        (&x.rep, x.commitments, &x.generators),
                        (&y.rep, y.commitments, &y.generators),
                        "{at}: a class"
                    );
                }
            } else {
                for x in p.states.iter().filter(|x| x.maximal) {
                    let y = whole.states.iter().find(|y| y.closure == x.closure);
                    assert!(y.is_some_and(|y| y.maximal), "{at}: a maximal state");
                }
            }
            if part > Part::Singles {
                assert_eq!(p.dead_alone_set, whole.dead_alone_set, "{at}: dead alone");
            }
        }
    }
    assert!(
        stops.contains(&Part::States) && stops.contains(&Part::Classes),
        "the budgets stop the run in {stops:?} only"
    );
}
