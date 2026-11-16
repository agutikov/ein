//! **The agenda answers what the written order answered, on every monotone
//! program.** The engine's order fires first the rules whose conclusion
//! either side of a contradiction can be made of, then the rest, in the
//! written and arrival order within each half (`ein_infer::saturator`'s
//! § The agenda, `saturator::AGENDA`), in root's saturation, every route's
//! node, the oracle's and every reader's. `ein_infer::saturator::fifo` is the
//! switch that runs every saturation the calling thread builds in the written
//! order alone, and no lever reaches it.
//!
//! Every question of `ein solve` on every corpus program and register witness
//! the static check calls monotone is asked twice, under the agenda and under
//! the written order, at the default budget and limit, and every claim the
//! program states is read under each: the two are held to the definition
//! below. On a program the check flags nothing is held: what is solved of one
//! is not claimed sound, correct or complete (the user's ruling on
//! Q-M1f.313), and such programs are counted and left.
//!
//! ## What *the same answer* means, written before the cells were read
//!
//! A run **finished** when no budget, limit or deadline stopped it.
//!
//! - **Root**: dead or alive alike, and what it owes alike, as a set of
//!   instances.
//! - **Both finished, describe** (and so `--count`): the two descriptions are equal
//!   field for field but what the order may move: the work (the stop falls
//!   elsewhere in a dead node, so its facts move; on the state route its
//!   saturations do not, its order being the states', and on the tree and the
//!   classes they may, the tree's coupled order reading its share of the
//!   run's facts); every core with its status, its
//!   rules and its sources (an explanation, which follows the order a fact's
//!   record keeps its derivations in); the order of the instances a node owes
//!   (the obligation pass reads the KB in the order its facts arrived), each
//!   compared as a set; and whether some node met owes (`owing_met`, read on
//!   dead nodes too and printed by no surface). `R`, `N` as sets, `dead
//!   alone`, the lines, the shape and every count are among the fields held.
//! - **Both finished, any other question**: one route, one verdict, one
//!   `exhausted`; a part of `R` both list whole is one set of states and one
//!   set of minimal commitments; a part a question stopped listing as asked
//!   is as many states from each, and every state either run lists is held
//!   to `R`, as the describe under the agenda lists it where it finished,
//!   else to the maximal line, listed whole both ways. The tree's coupled
//!   order reads its share of the run's facts, so its work may move.
//! - **Either stopped**: a verdict both reach is one verdict, and every state
//!   either lists is held as above. How far a stopped run reached follows
//!   what its dead nodes cost, which the order moves either way.
//! - **Every claim**: each `(query … :expect …)` and the `(solve … :expect …)`
//!   read off the question `ein test` asks first and off the describing one,
//!   one outcome under each order.
//!
//! The corpus's `slow` entries are read under `EIN_CORPUS_SLOW=1`, the
//! nightly's, in four shards (`EIN_TEST_SHARD=k/4`), each holding its share
//! of the floors; the two zebra puzzles every run.
//!
//! ```text
//! cargo test -p ein-infer --profile gate --test agenda
//! EIN_CORPUS_SLOW=1 cargo test -p ein-infer --profile gate --test agenda
//! EIN_CORPUS_SLOW=1 EIN_TEST_SHARD=2/4 cargo test -p ein-infer --profile gate --test agenda
//! ```

use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

use ein_core::{Kb, Terms};
use ein_corpus::repo_root;
use ein_infer::explore::{self, Answer, Ask, Mode, Route, Which};
use ein_infer::region::Cores;
use ein_infer::saturator;
use ein_ir::{Ast, load, parse};

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

/// Every corpus file and register witness, once; the `slow` entries that are
/// not [`INSTANCES`] where `slow_too` only; this run's shard of them.
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
    ein_corpus::shard::keep(files, |f| rel_of(f))
}

