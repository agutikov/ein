//! **Every core names its commitment, and derives its contradiction** — M1e2
//! [S1e2.2.21](../../../../plans/m1e2_correctness/p1e2.2_wrong_answers/s1e2.2.21_pair_core_one_side.md)'s
//! step 4, the class-4 claim no test held until then.
//!
//! A dead node's unsat core is the facts its refutation rests on, the
//! smallest frontier over every derivation the program records (Q-M1e2.11
//! (i)(b)). On a program whose root is alive, two things follow, and this test
//! holds both for every core a run reports:
//!
//! 1. **it names a fact root does not hold**, a fact of the node's commitment:
//!    root is alive, so the facts root holds refute nothing;
//! 2. **root with those facts derives a contradiction**: the core's
//!    commitment facts, written into root's fixpoint and saturated with
//!    root's plans, are dead. Root's facts are there already, so this is the
//!    claim *from the core, with root's plans, saturation derives `(false)`*
//!    read against root, which is what holds under `absent`: a typecheck rule
//!    over an `is-a` the core does not name would refute anything on an empty
//!    KB.
//!
//! It would have caught W19 (a core leaving out the activator its refutation
//! rests on, `(seed a)` alone) and W21 (a pair's core naming its positive side
//! alone, `zebra2.ein`'s N 201 of 366; and a `(false)` through a `forall`
//! naming the two facts its rule matched): each fails the first claim, and
//! `saturation/type-exclusivity/*`'s fail the second.
//!
//! **Which cores.** Every surface prints a core the node function takes, as
//! the walk visits a node or, for a member of N whose core the walk did not
//! take, after the walk against the walk's snapshot (M1f S1f.5.12,
//! `region::Cores`). Until S1f.5.12 `--dump-answer` and `--trace` took a
//! member of N the state route did not saturate through
//! `region::Nodes::refutation`, over a second preparation of root, and a run
//! past `--region-limit` printed its members with no core, `not-kept` (M1e2
//! S1e2.3.9). So the test reads the oracle's cube, every dead node, where the
//! oracle enumerates it, and describe's N as the run gives it, every member
//! with its core, past the limit too. Every corpus file and register witness
//! the loader accepts and *forbid* does not refuse; the slow entries under
//! `EIN_CORPUS_SLOW=1`, the nightly's.
//!
//! At S1e2.2.21's HEAD it fails, in either build, on 69 271 cores that name
//! none of their commitment and 28 that derive no contradiction (the
//! per-commit half; 2026-11-05). It reads the corpus's entries that are not
//! slow in the per-commit tier, 18 s in the dev profile, and the zebra
//! family's too in the nightly, 67 s in release (2026-11-05).
//!
//! **And no core is printed unqualified from a cut search** (M1e2
//! S1e2.2.23, W23): [`no_core_is_printed_unqualified_from_a_cut_search`]
//! reads every core a surface prints with the status printed beside it, root's
//! on a dead root and every dead node a route visits included, and holds each
//! status to its search's.
//!
//! **And every core's `refuted by` names its derivation's last step** (M1e2
//! S1e2.3.10, P1e2.3's N7): the same test reads the rules printed beside each
//! core, and holds them to one last step per fact of the contradiction the
//! core was taken over, the first the store records where the core supports
//! several (the user's ruling on Q-M1e2.21, (a)), read without the search by
//! a closure from the core ([`last_steps`]). At S1e2.3.10's HEAD it failed
//! on 65 559 distinct printed cores over 9 programs, release, per commit
//! (67 928 over 15 nightly; 2026-11-06): the first contradiction's
//! primaries, `bijection/06`'s cube most of them.
//!
//! ```text
//! cargo test -p ein-infer --release --test corpus_cores
//! EIN_CORPUS_SLOW=1 cargo test -p ein-infer --release --test corpus_cores
//! ```

use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};

use ein_core::{FactId, Kb, Terms};
use ein_corpus::repo_root;
use ein_infer::explain::{CoreStatus, ExplanationBudget, contradiction_frontier};
use ein_infer::explore::{self, Ask, Mode, Which};
use ein_infer::region::{self, Nodes, Outcome};
use ein_ir::{Ast, load, parse};

