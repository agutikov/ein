//! **L9, a fact nothing proposes and nothing writes is in no state**
//! (`ein_infer::unheld`), held four ways:
//!
//! - **Where it licenses**: over every corpus program and regression witness,
//!   the instances L9 licenses, each one L5 refuses for a witness that is no
//!   candidate and not denied, and the programs it does not reach named with
//!   why.
//! - **Against the oracle.** On every monotone cube the oracle enumerates
//!   with an instance L9 licenses: no node of the cube, alive or dead, holds
//!   a fact beyond root's, no candidate, of a relation L9 relies on (the
//!   premise, read at every node and not only at those a route computes);
//!   an alive state owes the instance exactly when it holds none of the
//!   alternatives; and every member of `R` above a state that owes it holds
//!   one of them (the licence). `explore.rs` holds every mode to the oracle
//!   on the same cubes.
//! - **With and without.** Every question of `ein solve` on every corpus
//!   program and regression witness with an instance L9 licenses is asked
//!   twice, the route's table with L9 and L5's alone
//!   (`unheld::without`, a switch on the calling thread that no lever
//!   reaches): one answer, *one answer* as `known_dead.rs` defines it; the
//!   read-out's `route` row names L9 only with it, and only where a branch
//!   took such an instance.
//! - **The premise broken.** Under `unheld::trusting`, which calls every
//!   relation unwritten, a program whose rule writes a witness of its debt
//!   has that instance licensed, and the route refuses the run at the first
//!   node that holds the witness.
//!
//! ```text
//! cargo test -p ein-infer --profile gate --test unheld
//! ```

use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

use ein_core::{Kb, Terms};
use ein_corpus::repo_root;
use ein_infer::explore::{self, Answer, Ask, Licence, Mode, Which};
use ein_infer::region::{self, Options, Outcome};
use ein_infer::unheld;
use ein_ir::{Ast, load, parse};

fn loaded(path: &Path) -> Option<(Kb, Terms, Ast)> {
    let text = std::fs::read_to_string(path).ok()?;
    let mut ast = Ast::new();
    let mut terms = Terms::new();
    let forms = parse(&mut ast, &text, path.to_str()).ok()?;
    let kb = load(&mut ast, &mut terms, &forms, path.parent()).ok()?;
    Some((kb, terms, ast))
}

fn loaded_text(name: &str, text: &str) -> (Kb, Terms, Ast) {
    let mut ast = Ast::new();
    let mut terms = Terms::new();
    let forms = parse(&mut ast, text, Some(name)).unwrap_or_else(|e| panic!("{name}: {e}"));
    let kb = load(&mut ast, &mut terms, &forms, None).unwrap_or_else(|e| panic!("{name}: {e}"));
    (kb, terms, ast)
}

fn rel_of(path: &Path) -> String {
    path.strip_prefix(repo_root())
        .unwrap_or(path)
        .to_string_lossy()
        .to_string()
}

/// Every corpus file and regression witness, once.
fn corpus_and_regressions() -> Vec<PathBuf> {
    let mut files = ein_corpus::corpus_files();
    for row in ein_corpus::regressions().case {
        files.push(repo_root().join(&row.witness));
    }
    files.sort();
    files.dedup();
    files
}

/// One instance L9 licenses: its text, its alternatives (bits), how many of
/// its witnesses no node can hold, and their relation.
struct Licensed {
    text: String,
    alternatives: Vec<u32>,
    unheld: u32,
    relation: String,
}

/// The instances of `path`'s table that L9 licenses, each with L5's reason
/// for refusing it; `None` where the program has no table.
fn licensed(path: &Path) -> Option<Vec<(Licensed, String)>> {
    let (mut kb, mut terms, ast) = loaded(path)?;
    let table = ein_infer::oblgen::alternatives_of(&mut kb, &mut terms, &ast).ok()??;
    Some(
        table
            .instances
            .iter()
            .filter_map(|i| {
                let u = i.unheld.as_ref()?;
                let why = i.alternatives.as_ref().err()?.clone();
                let (rule, activator, bindings) = &i.key;
                let text = format!(
                    "({} {}) {}",
                    terms.sym(*rule),
                    activator
                        .iter()
                        .map(|v| terms.display(*v))
                        .collect::<Vec<_>>()
                        .join(" "),
                    bindings
                        .iter()
                        .map(|(k, v)| format!("{}={}", terms.sym(*k), terms.display(*v)))
                        .collect::<Vec<_>>()
                        .join(" ")
                );
                Some((
                    Licensed {
                        text,
                        alternatives: u.alternatives.to_vec(),
                        unheld: u.witnesses,
                        relation: terms.sym(u.relation).to_string(),
                    },
                    why,
                ))
            })
            .collect(),
    )
}