/// The questions: describe, `--single`, `-n 1`, `--minimal`, `--maximal`.
/// `--count` is not asked again: `explore::run` answers it by describe's
/// route with describe's mode, so its answer is describe's by construction.
fn questions() -> [(&'static str, Mode); 5] {
    let all = |ask| Mode {
        which: Which::All,
        ask,
    };
    [
        ("describe", all(Ask::Describe)),
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

/// Run `f` under the agenda, or under the written order alone.
fn under<T>(agenda: bool, f: impl FnOnce() -> T) -> T {
    if agenda { f() } else { saturator::fifo(f) }
}

/// One question of `path`, at the default budget and limit. A describe takes
/// `N`'s cores, which is what a surface that prints a dead set's core reads.
fn ask(path: &Path, mode: Mode, agenda: bool) -> Result<Answer, String> {
    under(agenda, || {
        let (mut kb, mut terms, ast) = loaded(path).expect("loaded once already");
        let mut opts = explore::Options::new(mode);
        opts.cores = Cores {
            visited: false,
            nogoods: mode.ask == Ask::Describe && mode.which == Which::All,
        };
        explore::run(&mut kb, &mut terms, &ast, &opts).map_err(|e| e.to_string())
    })
}

/// Every claim `path` states, read off `mode`'s read-out: each query's
/// outcome in order, then the program's.
fn claims(path: &Path, mode: Mode, agenda: bool) -> Result<String, String> {
    under(agenda, || {
        let (mut kb, mut terms, ast) = loaded(path).expect("loaded once already");
        let opts = explore::Options::new(mode);
        let r = ein_infer::read_out::run(&mut kb, &mut terms, &ast, &opts)
            .map_err(|e| e.to_string())?;
        let mut out: Vec<String> = r
            .queries
            .iter()
            .map(|q| format!("{:?}", q.claim.as_ref().map(|c| c.outcome)))
            .collect();
        out.push(format!("{:?}", r.claim.as_ref().map(|c| c.outcome)));
        Ok(out.join(" "))
    })
}

/// What an answer says of `R`, with what the order may move taken out: the
/// work and the order's record, every core with its status, rules and
/// sources, the instances a node owes as a set, and whether some node met
/// owes.
fn said(a: &Answer) -> String {
    let mut d = a.described.clone();
    if let Some(d) = d.as_mut() {
        d.work = Default::default();
        d.owing_met = false;
        d.root.core.clear();
        d.root.core_status = None;
        d.root.refuted_by.clear();
        d.root.sources.clear();
        d.root.owed.sort();
        for x in &mut d.dead {
            x.core.clear();
            x.core_status = None;
            x.refuted_by.clear();
        }
        for o in &mut d.frontier {
            o.owed.sort();
        }
        if let Some(shape) = d.shape.as_mut()
            && let Some((_, holes)) = shape.holes.as_mut()
        {
            for h in holes {
                h.core.clear();
                h.core_status = None;
            }
        }
    }
    format!(
        "{:?} {:?} {:?} {:?} {} {:?} {:?} {:?} {:?} {:?} {:?}",
        a.states,
        a.minimal,
        a.counts,
        a.shape,
        a.exhausted,
        a.verdict,
        a.stopped.is_some(),
        a.route,
        a.root_dead,
        a.root_owes,
        d
    )
}

/// A listing as a set of states, each its closure.
fn closures(a: &Answer) -> BTreeSet<Vec<u32>> {
    a.states.iter().map(|s| s.closure.to_vec()).collect()
}

#[derive(Default)]
struct Tally {
    failures: Vec<String>,
    /// Programs read, monotone and flagged.
    monotone: usize,
    flagged: usize,
    /// Per question, over the cells of a monotone program, stopped ones with
    /// what they spent: saturations and facts under the agenda, then in the
    /// written order.
    work: std::collections::BTreeMap<&'static str, [u64; 4]>,
    /// Cells both orders answered, on monotone programs.
    cells: usize,
    /// Cells a budget stopped under either order.
    stopped: Vec<String>,
    /// Members of `N` whose core, status or rules moved, of those compared.
    cores: (usize, usize),
    /// Cells whose work moved, finished both ways.
    work_moved: usize,
    /// Cells whose two listings, both stopped as asked, hold other states.
    other_states: Vec<String>,
    /// Programs whose claims were read, and the claims.
    claimed: usize,
    claims: usize,
    /// Listings held to no finished describe or maximal line.
    unheld: Vec<String>,
}

/// What the sweep reads of a program before its questions, a unit of its
/// first pass: `None` where it does not load, else the two questions its
/// claims are read off, the one `ein test` asks first and the describing
/// one, where it states a claim ([`claims_held`]).
fn claimed(path: &Path) -> Option<Option<[Mode; 2]>> {
    let (kb, _, ast) = loaded(path)?;
    let program = kb.program();
    let stated = program
        .queries
        .iter()
        .any(|q| ein_infer::query_value(&ast, q, "expect").is_some())
        || program
            .solve()
            .is_some_and(|s| ein_infer::solve_value(&ast, s, "expect").is_some());
    Some(stated.then(|| {
        [
            ein_infer::read_out::least(&kb, &ast),
            ein_infer::read_out::describing(&kb, &ast),
        ]
    }))
}

/// One program that loads: its questions under both orders, held to the
/// module's definition of *the same answer*. `answers` are the runs, each
/// question's under the agenda and then in the written order, in
/// [`questions`]' order, each a unit of the sweep; a program found flagged
/// at its first question that answers is left there, and the runs after it
/// need not be given.
fn held(path: &Path, answers: Vec<Result<Answer, String>>) -> Tally {
    let mut t = Tally::default();
    let rel = rel_of(path);
    let mut r: Option<BTreeSet<Vec<u32>>> = None;
    let mut line: Option<BTreeSet<Vec<u32>>> = None;
    let mut listed: Vec<(String, BTreeSet<Vec<u32>>)> = Vec::new();
    let mut answers = answers.into_iter();
    for (flags, mode) in questions() {
        let at = format!("{rel} {flags}");
        let agenda = answers.next().expect("the run under the agenda");
        let written = answers.next().expect("the run in the written order");
        let (a, b) = match (agenda, written) {
            (Ok(a), Ok(b)) => (a, b),
            (Err(x), Err(y)) if x == y => continue,
            (x, y) => {
                t.failures
                    .push(format!("{at}: {:?} against {:?}", x.err(), y.err()));
                continue;
            }
        };
        if !a.monotonicity.is_monotone() {
            // Nothing is held of a flagged program (the module's header).
            t.flagged += 1;
            return t;
        }
        t.cells += 1;
        let w = t.work.entry(flags).or_default();
        w[0] += a.saturations;
        w[1] += a.facts;
        w[2] += b.saturations;
        w[3] += b.facts;
        let describing = mode.which == Which::All && matches!(mode.ask, Ask::Describe | Ask::Count);
        if (a.root_dead, a.root_owes, &a.root.owed) != (b.root_dead, b.root_owes, &b.root.owed) {
            let (mut x, mut y) = (a.root.owed.clone(), b.root.owed.clone());
            x.sort();
            y.sort();
            if (a.root_dead, a.root_owes, x) != (b.root_dead, b.root_owes, y) {
                t.failures.push(format!("{at}: root differs"));
            }
        }
        if a.verdict.is_some() && b.verdict.is_some() && a.verdict != b.verdict {
            t.failures
                .push(format!("{at}: {:?} against {:?}", a.verdict, b.verdict));
        }
        match (a.stopped.is_some(), b.stopped.is_some()) {
            (false, false) if describing => {
                if said(&a) != said(&b) {
                    t.failures.push(format!(
                        "{at}: the two descriptions differ beyond the order"
                    ));
                }
                // The state route's order is the states': its saturations do
                // not move. The tree and the classes walk the obligation tree,
                // whose coupled order reads its share of the run's facts.
                if a.route == Route::States && a.saturations != b.saturations {
                    t.failures.push(format!(
                        "{at}: {} saturations under the agenda, {} in the written order",
                        a.saturations, b.saturations
                    ));
                }
                t.work_moved += usize::from((a.saturations, a.facts) != (b.saturations, b.facts));
                if mode.ask == Ask::Describe
                    && let (Some(da), Some(db)) = (a.described.as_ref(), b.described.as_ref())
                {
                    for (x, y) in da.dead.iter().zip(&db.dead) {
                        t.cores.1 += 1;
                        t.cores.0 += usize::from(
                            (&x.core, x.core_status, &x.refuted_by)
                                != (&y.core, y.core_status, &y.refuted_by),
                        );
                    }
                }
                if mode.ask == Ask::Describe && a.exhausted {
                    r = a
                        .described
                        .as_ref()
                        .map(|d| d.states.iter().map(|s| s.closure.to_vec()).collect());
                }
            }
            (false, false) => {
                t.work_moved += usize::from((a.saturations, a.facts) != (b.saturations, b.facts));
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
            }
            _ => t.stopped.push(format!(
                "{at}: {} saturations under the agenda{}, {} in the written order{}",
                a.saturations,
                if a.stopped.is_some() { ", stopped" } else { "" },
                b.saturations,
                if b.stopped.is_some() { ", stopped" } else { "" }
            )),
        }
        match &r {
            Some(r) => {
                for (who, x) in [("under the agenda", &a), ("in the written order", &b)] {
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
    if t.cells == 0 {
        return t;
    }
    t.monotone += 1;
    t
}

/// Every claim of a monotone program that states one, off its two
/// questions ([`claimed`]): `read` is each question's claims under the
/// agenda and then in the written order, each run a unit of the sweep.
fn claims_held(path: &Path, modes: &[Mode; 2], read: Vec<Result<String, String>>, t: &mut Tally) {
    let rel = rel_of(path);
    t.claimed += 1;
    let mut read = read.into_iter();
    for mode in modes {
        t.claims += 1;
        let agenda = read.next().expect("the claims under the agenda");
        let written = read.next().expect("the claims in the written order");
        match (agenda, written) {
            (Ok(x), Ok(y)) if x == y => {}
            (Err(x), Err(y)) if x == y => {}
            (x, y) => t.failures.push(format!(
                "{rel} {}: the claims read {x:?} under the agenda and {y:?} in the written \
                 order",
                mode.flags()
            )),
        }
    }
}

/// **Every question and every claim answers alike under the agenda and in
/// the written order**, on every monotone corpus program and register
/// witness: the module's definition of *the same answer*, cell by cell.
#[test]
fn every_question_and_claim_answers_alike_under_the_agenda_and_the_written_order() {
    // The corpus's `slow` entries too, where the run asks for them: the one
    // test of this binary that reads the flag.
    let slow_too = std::env::var("EIN_CORPUS_SLOW").as_deref() == Ok("1");
    let files = files(slow_too);
    // Each run a unit of its own, so that a program's runs go side by side,
    // in four passes folded per program in the programs' order: which
    // programs load and the questions their claims are read off; each
    // one's first question under both orders, which says whether it is
    // flagged; the other questions of those that are not; and the claims of
    // the monotone ones that state one.
    const SWEEP: &str = "agenda::under_both_orders";
    let asked = questions();
    let at = |f: usize, q: usize, agenda: bool| {
        (
            rel_of(&files[f]),
            format!(
                "{} {}",
                asked[q].0,
                if agenda { "agenda" } else { "written order" }
            ),
        )
    };
    let pre = ein_corpus::pool::map(
        SWEEP,
        &files,
        |p| (rel_of(p), "load".into()),
        |p| claimed(p),
    );
    let firsts: Vec<(usize, usize, bool)> = (0..files.len())
        .filter(|&f| pre[f].is_some())
        .flat_map(|f| [(f, 0, true), (f, 0, false)])
        .collect();
    let first = ein_corpus::pool::map(
        SWEEP,
        &firsts,
        |&(f, q, agenda)| at(f, q, agenda),
        |&(f, q, agenda)| ask(&files[f], asked[q].1, agenda),
    );
    let flagged_at_first = |pair: &[Result<Answer, String>]| matches!(pair, [Ok(a), Ok(_)] if !a.monotonicity.is_monotone());
    let loaded_files: Vec<usize> = (0..files.len()).filter(|&f| pre[f].is_some()).collect();
    let rests: Vec<(usize, usize, bool)> = loaded_files
        .iter()
        .zip(first.chunks(2))
        .filter(|(_, pair)| !flagged_at_first(pair))
        .flat_map(|(&f, _)| (1..asked.len()).flat_map(move |q| [(f, q, true), (f, q, false)]))
        .collect();
    let rest = ein_corpus::pool::map(
        SWEEP,
        &rests,
        |&(f, q, agenda)| at(f, q, agenda),
        |&(f, q, agenda)| ask(&files[f], asked[q].1, agenda),
    );
    let mut first = first.into_iter();
    let mut rest = rest.into_iter();
    let mut results: Vec<Tally> = Vec::new();
    for f in 0..files.len() {
        if pre[f].is_none() {
            results.push(Tally::default());
            continue;
        }
        let mut answers: Vec<Result<Answer, String>> = first.by_ref().take(2).collect();
        if !flagged_at_first(&answers) {
            answers.extend(rest.by_ref().take(2 * (asked.len() - 1)));
        }
        results.push(held(&files[f], answers));
    }
    let claimed_by: Vec<(usize, usize, bool)> = (0..files.len())
        .filter(|&f| results[f].cells > 0 && matches!(pre[f], Some(Some(_))))
        .flat_map(|f| (0..2).flat_map(move |m| [(f, m, true), (f, m, false)]))
        .collect();
    let modes_of = |f: usize| pre[f].as_ref().and_then(|m| m.as_ref()).expect("stated");
    let read = ein_corpus::pool::map(
        SWEEP,
        &claimed_by,
        |&(f, m, agenda)| {
            (
                rel_of(&files[f]),
                format!(
                    "claims {} {}",
                    modes_of(f)[m].flags(),
                    if agenda { "agenda" } else { "written order" }
                ),
            )
        },
        |&(f, m, agenda)| claims(&files[f], modes_of(f)[m], agenda),
    );
    let mut read = read.into_iter();
    for (f, t) in results.iter_mut().enumerate() {
        if t.cells > 0
            && let Some(Some(modes)) = &pre[f]
        {
            claims_held(&files[f], modes, read.by_ref().take(4).collect(), t);
        }
    }
    let mut all = Tally::default();
    for t in results {
        all.failures.extend(t.failures);
        all.monotone += t.monotone;
        all.flagged += t.flagged;
        all.cells += t.cells;
        all.stopped.extend(t.stopped);
        all.work_moved += t.work_moved;
        all.cores.0 += t.cores.0;
        all.cores.1 += t.cores.1;
        all.other_states.extend(t.other_states);
        all.claimed += t.claimed;
        all.claims += t.claims;
        all.unheld.extend(t.unheld);
        for (q, w) in t.work {
            for (x, y) in all.work.entry(q).or_default().iter_mut().zip(w) {
                *x += y;
            }
        }
    }
    eprintln!(
        "{} monotone programs, {} cells, the work moved on {}, N's cores on {} members of {}; {} \
         flagged programs not held; \
         {} programs' claims read twice each under both orders ({} readings); other states in a \
         stopped listing: {:?}",
        all.monotone,
        all.cells,
        all.work_moved,
        all.cores.0,
        all.cores.1,
        all.flagged,
        all.claimed,
        all.claims,
        all.other_states
    );
    for (q, w) in &all.work {
        eprintln!(
            "{q}: {} saturations and {} facts under the agenda, {} and {} in the written order",
            w[0], w[1], w[2], w[3]
        );
    }
    for s in &all.stopped {
        eprintln!("stopped: {s}");
    }
    assert!(
        all.failures.is_empty(),
        "{} cell(s) answer otherwise under the agenda:\n{}",
        all.failures.len(),
        all.failures.join("\n")
    );
    assert!(
        all.unheld.is_empty(),
        "a listing that moved is held to R by no finished describe or maximal line: {:?}",
        all.unheld
    );
    // Not vacuous: the monotone programs are read, their claims too, and the
    // order moves the work somewhere. A shard of the nightly's holds its
    // share of each floor.
    let share = ein_corpus::shard::share;
    assert!(
        all.monotone >= share(400) && all.cells >= share(5 * 400) && all.claimed >= share(200),
        "{} monotone programs, {} cells, {} with claims",
        all.monotone,
        all.cells,
        all.claimed
    );
    assert!(
        all.work_moved >= share(20),
        "the work moved on {} cells",
        all.work_moved
    );
}