fn loaded(path: &Path) -> Option<(Kb, Terms, Ast)> {
    let text = std::fs::read_to_string(path).ok()?;
    let mut ast = Ast::new();
    let mut terms = Terms::new();
    let forms = parse(&mut ast, &text, path.to_str()).ok()?;
    let kb = load(&mut ast, &mut terms, &forms, path.parent()).ok()?;
    Some((kb, terms, ast))
}

fn slow_too() -> bool {
    std::env::var("EIN_CORPUS_SLOW").as_deref() == Ok("1")
}

/// Every corpus file and register witness, once, but the slow entries unless
/// `EIN_CORPUS_SLOW=1`.
fn files() -> Vec<PathBuf> {
    let slow: BTreeSet<PathBuf> = ein_corpus::corpus()
        .entry
        .iter()
        .filter(|e| e.slow)
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
    files.retain(|f| slow_too() || !slow.contains(f));
    files
}

fn jobs() -> usize {
    std::thread::available_parallelism().map_or(1, |n| n.get().min(8))
}

/// What a check found on one program: the failures, how many cores were
/// read, and how many of them the cube's ([`check`]) or cut ([`statuses`]).
type Checked = (Vec<String>, usize, usize);

/// [`statuses`]' failures and counts: cores read, cut, members of N a run
/// past `--region-limit` met, and cores through two rules.
type Statuses = (Vec<String>, usize, usize, usize, usize);

/// One reported core: the commitment, as candidate facts, and the core.
struct Reported {
    set: Vec<FactId>,
    core: Vec<String>,
}

/// Both claims for `reported`, each read in `nodes`' own terms: the
/// failures. Each core's commitment part is saturated once.
fn claims(rel: &str, nodes: &mut Nodes<'_>, reported: &[Reported]) -> Option<Vec<String>> {
    let mut bad = Vec::new();
    let mut parts: BTreeMap<Vec<FactId>, String> = BTreeMap::new();
    nodes
        .with_fixpoints(&[], |_, terms| {
            for r in reported {
                let set_text: BTreeMap<String, FactId> = r
                    .set
                    .iter()
                    .map(|&f| (ein_infer::events::sexpr(terms, f), f))
                    .collect();
                let mut part: Vec<FactId> = r
                    .core
                    .iter()
                    .filter_map(|c| set_text.get(c).copied())
                    .collect();
                part.sort_unstable();
                if part.is_empty() {
                    bad.push(format!(
                        "{rel}: the dead {:?} reports the core {:?}, which names none of it",
                        set_text.keys().collect::<Vec<_>>(),
                        r.core
                    ));
                    continue;
                }
                parts.entry(part).or_insert_with(|| {
                    format!("{:?} of {:?}", r.core, set_text.keys().collect::<Vec<_>>())
                });
            }
        })
        .ok()?;
    let keys: Vec<Vec<FactId>> = parts.keys().cloned().collect();
    let dead: Vec<bool> = nodes
        .with_fixpoints(&keys, |kbs, terms| {
            kbs.iter()
                .map(|k| ein_infer::has_contradiction(k, terms))
                .collect()
        })
        .ok()?;
    for (key, dead) in keys.iter().zip(dead) {
        if !dead {
            bad.push(format!(
                "{rel}: the core {}: root with its commitment facts derives no contradiction",
                parts[key]
            ));
        }
    }
    Some(bad)
}

