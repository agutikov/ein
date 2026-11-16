//! **A dead node stopped at its first contradiction answers what it answered
//! run to quiescence.** On a monotone program a route's dead node is
//! saturated only as far as the firing that makes its first contradiction
//! (`ein_infer::region`'s § What a node is); the oracle's node, root's, a
//! flagged program's, route 1's, a reader of the structure's and the whole
//! node the tree's order reads run on, and a core is taken over the fixpoint.
//! `ein_infer::region::to_quiescence` is the switch that runs every route the
//! calling thread opens without the stop, and no lever reaches it.
//!
//! Every question of `ein solve` on every corpus program and register
//! witness that searches is asked twice, with the stop and without, at the
//! default budget and limit, and the two answers are held to the definition
//! below. `explore.rs` holds every mode with the stop to the oracle, and
//! `known_dead.rs`, `order.rs`, `claim_routes.rs` and `region.rs` hold what
//! they held, with the stop.
//!
//! ## What *the same answer* means, written before the cells were read
//!
//! A run **finished** when no budget, limit or deadline stopped it.
//!
//! - **A program the stop does not reach**: a flagged program, whose routes
//!   take the cube and whose detector reads every node's fixpoint, and a
//!   program whose root is dead, decided by root's node. The two answers are
//!   equal field for field, **the work included**.
//! - **Both finished, describe and `--count`**: the two descriptions are equal
//!   field for field but two things a dead node reports of itself: the work,
//!   which falls by what the dead nodes derived after their stop, with the
//!   same saturations; and whether some node met owes (`owing_met`, read on
//!   dead nodes too and printed by no surface). `R`, `N` with every member's
//!   core (taken over the node's fixpoint either way), `dead alone`, the
//!   lines, the shape and every count are among the fields held.
//! - **Both finished, any other question**: one route, one verdict, one
//!   `exhausted`; a part of `R` both list whole is one set of states and one
//!   set of minimal commitments; a part a question stopped listing as asked
//!   (`-n 1`, a `--single` that found two) is as many states from each. The
//!   tree's coupled order reads the whole node within a share of the facts
//!   the run has derived, so with the stop it can read it at another
//!   saturation and branch otherwise: then the states a stopped listing holds
//!   may be others and the work another, and every state either run lists is
//!   held to `R`, as the describe with the stop lists it where it finished,
//!   else to the maximal line, listed whole both ways. Where neither run read
//!   the couplings, the same saturations and no more facts with the stop.
//! - **Either stopped**: the stop only lowers what a node costs, so a
//!   describe with it has met every state, every certified minimal commitment
//!   and every member of `N` the run without it met, its count of
//!   commitments is no lower, and it is complete at least as far; and a run
//!   with the stop is never stopped where the run without it finished. A
//!   verdict both reach is one verdict.
//!
//! The corpus's `slow` entries are read under `EIN_CORPUS_SLOW=1`, the
//! nightly's, in four shards (`EIN_TEST_SHARD=k/4`), each holding its share
//! of the floors; the two zebra puzzles every run.
//!
//! ```text
//! cargo test -p ein-infer --profile gate --test first_contradiction
//! EIN_CORPUS_SLOW=1 cargo test -p ein-infer --profile gate --test first_contradiction
//! EIN_CORPUS_SLOW=1 EIN_TEST_SHARD=2/4 cargo test -p ein-infer --profile gate --test first_contradiction
//! ```

use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

use ein_core::{Kb, Terms};
use ein_corpus::repo_root;
use ein_infer::explore::{self, Answer, Ask, Mode, Which};
use ein_infer::region::{self, Cores};
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

/// The programs every run reads, `slow` or not: the two puzzles whose dead
/// nodes derive most after their first contradiction.
const INSTANCES: [&str; 2] = ["examples/zebra.ein", "examples/zebra2.ein"];