/// **Where L9 licenses**: the instances, program by program, over the whole
/// corpus and register. Each is an instance L5 refuses because a witness is
/// no candidate and root does not deny it; each has a candidate among its
/// witnesses to branch on; and each relies on `sits`, which no rule of
/// either program concludes. No other program's unlicensed instance is
/// L9's: the Zebra puzzle's `slot-owed-fill` debts (`co-located` is written
/// by the slot rules and mirrored), which the list's being whole says
/// without naming the puzzle here, and `13_witness_of_two.ein`'s (`fed` is
/// written by two rules), held below.
#[test]
fn l9_licenses_where_nothing_writes_the_witness_relation() {
    let files = corpus_and_regressions();
    let found = ein_corpus::pool::map(
        "unheld::census",
        &files,
        |p| (rel_of(p), String::new()),
        |p| licensed(p).map(|l| (rel_of(p), l)),
    );
    let mut by_program: Vec<(String, Vec<String>)> = Vec::new();
    for (rel, instances) in found.into_iter().flatten() {
        if instances.is_empty() {
            continue;
        }
        let mut texts = Vec::new();
        for (l, why) in instances {
            assert!(
                why.contains("no candidate proposes") || why.contains("is not a candidate"),
                "{rel}: {}: L5's reason is another: {why}",
                l.text
            );
            assert!(
                !l.alternatives.is_empty() && l.unheld > 0,
                "{rel}: {}: {} alternatives, {} witnesses no node holds",
                l.text,
                l.alternatives.len(),
                l.unheld
            );
            texts.push(format!(
                "{}: {} alternatives, {} unheld `{}`",
                l.text,
                l.alternatives.len(),
                l.unheld,
                l.relation
            ));
        }
        by_program.push((rel, texts));
    }
    let want: Vec<(String, Vec<String>)> = [
        (
            "examples/lattice/06_friends_tables_everyone_sits.ein",
            vec![
                "(total-owed sits is-a) R=sits isa=is-a A=Friend B=Seat a=Cat: 3 alternatives, 2 unheld `sits`",
                "(total-owed sits is-a) R=sits isa=is-a A=Friend B=Seat a=Dan: 3 alternatives, 2 unheld `sits`",
                "(total-owed sits is-a) R=sits isa=is-a A=Friend B=Seat a=Eve: 3 alternatives, 2 unheld `sits`",
            ],
        ),
        (
            "examples/lattice/14_seats_nothing_proposes.ein",
            vec![
                "(total-owed sits is-a) R=sits isa=is-a A=Friend B=Seat a=Cat: 2 alternatives, 2 unheld `sits`",
                "(total-owed sits is-a) R=sits isa=is-a A=Friend B=Seat a=Dan: 2 alternatives, 2 unheld `sits`",
            ],
        ),
    ]
    .into_iter()
    .map(|(p, v)| (p.to_string(), v.into_iter().map(String::from).collect()))
    .collect();
    assert_eq!(by_program, want);
    // The implied witness's program it does not reach, and why: its witness
    // relation is written.
    let rel = "examples/lattice/13_witness_of_two.ein";
    let (mut kb, mut terms, ast) = loaded(&repo_root().join(rel)).expect("loads");
    let t = ein_infer::oblgen::alternatives_of(&mut kb, &mut terms, &ast)
        .expect("prepares")
        .expect("a table");
    let not = t
        .instances
        .iter()
        .filter(|i| i.alternatives.is_err() && i.unheld.is_none())
        .count();
    assert_eq!(not, 1, "{rel}");
}