/// Both claims for every core `path`'s runs report: the cube's dead nodes
/// where the oracle enumerates it, and N's members as describe gives them,
/// which `--dump-answer` and `--trace` print. The failures and how many cores
/// were read, the
/// cube's among them; `None` where the program does not load, is refused, or
/// its root is dead.
fn check(path: &Path) -> Option<Checked> {
    let rel = path
        .strip_prefix(repo_root())
        .unwrap_or(path)
        .to_string_lossy()
        .to_string();
    let mut bad = Vec::new();
    let mut read = 0usize;
    // The cube.
    let (mut kb, mut terms, ast) = loaded(path)?;
    let mut cube: Vec<Reported> = Vec::new();
    match region::compute(
        &mut kb,
        &mut terms,
        &ast,
        &region::Options::default(),
        &mut |_| {},
    ) {
        Err(_) => return None,
        Ok(Outcome::Region(r)) => {
            if r.root_dead {
                return None;
            }
            for (mask, node) in r.nodes.iter().enumerate() {
                if mask == 0 || r.status[mask] & region::DEAD == 0 {
                    continue;
                }
                let set = (0..r.n())
                    .filter(|b| mask >> b & 1 == 1)
                    .map(|b| r.candidates[b].fact)
                    .collect();
                cube.push(Reported {
                    set,
                    core: node.core.clone(),
                });
            }
        }
        Ok(_) => {}
    }
    if !cube.is_empty() {
        let mut nodes = Nodes::new(&mut kb, &mut terms, &ast, &[]).ok()?;
        bad.extend(claims(&rel, &mut nodes, &cube)?);
    }
    read += cube.len();
    // N.
    let (mut kb, mut terms, ast) = loaded(path)?;
    let opts = explore::Options::new(Mode {
        which: Which::All,
        ask: Ask::Describe,
    });
    let a = explore::run(&mut kb, &mut terms, &ast, &opts).ok()?;
    if a.root_dead {
        return None;
    }
    let sets: Vec<Reported> = a
        .described
        .as_ref()
        .map(|d| {
            d.dead
                .iter()
                .filter(|x| !x.set.is_empty())
                .map(|x| Reported {
                    set: x
                        .set
                        .iter()
                        .map(|&b| a.candidates[b as usize].fact)
                        .collect(),
                    core: x.core.clone(),
                })
                .collect()
        })
        .unwrap_or_default();
    if !sets.is_empty() {
        let mut nodes = Nodes::new(&mut kb, &mut terms, &ast, &a.candidates).ok()?;
        bad.extend(claims(&rel, &mut nodes, &sets)?);
        read += sets.len();
    }
    Some((bad, read, cube.len()))
}

/// **Every core a run reports for a dead node on an alive root names its
/// commitment, and with root derives the contradiction** — over the corpus
/// and the register's witnesses, a file per thread.
#[test]
fn every_core_names_its_commitment_and_derives_its_contradiction() {
    let files = files();
    let next = std::sync::atomic::AtomicUsize::new(0);
    let results: std::sync::Mutex<Vec<(usize, Option<Checked>)>> =
        std::sync::Mutex::new(Vec::new());
    std::thread::scope(|scope| {
        for _ in 0..jobs() {
            scope.spawn(|| {
                loop {
                    let i = next.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                    let Some(path) = files.get(i) else { break };
                    let got = check(path);
                    results.lock().expect("no checker panicked").push((i, got));
                }
            });
        }
    });
    let mut results = results.into_inner().expect("no checker panicked");
    results.sort_by_key(|(i, _)| *i);
    let mut failures: Vec<String> = Vec::new();
    let (mut programs, mut cores, mut cube) = (0usize, 0usize, 0usize);
    for (_, got) in results {
        let Some((bad, n, c)) = got else { continue };
        programs += 1;
        cores += n;
        cube += c;
        failures.extend(bad);
    }
    assert!(
        failures.is_empty(),
        "{}",
        failures.join(
            "
"
        )
    );
    // Not vacuous: 203 programs, 163 477 cores read, 162 363 of them the
    // cubes' (2026-11-05, at S1e2.2.21 T4; under `EIN_CORPUS_SLOW=1` 217,
    // 233 212 and 227 786).
    assert!(programs >= 100, "{programs} programs read");
    assert!(
        cores >= 100_000 && cube >= 100_000,
        "{cores} cores, {cube} the cubes'"
    );
    eprintln!("{programs} programs, {cores} cores, {cube} of them the cubes'");
}

// ── No core is printed unqualified from a cut search — M1e2 S1e2.2.23 ─────

/// A budget sixteen times the default's on every cap: where the default's
/// search is inside its budget, this one does the same work and names the
/// same core.
const LARGER: ExplanationBudget = ExplanationBudget {
    max_environments: 1_024,
    max_rounds: 1_024,
    max_env_size: None,
    max_facts: 320_000,
};

/// A printed core as [`Claims`] keys it: the commitment, the core, its
/// status and its rules.
type Seen = (Vec<FactId>, Vec<String>, Option<CoreStatus>, Vec<String>);

