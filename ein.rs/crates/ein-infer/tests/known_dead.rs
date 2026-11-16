//! What a route knows dead without a saturation, held three ways:
//! `ein_infer::known_dead`, the one memory the state route and the modes'
//! routes ask before they saturate a set, and L8, *what a state denies is
//! dead above it*.
//!
//! - **The licence against the oracle.** On every monotone cube the oracle
//!   enumerates, the node above an alive node by a candidate that node
//!   denies is dead, and no alive node holds a candidate it denies. On a
//!   flagged program the first can fail, which is why a flagged program gets
//!   nothing: a program written for it is the control.
//! - **With and without.** Every question of `ein solve` on every corpus
//!   program and register witness that searches is asked twice, the denials
//!   read and from deaths alone (`known_dead::from_deaths_alone`, a switch on
//!   the calling thread that no lever reaches): one answer.
//! - **The fixture.** `tests/region/10_denied_extensions.ein`, a bijection
//!   whose rules store what they exclude: how many extensions each question
//!   knew dead by a denial is pinned, so a change that drops the step fails
//!   here and not only in a work count.
//!
//! ## What *one answer* means, written before the cells were read
//!
//! A run **finished** when no budget, limit or deadline stopped it.
//!
//! - **Both finished, describe and `--count`**: the two descriptions are equal
//!   field for field, `N`'s cores among them, but the work.
//! - **Both finished, any other question**: one verdict, one `exhausted`; a
//!   part of `R` both list whole is one set of states and one set of minimal
//!   commitments; a part a question stopped listing as asked (`-n 1`, a
//!   `--single` that found two) is as many states from each. Which states a
//!   stopped listing holds is the route's order, and a tree told more is
//!   dead may branch on another instance; so every state either run lists
//!   is held to `R` as the describe with the denials lists it, wherever that
//!   describe finished; where it did not, to the maximal line, where that
//!   was listed whole both ways (a state on it is in `R`); and to the oracle
//!   by `explore.rs`. A cell that rested on a denial, or lists other states
//!   than the run from deaths alone, has its listing held by the describe or
//!   by the line: there is none in the corpus that neither holds.
//! - **Either stopped**: a describe's order does not depend on what is known
//!   dead, so the run with the denials has met every state, every certified
//!   minimal commitment and every member of `N` the run from deaths alone
//!   met, its count of commitments is no lower, and it is complete at least
//!   as far. Of any question: a verdict both reach is one verdict, and **a
//!   run with the denials is never stopped where the run from deaths alone
//!   finished**.
//! - **The work**: with the denials a describe saturates no more nodes and
//!   derives no more facts, on any program (its order is fixed, and a denial
//!   only replaces a saturation). A tree's work is recorded and not held:
//!   its order may move.
//!
//! The corpus's `slow` entries are read under `EIN_CORPUS_SLOW=1`, the
//! nightly's; the two zebra puzzles every run.
//!
//! ```text
//! cargo test -p ein-infer --profile gate --test known_dead
//! EIN_CORPUS_SLOW=1 cargo test -p ein-infer --profile gate --test known_dead
//! ```

use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

use ein_core::{Kb, Terms};
use ein_corpus::repo_root;
use ein_infer::explore::{self, Answer, Ask, Licence, Mode, Which};
use ein_infer::known_dead;
use ein_infer::region::{self, Cores, Outcome};
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

/// The programs every run reads, `slow` or not: the two puzzles whose rules
/// store what they exclude.
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
    files
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

/// One question of `path`, at the default budget and limit: with the denials
/// read, or from deaths alone. A describe takes `N`'s cores, which is what a
/// surface that prints a dead set's core reads.
fn ask(path: &Path, mode: Mode, deaths_alone: bool) -> Result<Answer, String> {
    let go = || {
        let (mut kb, mut terms, ast) = loaded(path).expect("loaded once already");
        let mut opts = explore::Options::new(mode);
        opts.cores = Cores {
            visited: false,
            nogoods: mode.ask == Ask::Describe && mode.which == Which::All,
        };
        explore::run(&mut kb, &mut terms, &ast, &opts).map_err(|e| e.to_string())
    };
    if deaths_alone {
        known_dead::from_deaths_alone(go)
    } else {
        go()
    }
}