/// **L9 is the oracle's.** On every monotone cube the oracle enumerates
/// with an instance L9 licenses, at every node and every alive state of the
/// cube: the module's three readings. Not vacuous: `14_seats_nothing_proposes`
/// is read, two instances L9 licenses, owed at states the cube holds.
#[test]
fn l9_is_the_oracle_on_every_monotone_program_it_reaches() {
    use ein_infer::events::Events;
    let oracle = Options {
        jobs: std::thread::available_parallelism().map_or(1, |n| n.get().min(8)),
        ..Options::default()
    };
    let mut failures: Vec<String> = Vec::new();
    let (mut programs, mut nodes_read, mut owing) = (0usize, 0usize, 0usize);
    for path in corpus_and_regressions() {
        let rel = rel_of(&path);
        let Some(lic) = licensed(&path) else {
            continue;
        };
        if lic.is_empty() {
            continue;
        }
        let Some((mut kb, mut terms, ast)) = loaded(&path) else {
            continue;
        };
        let Ok(Outcome::Region(r)) =
            region::compute(&mut kb, &mut terms, &ast, &oracle, &mut |_| {})
        else {
            continue;
        };
        if !r.monotonicity.is_monotone() {
            continue;
        }
        programs += 1;
        let (mut kb1, mut terms1, ast1) = loaded(&path).expect("loaded once already");
        let table = ein_infer::oblgen::alternatives_of(&mut kb1, &mut terms1, &ast1)
            .expect("prepares")
            .expect("a table")
            .with_l9();
        let relations: BTreeSet<String> = table
            .unheld_relations
            .iter()
            .map(|&s| terms1.sym(s).to_string())
            .collect();
        let l9: Vec<(usize, u64)> = table
            .instances
            .iter()
            .enumerate()
            .filter(|(i, _)| table.by_l9(*i as u32))
            .map(|(i, x)| {
                let alts = x.alternatives.as_ref().expect("licensed");
                (i, alts.iter().fold(0u64, |m, &b| m | 1 << b))
            })
            .collect();
        let n = r.n();
        let root_facts: BTreeSet<String> = kb1
            .facts()
            .map(|f| ein_infer::events::sexpr(&terms1, f))
            .collect();
        let candidates: BTreeSet<String> = r.candidates.iter().map(|c| c.text.clone()).collect();
        let in_r: Vec<u64> = (0..1u64 << n).filter(|&m| r.in_r(m)).collect();
        let memo = ein_infer::SharedMemo::default();
        let mut nodes = region::Nodes::new(&mut kb1, &mut terms1, &ast1, &r.candidates)
            .unwrap_or_else(|e| panic!("{rel}: {e}"));
        for m in 0..1u64 << n {
            let facts: Vec<_> = (0..n)
                .filter(|b| m >> b & 1 == 1)
                .map(|b| r.candidates[b].fact)
                .collect();
            let (breach, owed) = nodes
                .with_fixpoint(&facts, |kb, terms| {
                    let breach: Vec<String> = kb
                        .facts()
                        .map(|f| ein_infer::events::sexpr(terms, f))
                        .filter(|t| {
                            relations.iter().any(|r| t.starts_with(&format!("({r} ")))
                                && !root_facts.contains(t)
                                && !candidates.contains(t)
                        })
                        .collect();
                    let owed: Vec<Option<u32>> =
                        ein_infer::obligations::tally(kb, terms, &ast1, &memo, &mut Events::off())
                            .expect("the tally compiles")
                            .instances()
                            .iter()
                            .map(|o| {
                                let key = (o.rule, o.activator.clone(), o.bindings.clone());
                                table.index.get(&key).copied()
                            })
                            .collect();
                    (breach, owed)
                })
                .unwrap_or_else(|e| panic!("{rel}: {e}"));
            nodes_read += 1;
            if !breach.is_empty() {
                failures.push(format!(
                    "{rel}: {} holds {breach:?}, which nothing proposes or writes",
                    r.name(m)
                ));
            }
            let alive = r.status[m as usize] & region::DEAD == 0;
            if !alive || r.nodes[m as usize].closure != m {
                continue;
            }
            for &(i, alts) in &l9 {
                let witnessed = m & alts != 0;
                let is_owed = owed.contains(&Some(i as u32));
                if witnessed == is_owed {
                    failures.push(format!(
                        "{rel}: {} {} instance {i}, with {} alternative in it",
                        r.name(m),
                        if is_owed { "owes" } else { "has paid" },
                        if witnessed { "an" } else { "no" }
                    ));
                }
                if !is_owed {
                    continue;
                }
                owing += 1;
                for &d in &in_r {
                    if d & m == m && r.nodes[d as usize].closure & alts == 0 {
                        failures.push(format!(
                            "{rel}: {} is in R above {}, which owes instance {i}, and holds \
                             none of its alternatives",
                            r.name(d),
                            r.name(m)
                        ));
                    }
                }
            }
        }
    }
    assert!(failures.is_empty(), "{}", failures.join("\n"));
    assert!(
        programs >= 1 && nodes_read >= 4096 && owing >= 2,
        "{programs} programs, {nodes_read} nodes, {owing} owed instances L9 licenses"
    );
    eprintln!("L9: {programs} programs, {nodes_read} nodes, {owing} owed instances L9 licenses");
}