/// One core a surface prints, with the status and the rules printed beside
/// it.
struct Stated {
    /// Where it was read: `cube`, `root`, `N`, or the mode whose visit took it.
    at: &'static str,
    set: Vec<FactId>,
    core: Vec<String>,
    status: Option<CoreStatus>,
    /// Its `refuted by` (M1e2 S1e2.3.10).
    refuted_by: Vec<String>,
}

/// A dead node a route visited: its commitment, its core and the core's
/// status, as the `enter` line and `--dump-lattice` print them.
type Visit = (Vec<String>, Vec<String>, Option<CoreStatus>, Vec<String>);

/// What a route visits, as `--dump-lattice`'s watch collects it.
struct Collect(Arc<Mutex<Vec<Visit>>>);

impl ein_infer::events::Watch for Collect {
    fn node(&mut self, v: &ein_infer::events::Visited<'_>) {
        if v.kind == "dead" {
            self.0.lock().expect("one watcher").push((
                v.commitment.to_vec(),
                v.core.to_vec(),
                v.core_status,
                v.refuted_by.to_vec(),
            ));
        }
    }
}

/// A recorded justification's parts, as the core search folds them: its
/// premises, the activator a node derived or committed, and the facts its
/// universals read.
fn parts(kb: &Kb, p: &ein_core::Prov) -> Vec<FactId> {
    let mut v = p.premises.to_vec();
    v.extend(ein_core::walks::node_activator(kb, p));
    v.extend(p.universal.iter().copied());
    v.sort_unstable();
    v.dedup();
    v
}

/// For each contradiction of `kb` whose facts `core` derives through the
/// derivations `kb` records, whether it is `target`'s (the one the core was
/// taken over), and the rules each of its facts' **last step** from the core
/// can be, in the store's order ([`Kb::justifications`], the primary first),
/// each once: `(given)` for a fact the core holds, else the rule of every
/// justification whose parts ([`parts`]) the core derives. A closure from
/// the core upward, where the search keeps labels: what a derivation *from
/// the printed core* uses, read without the search.
fn last_steps(
    kb: &Kb,
    terms: &Terms,
    contradictions: &[ein_infer::contradiction::Contradiction],
    core: &[FactId],
    target: Option<FactId>,
) -> Vec<(bool, Vec<Vec<String>>)> {
    use ein_core::ProvKind;
    let frontier = |f: FactId| ein_core::walks::is_frontier(kb, terms, f);
    let rules = |f: FactId| -> Vec<(Option<ein_core::Symbol>, Vec<FactId>)> {
        kb.justifications(f)
            .into_iter()
            .map(|p| terms.provs.get(p))
            .filter(|p| p.kind == ProvKind::Rule)
            .map(|p| (p.rule, parts(kb, p)))
            .collect()
    };
    // Every fact below the contradictions, and who reads it.
    let mut below: BTreeSet<FactId> = BTreeSet::new();
    let mut readers: BTreeMap<FactId, BTreeSet<FactId>> = BTreeMap::new();
    let mut stack: Vec<FactId> = contradictions.iter().flat_map(|c| c.sides()).collect();
    while let Some(f) = stack.pop() {
        if !below.insert(f) || !kb.contains(f) || frontier(f) {
            continue;
        }
        for (_, ps) in rules(f) {
            for q in ps {
                readers.entry(q).or_default().insert(f);
                stack.push(q);
            }
        }
    }
    // What the core derives: itself, every fact no recorded rule step
    // grounds (a dangling id, a fact with no rule justification, a step with
    // no parts), then upward.
    let core_set: BTreeSet<FactId> = core.iter().copied().collect();
    let mut d: BTreeSet<FactId> = BTreeSet::new();
    for &f in &below {
        let free = !kb.contains(f)
            || (!frontier(f) && {
                let r = rules(f);
                r.is_empty() || r.iter().any(|(_, ps)| ps.is_empty())
            });
        if free || core_set.contains(&f) {
            d.insert(f);
        }
    }
    let mut work: Vec<FactId> = d.iter().copied().collect();
    while let Some(x) = work.pop() {
        for &c in readers.get(&x).into_iter().flatten() {
            if d.contains(&c) || frontier(c) {
                continue;
            }
            if rules(c)
                .iter()
                .any(|(_, ps)| ps.iter().all(|q| d.contains(q)))
            {
                d.insert(c);
                work.push(c);
            }
        }
    }
    let name = |r: Option<ein_core::Symbol>| {
        r.map_or_else(|| "(given)".to_string(), |r| terms.sym(r).to_string())
    };
    contradictions
        .iter()
        .filter(|c| c.sides().iter().all(|f| d.contains(f)))
        .map(|c| {
            (
                Some(c.witness()) == target,
                c.sides()
                    .into_iter()
                    .map(|f| {
                        if frontier(f) || !kb.contains(f) {
                            return [kb
                                .primary(f)
                                .map_or("(given)".to_string(), |p| name(terms.provs.get(p).rule))]
                            .into_iter()
                            .collect();
                        }
                        let r = rules(f);
                        if r.is_empty() {
                            return kb
                                .primary(f)
                                .map(|p| name(terms.provs.get(p).rule))
                                .into_iter()
                                .collect();
                        }
                        let mut v: Vec<String> = Vec::new();
                        for (r, _) in r.iter().filter(|(_, ps)| ps.iter().all(|q| d.contains(q))) {
                            let n = name(*r);
                            if !v.contains(&n) {
                                v.push(n);
                            }
                        }
                        v
                    })
                    .collect(),
            )
        })
        .collect()
}

