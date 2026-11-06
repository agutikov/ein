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
//! **Which cores.** Every surface prints a core one of two functions takes,
//! the node function a route saturates a node with and
//! `region::Nodes::refutation`, which `--dump-answer` and `--trace` call for a
//! member of N the state route did not saturate; both explain the node's
//! contradictions the same way. So the test reads both: the oracle's cube,
//! every dead node, where the oracle enumerates it, and describe's N, each
//! member's core through `refutation`. Every corpus file and register witness
//! the loader accepts and *forbid* does not refuse; the slow entries under
//! `EIN_CORPUS_SLOW=1`, the nightly's.
//!
//! **What it does not hold.** A run stopped past `--region-limit` keeps the
//! members of N it met as their two bits, and prints each with no core and
//! the status `not-kept` (M1e2 S1e2.3.9, P1e2.3's N6; until then with an
//! empty core and no status): this test takes each member's core itself.
//! [`no_core_is_printed_unqualified_from_a_cut_search`] holds that such a
//! member carries no core and appears on a run that stopped alone.
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

/// [`statuses`]' failures and counts: cores read, cut, and not kept.
type Statuses = (Vec<String>, usize, usize, usize);

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
/// where the oracle enumerates it, and N's members through `refutation`, as
/// `--dump-answer` takes them. The failures and how many cores were read, the
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
    let sets: Vec<Vec<FactId>> = a
        .described
        .as_ref()
        .map(|d| {
            d.dead
                .iter()
                .filter(|x| !x.set.is_empty())
                .map(|x| {
                    x.set
                        .iter()
                        .map(|&b| a.candidates[b as usize].fact)
                        .collect()
                })
                .collect()
        })
        .unwrap_or_default();
    if !sets.is_empty() {
        let mut nodes = Nodes::new(&mut kb, &mut terms, &ast, &a.candidates).ok()?;
        let mut n: Vec<Reported> = Vec::with_capacity(sets.len());
        for set in sets {
            let core = nodes.refutation(&set).ok()?.core;
            n.push(Reported { set, core });
        }
        bad.extend(claims(&rel, &mut nodes, &n)?);
        read += n.len();
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

/// One core a surface prints, with the status printed beside it.
struct Stated {
    /// Where it was read: `cube`, `root`, `N`, or the mode whose visit took it.
    at: &'static str,
    set: Vec<FactId>,
    core: Vec<String>,
    status: Option<CoreStatus>,
}

/// A dead node a route visited: its commitment, its core and the core's
/// status, as the `enter` line and `--dump-lattice` print them.
type Visit = (Vec<String>, Vec<String>, Option<CoreStatus>);

/// What a route visits, as `--dump-lattice`'s watch collects it.
struct Collect(Arc<Mutex<Vec<Visit>>>);

impl ein_infer::events::Watch for Collect {
    fn node(&mut self, v: &ein_infer::events::Visited<'_>) {
        if v.kind == "dead" {
            self.0.lock().expect("one watcher").push((
                v.commitment.to_vec(),
                v.core.to_vec(),
                v.core_status,
            ));
        }
    }
}

/// The three claims for one printed core, read against `nodes`, the
/// preparation of root that took it: the failures go to `bad`, and a core
/// already read is not read again.
struct Claims {
    rel: String,
    bad: Vec<String>,
    seen: BTreeSet<(Vec<FactId>, Vec<String>, Option<CoreStatus>)>,
    cut: usize,
    /// Members of N printed with no core, `not-kept` (M1e2 S1e2.3.9).
    not_kept: usize,
}

impl Claims {
    fn hold(&mut self, nodes: &mut Nodes<'_>, s: Stated) -> Option<()> {
        if !self.seen.insert((s.set.clone(), s.core.clone(), s.status)) {
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
        let (core, searched, larger) = nodes
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
                (text(&e), CoreStatus::of(&e), text(&l))
            })
            .ok()?;
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
/// describe's dead root and N, a member the route kept against a fresh one,
/// and a member the state route did not saturate against the one
/// `--dump-answer` and `--trace` take its core through, `refutation` after
/// the run. A member a run past `--region-limit` kept as its two bits is
/// held to carry no core, on a run that stopped there (M1e2 S1e2.3.9). The
/// failures, how many distinct cores were read, how many of them were cut,
/// and how many members were not kept; `None` where the program does not load
/// or is refused.
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
        not_kept: 0,
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
        for (commitment, core, status) in visited {
            let set = commitment.iter().map(|c| by_text[c.as_str()]).collect();
            claims.hold(
                &mut fresh,
                Stated {
                    at,
                    set,
                    core,
                    status,
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
                },
            )?;
        }
        let mut after = Nodes::new(&mut kb, &mut terms, &ast, &a.candidates).ok()?;
        for x in &d.dead {
            let set: Vec<FactId> = x
                .set
                .iter()
                .map(|&b| a.candidates[b as usize].fact)
                .collect();
            if x.core_status == Some(CoreStatus::NotKept) {
                // Kept as its two bits by a run past `--region-limit`, and
                // printed with no core (M1e2 S1e2.3.9): it carries none, and
                // only a run that stopped there has one.
                let stopped = matches!(a.stopped, Some(explore::Stopped::Limit { .. }));
                if d.exhausted || !stopped || !x.core.is_empty() || !x.refuted_by.is_empty() {
                    claims.bad.push(format!(
                        "{}: N {:?} is not kept, with the core {:?} and the rules {:?}, on a run \
                         {} --region-limit",
                        claims.rel,
                        set,
                        x.core,
                        x.refuted_by,
                        if stopped {
                            "stopped at"
                        } else {
                            "not stopped at"
                        }
                    ));
                }
                claims.not_kept += 1;
                continue;
            }
            if x.core.is_empty() && !x.set.is_empty() {
                // What `--dump-answer` and `--trace` print for it.
                let why = after.refutation(&set).ok()?;
                claims.hold(
                    &mut after,
                    Stated {
                        at: "N",
                        set,
                        core: why.core,
                        status: why.core_status,
                    },
                )?;
            } else {
                claims.hold(
                    &mut fresh,
                    Stated {
                        at: "N",
                        set,
                        core: x.core.clone(),
                        status: x.core_status,
                    },
                )?;
            }
        }
    }
    Some((claims.bad, claims.seen.len(), claims.cut, claims.not_kept))
}