/// Every question of `ein solve`: describe, `--count`, `--single`, `-n 1`,
/// each line whole and each line under `-n 1` and `--single`.
fn questions() -> Vec<(&'static str, Mode)> {
    let m = |which, ask| Mode { which, ask };
    vec![
        ("describe", m(Which::All, Ask::Describe)),
        ("--count", m(Which::All, Ask::Count)),
        ("--single", m(Which::All, Ask::Single)),
        ("-n 1", m(Which::All, Ask::Find(1))),
        ("--minimal", m(Which::Minimal, Ask::Describe)),
        ("--maximal", m(Which::Maximal, Ask::Describe)),
        ("--minimal -n 1", m(Which::Minimal, Ask::Find(1))),
        ("--minimal --single", m(Which::Minimal, Ask::Single)),
        ("--maximal -n 1", m(Which::Maximal, Ask::Find(1))),
        ("--maximal --single", m(Which::Maximal, Ask::Single)),
    ]
}

fn ask(path: &Path, mode: Mode, with_l9: bool) -> Result<Answer, String> {
    let go = || {
        let (mut kb, mut terms, ast) = loaded(path).expect("loaded once already");
        let opts = explore::Options::new(mode);
        explore::run(&mut kb, &mut terms, &ast, &opts).map_err(|e| e.to_string())
    };
    if with_l9 { go() } else { unheld::without(go) }
}

fn closures(a: &Answer) -> BTreeSet<Vec<u32>> {
    a.states.iter().map(|s| s.closure.to_vec()).collect()
}

/// **Every question answers alike with L9 and without it**, on every corpus
/// program and regression witness with an instance L9 licenses: one verdict,
/// one `exhausted`, a part of `R` listed whole the same states and minimal
/// commitments, a listing stopped as asked as many states, each in `R` as
/// describe lists it; describe and `--count` every field of R both compute
/// (`Described::differences`), the tree and the classes taking them where L9
/// lets the tree branch and the state route without it. And the work: with
/// L9 the
/// tree's questions saturate fewer nodes and derive fewer facts over these
/// programs, the price list's first clause, each cell printed.
#[test]
fn every_question_answers_alike_with_l9_and_without() {
    let files: Vec<PathBuf> = corpus_and_regressions()
        .into_iter()
        .filter(|p| licensed(p).is_some_and(|l| !l.is_empty()))
        .collect();
    assert!(files.len() >= 2, "{files:?}");
    let mut failures: Vec<String> = Vec::new();
    let mut totals: Vec<(&str, [u64; 4])> = questions().iter().map(|(q, _)| (*q, [0; 4])).collect();
    let mut named = 0usize;
    for path in &files {
        let rel = rel_of(path);
        let mut r: Option<BTreeSet<Vec<u32>>> = None;
        for (k, (flags, mode)) in questions().into_iter().enumerate() {
            let at = format!("{rel} {flags}");
            let (a, b) = match (ask(path, mode, true), ask(path, mode, false)) {
                (Ok(a), Ok(b)) => (a, b),
                (x, y) => {
                    failures.push(format!("{at}: {:?} against {:?}", x.err(), y.err()));
                    continue;
                }
            };
            eprintln!(
                "{at}: with L9 {} saturations · {} facts, without {} · {}",
                a.saturations, a.facts, b.saturations, b.facts
            );
            let t = &mut totals[k].1;
            t[0] += a.saturations;
            t[1] += a.facts;
            t[2] += b.saturations;
            t[3] += b.facts;
            if b.licences.contains(&Licence::Unheld) {
                failures.push(format!("{at}: a run without L9 names it"));
            }
            named += usize::from(a.licences.contains(&Licence::Unheld));
            if a.stopped.is_some() || b.stopped.is_some() {
                failures.push(format!("{at}: stopped, within the default budget"));
                continue;
            }
            let describing =
                mode.which == Which::All && matches!(mode.ask, Ask::Describe | Ask::Count);
            if describing {
                // Describe takes the tree and the classes where the tree
                // branches, which L9 decides here, and the state route
                // without it: two routes, one R.
                let diff = match (a.described.as_deref(), b.described.as_deref()) {
                    (Some(x), Some(y)) => x.differences(y),
                    _ => vec!["the description"],
                };
                if !diff.is_empty() {
                    failures.push(format!("{at}: {diff:?} moved without L9's table"));
                }
                if mode.ask == Ask::Describe {
                    r = a
                        .described
                        .as_ref()
                        .map(|d| d.states.iter().map(|s| s.closure.to_vec()).collect());
                }
                continue;
            }
            if (a.verdict, a.exhausted) != (b.verdict, b.exhausted) {
                failures.push(format!(
                    "{at}: {:?}, exhausted {}, against {:?}, exhausted {}",
                    a.verdict, a.exhausted, b.verdict, b.exhausted
                ));
            } else if a.exhausted {
                let minimal = |x: &Answer| -> BTreeSet<Vec<u32>> {
                    x.minimal.iter().map(|(g, _)| g.to_vec()).collect()
                };
                if closures(&a) != closures(&b) || minimal(&a) != minimal(&b) {
                    failures.push(format!("{at}: one part of R, listed whole, two ways"));
                }
            } else if a.states.len() != b.states.len() {
                failures.push(format!(
                    "{at}: {} states listed against {}",
                    a.states.len(),
                    b.states.len()
                ));
            }
            let r = r.as_ref().expect("describe is asked first and finishes");
            for (who, x) in [("with L9", &a), ("without", &b)] {
                if !closures(x).is_subset(r) {
                    failures.push(format!("{at}: {who} lists a state describe does not"));
                }
            }
        }
    }
    assert!(failures.is_empty(), "{}", failures.join("\n"));
    // L9 is named where a branch took it, and the tree's questions do less.
    assert!(named >= 2 * 8, "L9 named on {named} cells");
    for (q, [s, f, s0, f0]) in totals {
        eprintln!("{q}: with L9 {s} saturations · {f} facts, without {s0} · {f0}");
        if !q.starts_with("describe") && !q.starts_with("--count") {
            assert!(
                s < s0 && f < f0,
                "{q}: {s} · {f} with L9 against {s0} · {f0}"
            );
        }
    }
}