/// The four claims for one printed core, read against `nodes`, the
/// preparation of root that took it: the failures go to `bad`, and a core
/// already read is not read again.
struct Claims {
    rel: String,
    bad: Vec<String>,
    seen: BTreeSet<Seen>,
    cut: usize,
    /// Members of N a run past `--region-limit` met, each printed with the
    /// core taken after the walk (M1f S1f.5.12; `not-kept` since M1e2
    /// S1e2.3.9 until then).
    past_limit: usize,
    /// Cores whose contradiction has a fact the core derives through two
    /// rules, where `refuted by` names the first the store records (M1e2
    /// S1e2.3.10, Q-M1e2.21 (a)).
    two_rules: usize,
}

impl Claims {
    fn hold(&mut self, nodes: &mut Nodes<'_>, s: Stated) -> Option<()> {
        if !self.seen.insert((
            s.set.clone(),
            s.core.clone(),
            s.status,
            s.refuted_by.clone(),
        )) {
            return Some(());
        }
        let rel = &self.rel;
        let Some(status) = s.status else {
            self.bad.push(format!(
                "{rel}: {} {:?} prints the core {:?} with no status",
                s.at, s.set, s.core
            ));
            return Some(());
        };
        self.cut += usize::from(status != CoreStatus::WithinBudget);
        let (core, searched, larger, steps) = nodes
            .with_fixpoint(&s.set, |kb, terms| {
                let contradictions = ein_infer::contradiction::detect(kb, terms);
                let text = |e: &ein_infer::explain::Explanation| -> Vec<String> {
                    let mut v: Vec<String> = e
                        .frontier
                        .iter()
                        .map(|&f| ein_infer::events::sexpr(terms, f))
                        .collect();
                    v.sort();
                    v
                };
                let e = contradiction_frontier(
                    kb,
                    terms,
                    &contradictions,
                    &ExplanationBudget::default(),
                );
                let l = contradiction_frontier(kb, terms, &contradictions, &LARGER);
                let steps = last_steps(kb, terms, &contradictions, &e.frontier, e.target);
                (text(&e), CoreStatus::of(&e), text(&l), steps)
            })
            .ok()?;
        // `refuted by` names one last step per fact of a contradiction the
        // printed core derives (M1e2 S1e2.3.10, P1e2.3's N7): the core's, and
        // of each fact's steps the first the store records (Q-M1e2.21 (a)).
        if !s.core.is_empty() {
            let named = steps.iter().any(|(_, sides)| {
                sides
                    .iter()
                    .all(|opts| opts.iter().any(|r| s.refuted_by.contains(r)))
                    && s.refuted_by
                        .iter()
                        .all(|r| sides.iter().any(|o| o.contains(r)))
                    && s.refuted_by.len() <= sides.len()
            });
            let first: Option<Vec<String>> =
                steps.iter().find(|(won, _)| *won).map(|(_, sides)| {
                    let mut v: Vec<String> =
                        sides.iter().filter_map(|o| o.first().cloned()).collect();
                    v.sort();
                    v.dedup();
                    v
                });
            if !named {
                self.bad.push(format!(
                    "{rel}: {} {:?} prints the core {:?} refuted by {:?}, and the last steps its \
                     derivations end in are {steps:?}",
                    s.at, s.set, s.core, s.refuted_by
                ));
            } else if first.as_ref() != Some(&s.refuted_by) {
                self.bad.push(format!(
                    "{rel}: {} {:?} prints the core {:?} refuted by {:?}, and the first recorded \
                     steps of its contradiction are {first:?} ({steps:?})",
                    s.at, s.set, s.core, s.refuted_by
                ));
            }
            if steps
                .iter()
                .any(|(won, sides)| *won && sides.iter().any(|opts| opts.len() > 1))
            {
                self.two_rules += 1;
            }
        }
        if core != s.core || searched != status {
            self.bad.push(format!(
                "{rel}: {} {:?} prints {:?} as {status:?}, and its search gives {core:?} as {searched:?}",
                s.at, s.set, s.core
            ));
        } else if status == CoreStatus::WithinBudget && larger != core {
            self.bad.push(format!(
                "{rel}: {} {:?} prints {:?} as within its budget, and a budget sixteen times larger gives {larger:?}",
                s.at, s.set, s.core
            ));
        }
        Some(())
    }
}