/// **No core is printed unqualified from a cut search** — M1e2
/// [S1e2.2.23](../../../../plans/m1e2_correctness/p1e2.2_wrong_answers/s1e2.2.23_core_search_cut.md)'s
/// claim, W23's, over the corpus and the register's witnesses.
///
/// Every core a surface prints, with the status printed beside it
/// ([`stated`]): the cube's dead nodes, root's among them, `ein oracle
/// --dump`'s; describe's dead root and N, the summary's, the read-out's and
/// `--dump-answer`'s, a member the state route did not saturate taken
/// through `refutation` as `--dump-answer` and `--trace` take it; and every
/// dead node describe and `--maximal` visit, `--dump-lattice`'s and
/// `--events`'. For each distinct one, three things:
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
/// **And a member of N printed with no core says why** (M1e2 S1e2.3.9, N6):
/// its status is `not-kept`, it carries no core and no rule, and the run that
/// printed it stopped at `--region-limit`. Until S1e2.3.9 such a member had
/// an empty core and no status, which this test read as a core the dump
/// took through `refutation`, as the state route's are.
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
    let (mut programs, mut cores, mut cut, mut not_kept) = (0usize, 0usize, 0usize, 0usize);
    for (_, got) in results {
        let Some((bad, n, c, k)) = got else { continue };
        programs += 1;
        cores += n;
        cut += c;
        not_kept += k;
        failures.extend(bad);
    }
    assert!(failures.is_empty(), "{}", failures.join("\n"));
    assert!(programs >= 100, "{programs} programs read");
    assert!(
        cores >= 100_000 && cut >= 10 && not_kept >= 1,
        "{cores} cores, {cut} of them cut, {not_kept} members not kept"
    );
    eprintln!(
        "{programs} programs, {cores} distinct cores, {cut} of them cut, {not_kept} members of N \
         not kept"
    );
}