/// Every corpus file and register witness, once; the `slow` entries that are
/// not [`INSTANCES`] where `slow_too` only.
fn files(slow_too: bool) -> Vec<PathBuf> {
    let slow: BTreeSet<PathBuf> = ein_corpus::corpus()
        .entry
        .iter()
        .filter(|e| e.slow && !INSTANCES.contains(&e.path.as_str()))
        .map(|e| repo_root().join(&e.path))
        .collect();
    let mut files = ein_corpus::corpus_files();
    for row in ein_corpus::register().case {
        for (_, a) in row.answers() {
            files.push(repo_root().join(row.witness_of(a)));
        }
    }
    files.sort();
    files.dedup();
    files.retain(|f| slow_too || !slow.contains(f));
    // The nightly's shard of them (`EIN_TEST_SHARD`).
    ein_corpus::shard::keep(files, |f| rel_of(f))
}

/// The six questions: describe, `--count`, `--single`, `-n 1`, `--minimal`,
/// `--maximal`.
fn questions() -> [(&'static str, Mode); 6] {
    let all = |ask| Mode {
        which: Which::All,
        ask,
    };
    [
        ("describe", all(Ask::Describe)),
        ("--count", all(Ask::Count)),
        ("--single", all(Ask::Single)),
        ("-n 1", all(Ask::Find(1))),
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

/// One question of `path`, at the default budget and limit, with the stop or
/// to quiescence. A describe takes `N`'s cores, which is what a surface that
/// prints a dead set's core reads.
fn ask(path: &Path, mode: Mode, stop: bool) -> Result<Answer, String> {
    let go = || {
        let (mut kb, mut terms, ast) = loaded(path).expect("loaded once already");
        let mut opts = explore::Options::new(mode);
        opts.cores = Cores {
            visited: false,
            nogoods: mode.ask == Ask::Describe && mode.which == Which::All,
        };
        explore::run(&mut kb, &mut terms, &ast, &opts).map_err(|e| e.to_string())
    };
    if stop {
        go()
    } else {
        region::to_quiescence(go)
    }
}

/// What an answer says of `R`, the work and the order's record left out, and
/// with `reported` false also whether some node met owes, which a dead node
/// reports of itself.
fn said(a: &Answer, reported: bool) -> String {
    let mut d = a.described.clone();
    if let Some(d) = d.as_mut() {
        d.work = Default::default();
        if !reported {
            d.owing_met = false;
        }
    }
    format!(
        "{:?} {:?} {:?} {:?} {:?} {} {:?} {:?} {:?} {:?} {:?} {:?}",
        a.states,
        a.minimal,
        a.counts,
        a.shape,
        a.partial,
        a.exhausted,
        a.verdict,
        a.stopped,
        a.route,
        a.licences,
        a.denials,
        d
    )
}

/// A listing as a set of states, each its closure.
fn closures(a: &Answer) -> BTreeSet<Vec<u32>> {
    a.states.iter().map(|s| s.closure.to_vec()).collect()
}

/// Whether the run read the couplings off the whole node, or tried to.
fn read_couplings(a: &Answer) -> bool {
    a.order
        .is_some_and(|o| o.couplings.is_some() || o.unaffordable > 0)
}

#[derive(Default)]
struct Tally {
    failures: Vec<String>,
    /// Cells both runs answered.
    cells: usize,
    /// Cells of a monotone program, its root alive.
    reached: usize,
    /// Of those, cells with fewer facts with the stop.
    fewer: usize,
    /// Facts with the stop and without, over the cells both finished.
    facts: [u64; 2],
    /// Cells a budget stopped without the stop and not with it.
    finished_with: Vec<String>,
    /// Cells a budget stopped both ways.
    stopped: usize,
    /// What each of those reached with the stop and without.
    reach: Vec<String>,
    /// Per question, over the cells of a monotone program with root alive,
    /// stopped ones with what they spent: saturations and facts with the
    /// stop, then without.
    work: std::collections::BTreeMap<&'static str, [u64; 4]>,
    /// Cells whose two listings, both stopped as asked, hold other states.
    other_states: Vec<String>,
    /// Cells whose work moved where the coupled order was read.
    order_moved: Vec<String>,
    /// Describes whose `owing_met` moved.
    owing_moved: Vec<String>,
    /// Members of `N` held with their cores.
    cores: usize,
    /// Listings held to no finished describe or maximal line.
    unheld: Vec<String>,
}

/// Whether the sweep reads `path`: it loads, and it searches.
fn searches(path: &Path) -> bool {
    let Some((mut kb, mut terms, ast)) = loaded(path) else {
        return false;
    };
    searching::searches(&ast, &mut terms, &mut kb)
}

/// One program that searches: its six questions both ways, held to the
/// module's definition of *the same answer*. `answers` are the twelve runs,
/// each question's with the stop and then without, in [`questions`]' order,
/// each a unit of the sweep.
fn held(path: &Path, answers: Vec<Result<Answer, String>>) -> Tally {
    let mut t = Tally::default();
    let rel = rel_of(path);
    let mut r: Option<BTreeSet<Vec<u32>>> = None;
    let mut line: Option<BTreeSet<Vec<u32>>> = None;
    let mut listed: Vec<(String, BTreeSet<Vec<u32>>)> = Vec::new();
    let mut answers = answers.into_iter();
    for (flags, mode) in questions() {
        let at = format!("{rel} {flags}");
        let with = answers.next().expect("the run with the stop");
        let without = answers.next().expect("the run without it");
        let (a, b) = match (with, without) {
            (Ok(a), Ok(b)) => (a, b),
            (Err(x), Err(y)) if x == y => continue,
            (x, y) => {
                t.failures
                    .push(format!("{at}: {:?} against {:?}", x.err(), y.err()));
                continue;
            }
        };
        t.cells += 1;
        let describing = mode.which == Which::All && matches!(mode.ask, Ask::Describe | Ask::Count);
        // Where the stop does not reach, nothing moves, the work included.
        if !a.monotonicity.is_monotone() || a.root_dead {
            if said(&a, true) != said(&b, true)
                || (a.saturations, a.facts) != (b.saturations, b.facts)
                || a.order != b.order
            {
                t.failures.push(format!(
                    "{at}: the stop moved a program it does not reach ({} saturations and {} \
                     facts against {} and {})",
                    a.saturations, a.facts, b.saturations, b.facts
                ));
            }
            continue;
        }
        t.reached += 1;
        let w = t.work.entry(flags).or_default();
        w[0] += a.saturations;
        w[1] += a.facts;
        w[2] += b.saturations;
        w[3] += b.facts;
        t.fewer += usize::from(a.facts < b.facts);
        if a.verdict.is_some() && b.verdict.is_some() && a.verdict != b.verdict {
            t.failures
                .push(format!("{at}: {:?} against {:?}", a.verdict, b.verdict));
        }
        let coupled = read_couplings(&a) || read_couplings(&b);
        match (a.stopped.is_some(), b.stopped.is_some()) {
            (false, false) => {
                t.facts[0] += a.facts;
                t.facts[1] += b.facts;
                if coupled {
                    if (a.saturations, a.facts) != (b.saturations, b.facts) {
                        t.order_moved.push(format!(
                            "{at}: {} saturations and {} facts, {} and {} to quiescence",
                            a.saturations, a.facts, b.saturations, b.facts
                        ));
                    }
                } else if a.saturations != b.saturations || a.facts > b.facts {
                    t.failures.push(format!(
                        "{at}: {} saturations and {} facts with the stop, {} and {} without",
                        a.saturations, a.facts, b.saturations, b.facts
                    ));
                }
            }
            (true, false) => t.failures.push(format!(
                "{at}: stopped with the stop ({:?}) and finished without it",
                a.stopped
            )),
            (false, true) => t.finished_with.push(at.clone()),
            (true, true) => {
                t.stopped += 1;
                // How far each reached: the states it listed, and a
                // describe's states met and how far it is complete.
                let reach = |x: &Answer| match x.described.as_ref() {
                    Some(d) => format!(
                        "{} states met, complete up to {:?}, {} saturations",
                        d.states.len(),
                        d.states_partial.and_then(|p| p.up_to),
                        x.saturations
                    ),
                    None => format!("{} states, {} saturations", x.states.len(), x.saturations),
                };
                t.reach
                    .push(format!("{at}: {}; without: {}", reach(&a), reach(&b)));
            }
        }
        match (a.stopped.is_some(), b.stopped.is_some()) {
            (false, false) if describing => {
                if said(&a, false) != said(&b, false) {
                    t.failures
                        .push(format!("{at}: the two descriptions differ beyond the work"));
                }
                let (da, db) = (
                    a.described.as_ref().expect("described"),
                    b.described.as_ref().expect("described"),
                );
                if da.owing_met != db.owing_met {
                    t.owing_moved.push(at.clone());
                }
                if mode.ask == Ask::Describe {
                    // Each member of N with its core, held above; counted so
                    // that a sweep that took none is seen.
                    t.cores += da.dead.iter().filter(|x| x.core_status.is_some()).count();
                    if a.exhausted {
                        r = Some(da.states.iter().map(|s| s.closure.to_vec()).collect());
                    }
                }
            }
            (false, false) => {
                if (a.route, a.verdict, a.exhausted) != (b.route, b.verdict, b.exhausted) {
                    t.failures.push(format!(
                        "{at}: {:?}, {:?}, exhausted {}, against {:?}, {:?}, exhausted {}",
                        a.route, a.verdict, a.exhausted, b.route, b.verdict, b.exhausted
                    ));
                } else if a.exhausted {
                    let minimal = |x: &Answer| -> BTreeSet<Vec<u32>> {
                        x.minimal.iter().map(|(g, _)| g.to_vec()).collect()
                    };
                    if closures(&a) != closures(&b) || minimal(&a) != minimal(&b) {
                        t.failures
                            .push(format!("{at}: one part of R, listed whole, two ways"));
                    }
                } else if a.states.len() != b.states.len() {
                    t.failures.push(format!(
                        "{at}: {} states listed against {}",
                        a.states.len(),
                        b.states.len()
                    ));
                } else if closures(&a) != closures(&b) {
                    t.other_states.push(at.clone());
                }
                if !coupled && said(&a, true) != said(&b, true) {
                    t.failures.push(format!(
                        "{at}: the answers differ beyond the work, the order unread"
                    ));
                }
            }
            (_, true) if describing => {
                let (da, db) = (
                    a.described.as_ref().expect("described"),
                    b.described.as_ref().expect("described"),
                );
                let states = |d: &ein_infer::described::Described| -> BTreeSet<Vec<u32>> {
                    d.states.iter().map(|s| s.closure.to_vec()).collect()
                };
                let lines = |d: &ein_infer::described::Described| -> BTreeSet<Vec<u32>> {
                    d.minimal_commitments.iter().map(|g| g.to_vec()).collect()
                };
                let dead = |d: &ein_infer::described::Described| -> BTreeSet<Vec<u32>> {
                    d.dead.iter().map(|x| x.set.to_vec()).collect()
                };
                let up_to =
                    |d: &ein_infer::described::Described| d.states_partial.and_then(|p| p.up_to);
                let further = da.exhausted || up_to(da) >= up_to(db);
                if !states(db).is_subset(&states(da))
                    || !lines(db).is_subset(&lines(da))
                    || (!da.nogoods_skipped && !dead(db).is_subset(&dead(da)))
                    || da.k_c < db.k_c
                    || !further
                {
                    t.failures.push(format!(
                        "{at}: the run with the stop has not met what the run without it met"
                    ));
                }
            }
            _ => {}
        }
        // Every state either run lists is a state of R: by the describe where
        // it finished, else by the maximal line, below.
        match &r {
            Some(r) => {
                for (who, x) in [("with the stop", &a), ("without it", &b)] {
                    if !closures(x).is_subset(r) {
                        t.failures
                            .push(format!("{at}: {who} lists a state the describe does not"));
                    }
                }
            }
            None if !describing && closures(&a) != closures(&b) => {
                let both: BTreeSet<Vec<u32>> = closures(&a).union(&closures(&b)).cloned().collect();
                listed.push((at.clone(), both));
            }
            None => {}
        }
        if mode.which == Which::Maximal
            && a.exhausted
            && b.exhausted
            && a.stopped.is_none()
            && b.stopped.is_none()
        {
            line = Some(closures(&a));
        }
    }
    for (at, states) in listed {
        if !states.is_empty() && !line.as_ref().is_some_and(|l| states.is_subset(l)) {
            t.unheld.push(at);
        }
    }
    t
}

/// **Every question answers alike with a route's dead nodes stopped at their
/// first contradiction and run to quiescence**, on every corpus program and
/// register witness that searches: the module's definition of *the same
/// answer*, cell by cell.
#[test]
fn every_question_answers_alike_with_the_stop_and_without() {
    // The corpus's `slow` entries too, where the run asks for them: the one
    // test of this binary that reads the flag.
    let slow_too = std::env::var("EIN_CORPUS_SLOW").as_deref() == Ok("1");
    let files = files(slow_too);
    // The programs that search, then one unit per (program, question, run),
    // so that a program's twelve runs go side by side; folded per program,
    // in the programs' order.
    let read = ein_corpus::pool::map(
        "first_contradiction::with_and_without",
        &files,
        |p| (rel_of(p), "searches".to_string()),
        |p| searches(p),
    );
    let files: Vec<&PathBuf> = files
        .iter()
        .zip(read)
        .filter_map(|(p, read)| read.then_some(p))
        .collect();
    let asked = questions();
    let units: Vec<(usize, usize, bool)> = (0..files.len())
        .flat_map(|f| (0..asked.len()).flat_map(move |q| [(f, q, true), (f, q, false)]))
        .collect();
    let mut answers = ein_corpus::pool::map(
        "first_contradiction::with_and_without",
        &units,
        |&(f, q, stop)| {
            (
                rel_of(files[f]),
                format!(
                    "{} {}",
                    asked[q].0,
                    if stop {
                        "with the stop"
                    } else {
                        "to quiescence"
                    }
                ),
            )
        },
        |&(f, q, stop)| ask(files[f], asked[q].1, stop),
    )
    .into_iter();
    let results: Vec<Tally> = files
        .iter()
        .map(|p| held(p, answers.by_ref().take(2 * asked.len()).collect()))
        .collect();
    let mut all = Tally::default();
    let mut programs = 0usize;
    for t in results {
        programs += usize::from(t.cells > 0);
        all.failures.extend(t.failures);
        all.cells += t.cells;
        all.reached += t.reached;
        all.fewer += t.fewer;
        all.facts[0] += t.facts[0];
        all.facts[1] += t.facts[1];
        all.finished_with.extend(t.finished_with);
        all.stopped += t.stopped;
        all.reach.extend(t.reach);
        for (q, w) in t.work {
            for (x, y) in all.work.entry(q).or_default().iter_mut().zip(w) {
                *x += y;
            }
        }
        all.other_states.extend(t.other_states);
        all.order_moved.extend(t.order_moved);
        all.owing_moved.extend(t.owing_moved);
        all.cores += t.cores;
        all.unheld.extend(t.unheld);
    }
    eprintln!(
        "{programs} programs that search, {} cells, {} of them monotone with root alive, {} \
         with fewer facts; {} facts with the stop and {} without over the cells both finished; \
         {} cells stopped both ways; finished with the stop alone: {:?}; the work moved where \
         the coupled order was read: {:?}; other states in a stopped listing: {:?}; `owing_met` \
         moved: {:?}; {} members of N held with their cores",
        all.cells,
        all.reached,
        all.fewer,
        all.facts[0],
        all.facts[1],
        all.stopped,
        all.finished_with,
        all.order_moved,
        all.other_states,
        all.owing_moved,
        all.cores
    );
    for (q, w) in &all.work {
        eprintln!(
            "{q}: {} saturations and {} facts with the stop, {} and {} without",
            w[0], w[1], w[2], w[3]
        );
    }
    for r in &all.reach {
        eprintln!("stopped both ways: {r}");
    }
    assert!(
        all.failures.is_empty(),
        "{} cell(s) answer otherwise with the stop:\n{}",
        all.failures.len(),
        all.failures.join("\n")
    );
    assert!(
        all.unheld.is_empty(),
        "a listing that moved is held to R by no finished describe or maximal line: {:?}",
        all.unheld
    );
    // Not vacuous: the programs that search are read, the stop reaches most
    // of their cells, and where it does the work falls. A shard of the
    // nightly's (`EIN_TEST_SHARD`) holds its share of each floor.
    let share = ein_corpus::shard::share;
    assert!(
        programs >= share(150) && all.cells >= share(6 * 150) && all.reached >= share(6 * 120),
        "{programs} programs, {} cells, {} reached",
        all.cells,
        all.reached
    );
    assert!(
        all.fewer >= share(50) && all.facts[0] < all.facts[1] && all.cores >= share(1000),
        "{} cells with fewer facts; {} facts against {}; {} cores held",
        all.fewer,
        all.facts[0],
        all.facts[1],
        all.cores
    );
}