/// The claims for every core `path`'s surfaces print, each read against the
/// preparation of root that took it: the cube's dead nodes, root's included,
/// where the oracle enumerates it, and the dead nodes describe and
/// `--maximal` visit, against a fresh one, as each run prepares its own; and
/// describe's dead root and N, every member against a fresh one, since the
/// run takes each member's core during its walk or after it against the
/// walk's snapshot, which a fresh preparation's equals (M1f S1f.5.12). A
/// member with no core is a failure, past `--region-limit` too, where until
/// S1f.5.12 a run printed its members `not-kept`. The failures, how many
/// distinct cores were read, how many of them were cut, how many members of N
/// a run past the limit met, and how many cores derive their contradiction
/// through two rules; `None` where the program does not load or is refused.
fn statuses(path: &Path) -> Option<Statuses> {
    let mut claims = Claims {
        rel: path
            .strip_prefix(repo_root())
            .unwrap_or(path)
            .to_string_lossy()
            .to_string(),
        bad: Vec::new(),
        seen: BTreeSet::new(),
        cut: 0,
        past_limit: 0,
        two_rules: 0,
    };
    let (mut kb0, mut terms0, ast0) = loaded(path)?;
    let mut fresh = Nodes::new(&mut kb0, &mut terms0, &ast0, &[]).ok()?;
    // The cube.
    let (mut kb, mut terms, ast) = loaded(path)?;
    if let Ok(Outcome::Region(r)) = region::compute(
        &mut kb,
        &mut terms,
        &ast,
        &region::Options::default(),
        &mut |_| {},
    ) {
        for (mask, node) in r.nodes.iter().enumerate() {
            if r.status[mask] & region::DEAD == 0 {
                continue;
            }
            let set = (0..r.n())
                .filter(|b| mask >> b & 1 == 1)
                .map(|b| r.candidates[b].fact)
                .collect();
            claims.hold(
                &mut fresh,
                Stated {
                    at: "cube",
                    set,
                    core: node.core.clone(),
                    status: node.core_status,
                    refuted_by: node.refuted_by.clone(),
                },
            )?;
        }
    }
    // `--maximal`, then describe, each with what it visits.
    for which in [Which::Maximal, Which::All] {
        let (mut kb, mut terms, ast) = loaded(path)?;
        let opts = explore::Options::new(Mode {
            which,
            ask: Ask::Describe,
        });
        let visits = Arc::new(Mutex::new(Vec::new()));
        let mut events =
            ein_infer::events::Events::off().watched(Box::new(Collect(visits.clone())));
        let a = explore::run_with(&mut kb, &mut terms, &ast, &opts, &mut events).ok()?;
        let by_text: BTreeMap<&str, FactId> = a
            .candidates
            .iter()
            .map(|c| (c.text.as_str(), c.fact))
            .collect();
        let at = if which == Which::All {
            "describe"
        } else {
            "maximal"
        };
        let visited: Vec<Visit> = visits.lock().expect("done").drain(..).collect();
        for (commitment, core, status, refuted_by) in visited {
            let set = commitment.iter().map(|c| by_text[c.as_str()]).collect();
            claims.hold(
                &mut fresh,
                Stated {
                    at,
                    set,
                    core,
                    status,
                    refuted_by,
                },
            )?;
        }
        if which != Which::All {
            continue;
        }
        let Some(d) = a.described.as_ref() else {
            continue;
        };
        if !d.root.alive {
            claims.hold(
                &mut fresh,
                Stated {
                    at: "root",
                    set: Vec::new(),
                    core: d.root.core.clone(),
                    status: d.root.core_status,
                    refuted_by: d.root.refuted_by.clone(),
                },
            )?;
        }
        let stopped = matches!(a.stopped, Some(explore::Stopped::Limit { .. }));
        for x in &d.dead {
            let set: Vec<FactId> = x
                .set
                .iter()
                .map(|&b| a.candidates[b as usize].fact)
                .collect();
            if x.core_status.is_none() || (x.core.is_empty() && !x.set.is_empty()) {
                // Every member is printed with its core since M1f S1f.5.12,
                // past `--region-limit` too.
                claims.bad.push(format!(
                    "{}: N {:?} carries no core ({:?}), on a run {} --region-limit",
                    claims.rel,
                    set,
                    x.core_status,
                    if stopped {
                        "stopped at"
                    } else {
                        "not stopped at"
                    }
                ));
                continue;
            }
            if stopped {
                claims.past_limit += 1;
            }
            claims.hold(
                &mut fresh,
                Stated {
                    at: "N",
                    set,
                    core: x.core.clone(),
                    status: x.core_status,
                    refuted_by: x.refuted_by.clone(),
                },
            )?;
        }
    }
    Some((
        claims.bad,
        claims.seen.len(),
        claims.cut,
        claims.past_limit,
        claims.two_rules,
    ))
}