/// A debt `takeaway` can pay with a fact no candidate proposes: Ann's
/// witnesses are `(fed Ann Soup)`, a candidate, and `(fed Ann Stew)`, which
/// the rule writes from `(orders Ann)`.
const WRITTEN_WITNESS: &str = r#"
(import std.algebra :symbols (total-owed))
(relation is-a   T T        :why "{?3} is a {?4}")
(relation fed    Guest Dish :why "{?3} is fed {?4}")
(relation orders Guest      :why "{?3} orders a takeaway")
(is-a Ann Guest) (is-a Soup Dish) (is-a Stew Dish)
(total-owed fed is-a)
(rule takeaway ()
  :match  (orders Ann)
  :assert (fed Ann Stew)
  :why    "Ann's takeaway is stew")
(hrule feed ()
  :match  (is-a ?g Guest)
  :assert (fed ?g Soup)
  :why    "suppose {?g} is fed soup")
(hrule order ()
  :match  (is-a ?g Guest)
  :assert (orders ?g)
  :why    "suppose {?g} orders a takeaway")
(solve :hrules (feed order))
"#;

/// **A node that breaks L9's premise refuses the run.** `takeaway` writes
/// `fed`, so L9 does not license Ann's debt and every question answers
/// from L5's table: R is three states. Under `unheld::trusting`, which
/// calls every relation unwritten, the debt is licensed with `(fed Ann
/// Soup)` alone, and the first node a route computes that holds `(fed Ann
/// Stew)` beside root's facts refuses the run: an engine defect, never an
/// answer.
#[test]
fn a_node_that_breaks_the_premise_refuses_the_run() {
    let run = |mode: Mode| {
        let (mut kb, mut terms, ast) = loaded_text("written_witness", WRITTEN_WITNESS);
        explore::run(&mut kb, &mut terms, &ast, &explore::Options::new(mode))
    };
    let maximal = Mode {
        which: Which::Maximal,
        ask: Ask::Describe,
    };
    let single = Mode {
        which: Which::All,
        ask: Ask::Single,
    };
    let describe = Mode {
        which: Which::All,
        ask: Ask::Describe,
    };
    let (mut kb, mut terms, ast) = loaded_text("written_witness", WRITTEN_WITNESS);
    let t = ein_infer::oblgen::alternatives_of(&mut kb, &mut terms, &ast)
        .expect("prepares")
        .expect("a table");
    assert!(t.instances.len() == 1 && t.instances[0].unheld.is_none());
    let d = run(describe).expect("answers");
    assert_eq!(d.described.as_ref().expect("described").states.len(), 3);
    for mode in [maximal, single] {
        let a = run(mode).expect("answers");
        assert!(!a.licences.contains(&Licence::Unheld), "{mode:?}");
        let err = unheld::trusting(|| run(mode)).expect_err("the premise broken refuses");
        let err = err.to_string();
        assert!(
            err.contains("L9: a node holds `(fed Ann Stew)`"),
            "{mode:?}: {err}"
        );
    }
    // The switches are the calling thread's, and gone when they return.
    assert!(run(maximal).is_ok() && !unheld::is_without());
}