/// What a describing answer says of `R`: every field but the work and what
/// names the work's licences.
fn described(a: &Answer) -> String {
    let mut d = a.described.clone();
    if let Some(d) = d.as_mut() {
        d.work = Default::default();
    }
    format!(
        "{:?} {:?} {:?} {:?} {} {:?} {:?} {:?} {:?}",
        a.states, a.minimal, a.counts, a.shape, a.exhausted, a.verdict, a.stopped, a.route, d
    )
}

/// A listing as a set of states, each its closure.
fn closures(a: &Answer) -> BTreeSet<Vec<u32>> {
    a.states.iter().map(|s| s.closure.to_vec()).collect()
}

#[derive(Default)]
struct Tally {
    failures: Vec<String>,
    /// Cells both runs answered.
    cells: usize,
    /// Cells whose answer rested on a denial.
    spent: usize,
    /// Cells with fewer saturations with the denials, and with more.
    fewer: usize,
    more: Vec<String>,
    /// Cells a budget stopped from deaths alone and not with the denials.
    finished_with: Vec<String>,
    /// Cells a budget stopped both ways.
    stopped: usize,
    /// Cells whose two listings, both stopped as asked, hold other states.
    other_states: Vec<String>,
    /// Cells whose answer rested on a denial, or whose two listings differ,
    /// that list a state neither a finished describe nor a maximal line
    /// listed whole holds to `R`.
    unheld: Vec<String>,
    saturations: [u64; 2],
}