/// **No core is printed unqualified from a cut search** — M1e2
/// [S1e2.2.23](../../../../plans/m1e2_correctness/p1e2.2_wrong_answers/s1e2.2.23_core_search_cut.md)'s
/// claim, W23's, over the corpus and the register's witnesses.
///
/// Every core a surface prints, with the status printed beside it
/// ([`stated`]): the cube's dead nodes, root's among them, `ein oracle
/// --dump`'s; describe's dead root and N, the summary's, the read-out's and
/// `--dump-answer`'s, every member's as the run takes it, after the walk
/// where the walk took none (M1f S1f.5.12); and every dead node describe and
/// `--maximal` visit, `--dump-lattice`'s and `--events`'. For each distinct
/// one, three things:
///
/// 1. **it has a status**: a surface that prints a core prints whether its
///    search was cut;
/// 2. **the status is its search's**: root plus the commitment, saturated
///    again and searched at the default budget, gives the same core and the
///    same status, the cap it stopped at included;
/// 3. **a core printed as within its budget is no cut core**: a budget
///    sixteen times larger on every cap gives the same core.
///
/// The cut cores the per-commit half reads are the witness's and its
/// control's; the slow entries' (`zebra.ein` at the rounds, the `zebra2`
/// family at the environments) are the nightly's. At S1e2.2.23's T1 no
/// surface printed a status (2026-11-05).
///
/// **And `refuted by` is the core's derivation's last step** (M1e2
/// S1e2.3.10, N7): beside each such core, the rules printed are, for each
/// fact of the contradiction the core was taken over, the first step the
/// store records that the core derives ([`last_steps`], Q-M1e2.21 (a)); and
/// at least they name one step per fact of some contradiction the core
/// derives, the acceptance's claim, reported apart. The cores whose
/// contradiction the core derives through two rules are counted, 5 041 of
/// 230 154 per commit (2026-11-06).
///
/// **And every member of N is printed with its core, past `--region-limit`
/// too** (M1f S1f.5.12, the user's ruling of 2026-11-08 on P-15, (c)): the
/// run takes each member's core after its walk where the walk took none, so
/// no member carries no core and none is `not-kept`, M1e2 S1e2.3.9's word for
/// a member a run past the limit had kept as its two bits. The members of N
/// runs past the limit met are counted, each held as any other core.
#[test]
fn no_core_is_printed_unqualified_from_a_cut_search() {
    let files = files();
    let next = std::sync::atomic::AtomicUsize::new(0);
    let results: Mutex<Vec<(usize, Option<Statuses>)>> = Mutex::new(Vec::new());
    std::thread::scope(|scope| {
        for _ in 0..jobs() {
            scope.spawn(|| {
                loop {
                    let i = next.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                    let Some(path) = files.get(i) else { break };
                    let got = statuses(path);
                    results.lock().expect("no checker panicked").push((i, got));
                }
            });
        }
    });
    let mut results = results.into_inner().expect("no checker panicked");
    results.sort_by_key(|(i, _)| *i);
    let mut failures: Vec<String> = Vec::new();
    let (mut programs, mut cores, mut cut, mut past_limit, mut two_rules) =
        (0usize, 0usize, 0usize, 0usize, 0usize);
    for (_, got) in results {
        let Some((bad, n, c, k, t)) = got else {
            continue;
        };
        programs += 1;
        cores += n;
        cut += c;
        past_limit += k;
        two_rules += t;
        failures.extend(bad);
    }
    assert!(failures.is_empty(), "{}", failures.join("\n"));
    assert!(programs >= 100, "{programs} programs read");
    // The ruling has witnesses: 5 041 per commit (2026-11-06, S1e2.3.10).
    assert!(
        cores >= 100_000 && cut >= 10 && past_limit >= 1 && two_rules >= 100,
        "{cores} cores, {cut} of them cut, {past_limit} members of N past the limit, \
         {two_rules} cores through two rules"
    );
    eprintln!(
        "{programs} programs, {cores} distinct cores, {cut} of them cut, {past_limit} members of N \
         past --region-limit, {two_rules} whose contradiction the core derives through two rules"
    );
}