/// One program that searches: its six questions both ways, held to the
/// module's definition of *one answer*.
fn held(path: &Path) -> Tally {
    let mut t = Tally::default();
    let Some((mut kb, mut terms, ast)) = loaded(path) else {
        return t;
    };
    if !searching::searches(&ast, &mut terms, &mut kb) {
        return t;
    }
    let rel = rel_of(path);
    // R as the describe with the denials lists it, where it finished: what
    // every state any cell lists is held to.
    let mut r: Option<BTreeSet<Vec<u32>>> = None;
    // Where it did not: the maximal line, listed whole both ways, and the
    // listings to hold to it.
    let mut line: Option<BTreeSet<Vec<u32>>> = None;
    let mut listed: Vec<(String, BTreeSet<Vec<u32>>)> = Vec::new();
    for (flags, mode) in questions() {
        let at = format!("{rel} {flags}");
        let (a, b) = match (ask(path, mode, false), ask(path, mode, true)) {
            (Ok(a), Ok(b)) => (a, b),
            (Err(x), Err(y)) if x == y => continue,
            (x, y) => {
                t.failures
                    .push(format!("{at}: {:?} against {:?}", x.err(), y.err()));
                continue;
            }
        };
        t.cells += 1;
        t.saturations[0] += a.saturations;
        t.saturations[1] += b.saturations;
        let l8 = a.licences.contains(&Licence::Denied);
        if l8 != (a.denials > 0) {
            t.failures.push(format!(
                "{at}: L8 named {l8}, and {} denials spent",
                a.denials
            ));
        }
        if b.denials > 0 || b.licences.contains(&Licence::Denied) {
            t.failures
                .push(format!("{at}: a run from deaths alone spent a denial"));
        }
        if !a.monotonicity.is_monotone() && a.denials > 0 {
            t.failures
                .push(format!("{at}: a flagged program spent a denial"));
        }
        t.spent += usize::from(a.denials > 0);
        match a.saturations.cmp(&b.saturations) {
            std::cmp::Ordering::Less => t.fewer += 1,
            std::cmp::Ordering::Greater => t.more.push(format!(
                "{at}: {} saturations, {} from deaths alone",
                a.saturations, b.saturations
            )),
            std::cmp::Ordering::Equal => {}
        }
        let describing = mode.which == Which::All && matches!(mode.ask, Ask::Describe | Ask::Count);
        if describing && (a.saturations > b.saturations || a.facts > b.facts) {
            t.failures.push(format!(
                "{at}: a describe does more with the denials: {} saturations and {} facts \
                 against {} and {}",
                a.saturations, a.facts, b.saturations, b.facts
            ));
        }
        if a.verdict.is_some() && b.verdict.is_some() && a.verdict != b.verdict {
            t.failures
                .push(format!("{at}: {:?} against {:?}", a.verdict, b.verdict));
        }
        match (a.stopped.is_some(), b.stopped.is_some()) {
            (false, false) if describing => {
                if described(&a) != described(&b) {
                    t.failures
                        .push(format!("{at}: the two descriptions differ beyond the work"));
                }
                if mode.ask == Ask::Describe && a.exhausted {
                    r = a
                        .described
                        .as_ref()
                        .map(|d| d.states.iter().map(|s| s.closure.to_vec()).collect());
                }
            }
            (false, false) => {
                if (a.verdict, a.exhausted) != (b.verdict, b.exhausted) {
                    t.failures.push(format!(
                        "{at}: {:?}, exhausted {}, against {:?}, exhausted {}",
                        a.verdict, a.exhausted, b.verdict, b.exhausted
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
            }
            (true, false) => t.failures.push(format!(
                "{at}: stopped with the denials ({:?}) and finished from deaths alone",
                a.stopped
            )),
            (stopped_with, true) => {
                if stopped_with {
                    t.stopped += 1;
                } else {
                    t.finished_with.push(at.clone());
                }
                if describing {
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
                    let up_to = |d: &ein_infer::described::Described| {
                        d.states_partial.and_then(|p| p.up_to)
                    };
                    let further = da.exhausted || up_to(da) >= up_to(db);
                    if !states(db).is_subset(&states(da))
                        || !lines(db).is_subset(&lines(da))
                        || (!da.nogoods_skipped && !dead(db).is_subset(&dead(da)))
                        || da.k_c < db.k_c
                        || !further
                    {
                        t.failures.push(format!(
                            "{at}: the run with the denials has not met what the run from \
                             deaths alone met"
                        ));
                    }
                }
            }
        }
        // Every state either run lists is a state of R: by the describe
        // where it finished, else by the maximal line, below.
        match &r {
            Some(r) => {
                for (who, x) in [("with the denials", &a), ("from deaths alone", &b)] {
                    if !closures(x).is_subset(r) {
                        t.failures
                            .push(format!("{at}: {who} lists a state the describe does not"));
                    }
                }
            }
            // A cell that spent no denial and lists what the run from deaths
            // alone lists is that run again: nothing of it is the step's.
            None if !describing && (a.denials > 0 || closures(&a) != closures(&b)) => {
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
    // Where no describe finished: a state on the maximal line is in R, so a
    // listing inside that line, listed whole both ways, is held. One that is
    // not is named, and is the oracle's to hold where it reaches.
    for (at, states) in listed {
        if !states.is_empty() && !line.as_ref().is_some_and(|l| states.is_subset(l)) {
            t.unheld.push(at);
        }
    }
    t
}

/// **Every question answers alike with the denials read and from deaths
/// alone**, on every corpus program and register witness that searches: the
/// module's definition of *one answer*, cell by cell.
#[test]
fn every_question_answers_alike_with_the_denials_and_from_deaths_alone() {
    // The corpus's `slow` entries too, where the run asks for them: the one
    // test of this binary that reads the flag.
    let slow_too = std::env::var("EIN_CORPUS_SLOW").as_deref() == Ok("1");
    let files = files(slow_too);
    let results = ein_corpus::pool::map(
        "known_dead::with_and_without",
        &files,
        |p| (rel_of(p), String::new()),
        |p| held(p),
    );
    let mut all = Tally::default();
    let mut programs = 0usize;
    for t in results {
        programs += usize::from(t.cells > 0);
        all.failures.extend(t.failures);
        all.cells += t.cells;
        all.spent += t.spent;
        all.fewer += t.fewer;
        all.more.extend(t.more);
        all.finished_with.extend(t.finished_with);
        all.stopped += t.stopped;
        all.other_states.extend(t.other_states);
        all.unheld.extend(t.unheld);
        all.saturations[0] += t.saturations[0];
        all.saturations[1] += t.saturations[1];
    }
    eprintln!(
        "{programs} programs that search, {} cells: {} rest on a denial, {} saturate fewer nodes \
         and {} more; {} saturations with the denials, {} from deaths alone; {} cells stopped \
         both ways; finished with the denials alone: {:?}; other states in a stopped listing: \
         {:?}; a listing held to no finished describe or maximal line: {:?}; more: {:?}",
        all.cells,
        all.spent,
        all.fewer,
        all.more.len(),
        all.saturations[0],
        all.saturations[1],
        all.stopped,
        all.finished_with,
        all.other_states,
        all.unheld,
        all.more
    );
    assert!(
        all.failures.is_empty(),
        "{} cell(s) answer otherwise with the denials:\n{}",
        all.failures.len(),
        all.failures.join("\n")
    );
    assert!(
        all.unheld.is_empty(),
        "a listing the denials touched is held to R by no finished describe or maximal line: \
         {:?}",
        all.unheld
    );
    // Not vacuous: the programs that search are read, some rest on a denial,
    // and where they do the work falls.
    assert!(
        programs >= 150 && all.cells >= 6 * 150,
        "{programs} programs, {} cells",
        all.cells
    );
    assert!(
        all.spent >= 20 && all.fewer >= 20 && all.saturations[0] < all.saturations[1],
        "{} cells rest on a denial, {} saturate fewer nodes",
        all.spent,
        all.fewer
    );
}

/// **L8 against the oracle.** On every monotone cube the oracle enumerates:
/// no alive node holds a candidate it denies, and the node above an alive
/// node by a candidate it denies is dead, which is all L8 claims (L1 closes
/// it upward, and `region.rs` holds L1 on the same cubes). A flagged cube of
/// the corpus that breaks the second is named in the test's line; whether or
/// not one does, a flagged program is told nothing
/// (`a_flagged_program_spends_no_denial`).
#[test]
fn what_an_alive_node_denies_is_dead_above_it_on_every_monotone_cube() {
    let mut files = ein_corpus::corpus_files();
    for row in ein_corpus::register().case {
        for (_, a) in row.answers() {
            files.push(repo_root().join(row.witness_of(a)));
        }
    }
    files.sort();
    files.dedup();
    // (failures, monotone cubes, denials checked, flagged cubes that break it)
    type Read = (Vec<String>, usize, u64, Option<String>);
    let results: Vec<Read> = ein_corpus::pool::map(
        "known_dead::against_the_oracle",
        &files,
        |p| (rel_of(p), String::new()),
        |path| {
            let mut failures = Vec::new();
            let Some((mut kb, mut terms, ast)) = loaded(path) else {
                return (failures, 0, 0, None);
            };
            let oracle = region::Options {
                cores: Cores::READ_OUT,
                ..region::Options::default()
            };
            let out = ein_infer::forbid::studying(|| {
                region::compute(&mut kb, &mut terms, &ast, &oracle, &mut |_| {})
            });
            let Ok(Outcome::Region(r)) = out else {
                return (failures, 0, 0, None);
            };
            let rel = rel_of(path);
            let monotone = r.monotonicity.is_monotone();
            let alive = |m: u64| r.status[m as usize] & region::DEAD == 0;
            let mut checked = 0u64;
            let mut broken: Option<String> = None;
            for m in (0..1u64 << r.n()).filter(|&m| alive(m)) {
                let node = &r.nodes[m as usize];
                if node.closure & node.excluded != 0 {
                    failures.push(format!(
                        "{rel}: the alive node {} holds a candidate it denies",
                        r.name(m)
                    ));
                }
                for b in (0..r.n()).filter(|b| node.excluded >> b & 1 == 1) {
                    checked += 1;
                    if !alive(m | 1 << b) {
                        continue;
                    }
                    let what = format!(
                        "{rel}: {} denies {} and {} is alive",
                        r.name(m),
                        r.candidates[b].text,
                        r.name(m | 1 << b)
                    );
                    if monotone {
                        failures.push(what);
                    } else {
                        broken.get_or_insert(what);
                    }
                }
            }
            (
                failures,
                usize::from(monotone),
                if monotone { checked } else { 0 },
                broken,
            )
        },
    );
    let mut failures = Vec::new();
    let (mut cubes, mut checked) = (0usize, 0u64);
    let mut flagged: Vec<String> = Vec::new();
    for (f, c, k, b) in results {
        failures.extend(f);
        cubes += c;
        checked += k;
        flagged.extend(b);
    }
    eprintln!(
        "{cubes} monotone cubes, {checked} denials of an alive node checked; flagged cubes \
         where a denial does not kill: {flagged:?}"
    );
    assert!(failures.is_empty(), "{}", failures.join("\n"));
    assert!(
        cubes >= 150 && checked >= 1000,
        "{cubes} monotone cubes, {checked} denials checked"
    );
}

/// The fixture's path.
const FIXTURE: &str = "tests/region/10_denied_extensions.ein";

/// **The fixture**: a bijection of three over three whose rules store what a
/// placement excludes. Each question's answer is the same both ways, and the
/// extensions it knew dead by a denial, with the saturations they left, are
/// pinned: the step dropped, or spent where it was not, fails here.
#[test]
fn the_fixture_pins_the_extensions_known_dead_by_a_denial() {
    let path = repo_root().join(FIXTURE);
    // (question, denials spent, saturations, saturations from deaths alone)
    // Describe takes the tree and the classes here, root owing an instance a
    // licence takes: its 18 are the 18 pairs of seatings that exclude each
    // other, as the state route's are (34 with them, and 52 from deaths
    // alone by the state route, which `--describe-route states` asks). A tree
    // meets states out of size order, so it also takes larger sets for dead
    // on a denial, and from deaths alone it saturates more than the state
    // route does.
    let expected: [(&str, u64, u64, u64); 6] = [
        ("describe", 18, 34, 67),
        ("--count", 18, 34, 67),
        ("--single", 6, 12, 19),
        ("-n 1", 5, 11, 16),
        ("--minimal", 60, 34, 70),
        ("--maximal", 18, 16, 49),
    ];
    let mut got: Vec<(&str, u64, u64, u64)> = Vec::new();
    for (flags, mode) in questions() {
        let a = ask(&path, mode, false).unwrap_or_else(|e| panic!("{flags}: {e}"));
        let b = ask(&path, mode, true).unwrap_or_else(|e| panic!("{flags}: {e}"));
        assert_eq!(b.denials, 0, "{flags}: from deaths alone");
        assert_eq!(
            a.licences.contains(&Licence::Denied),
            a.denials > 0,
            "{flags}: L8 is named exactly where a denial was spent"
        );
        assert!(a.stopped.is_none() && b.stopped.is_none(), "{flags}");
        assert_eq!(
            (a.verdict, a.exhausted, closures(&a)),
            (b.verdict, b.exhausted, closures(&b)),
            "{flags}"
        );
        got.push((flags, a.denials, a.saturations, b.saturations));
    }
    assert_eq!(got, expected);
}

/// **A flagged program spends no denial**, and this one shows why it may
/// not. `(p A)` denies `(p B)` only while `(p B)` is absent, so the node of
/// `{(p A)}` is alive and denies `(p B)`, and the node of both is alive too:
/// L8 read there would call a member of `R` dead. The static check flags the
/// program for that `absent`, its route is the cube, and the cube asks the
/// memory nothing.
#[test]
fn a_flagged_program_spends_no_denial() {
    let text = "(relation p T :why \"p holds of {?1}\")
         (relation item T :why \"{?1} is an item\")
         (item A) (item B)
         (rule deny ()
           :match (and (p A) (absent (p B)))
           :assert (not (p B))
           :why \"p holds of A and not of B, so p of B is denied\")
         (hrule guess () :match (item ?x) :assert (p ?x) :why \"guess p of {?x}\")
         (solve :hrules (guess))";
    let run = |mode: Mode| {
        let mut ast = Ast::new();
        let mut terms = Terms::new();
        let forms = parse(&mut ast, text, Some("<fixture>")).expect("parses");
        let mut kb = load(&mut ast, &mut terms, &forms, None).unwrap_or_else(|e| panic!("{e}"));
        ein_infer::forbid::studying(|| {
            explore::run(&mut kb, &mut terms, &ast, &explore::Options::new(mode))
        })
        .unwrap_or_else(|e| panic!("{e}"))
    };
    let all = |ask| Mode {
        which: Which::All,
        ask,
    };
    let a = run(all(Ask::Describe));
    assert!(!a.monotonicity.is_monotone(), "the static check flags it");
    assert_eq!(a.route, explore::Route::Cube);
    assert_eq!((a.denials, a.saturations), (0, 4), "the whole cube");
    assert!(!a.licences.contains(&Licence::Denied));
    // R is all four commitments: the one above the denial among them.
    let names: BTreeSet<String> = a.states.iter().map(|s| a.name(&s.closure)).collect();
    assert_eq!(
        names,
        ["∅", "{(p A)}", "{(p B)}", "{(p A), (p B)}"]
            .into_iter()
            .map(str::to_string)
            .collect()
    );
    for (flags, mode) in questions() {
        let a = run(mode);
        assert_eq!(a.denials, 0, "{flags}");
        assert!(!a.licences.contains(&Licence::Denied), "{flags}");
    }
}