// ── A rule with no premise: the empty core — M1f S1f.5.9 ────────────────────

/// **A rule with no premise refutes root with the empty core** — M1f S1f.5.9
/// (W31), on the user's ruling of 2026-11-07 on P1f.5's F12: `:match ()` is
/// the empty conjunction, true, so `(rule r () :match () :assert (false))`
/// fires once at root and root is dead. Its refutation rests on no fact, so
/// its core is `[]`, the empty set (Q-M1e2.27), the first in Ein, taken
/// inside the budget, and `refuted by` names `r`, on describe and on the cube
/// alike. [`every_core_names_its_commitment_and_derives_its_contradiction`]
/// skips a dead root, and [`no_core_is_printed_unqualified_from_a_cut_search`]
/// holds its status and rules with every other's; this test holds the core
/// itself. Until S1f.5.9 the rule never fired and root was alive.
#[test]
fn a_rule_with_no_premise_refutes_root_with_the_empty_core() {
    let path = repo_root().join("examples/ein-bugs/wrong-answers/empty-match-false.ein");
    let (mut kb, mut terms, ast) = loaded(&path).expect("the witness loads");
    let opts = explore::Options::new(Mode {
        which: Which::All,
        ask: Ask::Describe,
    });
    let a = explore::run(&mut kb, &mut terms, &ast, &opts).expect("it answers");
    assert!(a.root_dead, "root is dead");
    let root = &a.described.as_ref().expect("describe").root;
    assert!(!root.alive);
    assert_eq!(
        root.core,
        Vec::<String>::new(),
        "root's core is the empty set"
    );
    assert_eq!(root.core_status, Some(CoreStatus::WithinBudget));
    assert_eq!(root.refuted_by, ["r"]);
    // The cube: the oracle's root node says the same.
    let (mut kb, mut terms, ast) = loaded(&path).expect("the witness loads");
    let Ok(Outcome::Region(r)) = region::compute(
        &mut kb,
        &mut terms,
        &ast,
        &region::Options::default(),
        &mut |_| {},
    ) else {
        panic!("the oracle answers");
    };
    assert!(r.root_dead);
    assert_eq!(r.nodes[0].core, Vec::<String>::new());
    assert_eq!(r.nodes[0].core_status, Some(CoreStatus::WithinBudget));
    assert_eq!(r.nodes[0].refuted_by, ["r"]);
}
