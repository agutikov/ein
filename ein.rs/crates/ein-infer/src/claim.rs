//! A `(solve … :expect …)` claim, checked against R — M1e2
//! [S1e2.1a.4](../../../../plans/m1e2_correctness/p1e2.1a_the_solution_region/s1e2.1a.4_the_language.md).
//!
//! The shape is [`ein_ir::claim`]'s and the loader has validated it. This is
//! the comparison, and it never runs the walk: R comes from route 1 on a
//! monotone program and from the cube on a flagged one, the two routes
//! `ein solve` describes R by ([`crate::explore`]). Each state's facts are its
//! fixpoint's, root's included, read by the oracle's own node.
//!
//! The rules (`region_examples.md` § 1, accepted 2026-10-16):
//!
//! - every relation the claim names positively is closed, across the whole
//!   claim, and a listed `(not …)` is checked for presence;
//! - the `(model …)`s are R's states, one to one;
//! - `(merged …)` is all or nothing: with one, every state reached by more
//!   than one commitment is listed, with exactly its minimal commitments;
//! - `(owing …)` says R is empty and lists the alive, owing nodes with no
//!   alive node one fact up;
//! - `(false)` says every node is dead, and `(region)` that R is empty.

use std::collections::{BTreeMap, BTreeSet};
use std::time::Instant;

use ein_core::{FactId, Kb, Terms};
use ein_ir::Ast;
use ein_ir::claim::{Claim, Region as Stated};
use ein_ir::expect::Model;
use rustc_hash::FxHashSet;

use crate::events::sexpr;
use crate::expect::{Outcome, Report};
use crate::region::{self, Axes, DEAD, OWING, fan_out, pool_of, prepare};
use crate::solve::SolveError;
use crate::{downset, region::Candidate};

/// How a claim is checked: the budgets of the two routes.
#[derive(Clone, Copy, Debug)]
pub struct Options {
    /// Route 1's, in facts beyond root.
    pub budget: u64,
    /// The largest `n` the cube is computed for, on a flagged program.
    pub limit: u32,
    pub jobs: usize,
}

impl Default for Options {
    fn default() -> Self {
        Options {
            budget: downset::DEFAULT_BUDGET,
            limit: region::DEFAULT_LIMIT,
            jobs: 1,
        }
    }
}

/// What the check found, with what it ran.
#[derive(Debug)]
pub struct Checked {
    pub report: Report,
    /// `route 1` or `route 2`; `None` when no route described R.
    pub route: Option<&'static str>,
    pub monotone: bool,
    pub k_s: u64,
    pub k_c: u64,
}

/// One state of R: a commitment that reaches it, and every commitment of R
/// that does.
struct State {
    rep: Vec<u32>,
    sets: Vec<Vec<u32>>,
}

/// R, as the claim needs it.
struct Lattice {
    candidates: Vec<Candidate>,
    route: &'static str,
    monotone: bool,
    any_alive: bool,
    states: Vec<State>,
    /// Alive, owing, and no alive node one fact up.
    frontier: Vec<Vec<u32>>,
}

/// Check `claim` against the R of `root`, a **freshly loaded, unsaturated**
/// program, as [`crate::explore::run`] asks.
pub fn check(
    root: &mut Kb,
    terms: &mut Terms,
    ast: &Ast,
    claim: &Claim,
    opts: &Options,
) -> Result<Checked, SolveError> {
    let lattice = match lattice(root, terms, ast, opts)? {
        Ok(l) => l,
        Err((why, monotone)) => {
            return Ok(Checked {
                report: Report {
                    outcome: Outcome::NotChecked,
                    lines: vec![why],
                },
                route: None,
                monotone,
                k_s: 0,
                k_c: 0,
            });
        }
    };
    let k_s = lattice.states.len() as u64;
    let k_c = lattice.states.iter().map(|s| s.sets.len() as u64).sum();
    let report = compare(root, terms, ast, claim, &lattice, opts)?;
    Ok(Checked {
        report,
        route: Some(lattice.route),
        monotone: lattice.monotone,
        k_s,
        k_c,
    })
}

/// R, by route 1 on a monotone program and by the cube otherwise — or why no
/// route described it, with the static verdict.
fn lattice(
    root: &mut Kb,
    terms: &mut Terms,
    ast: &Ast,
    opts: &Options,
) -> Result<Result<Lattice, (String, bool)>, SolveError> {
    let t0 = Instant::now();
    let prepared = prepare(root, terms, ast)?;
    let monotone = prepared.monotonicity.is_monotone();
    let n = prepared.candidates.len();
    if monotone {
        let d = downset::compute_prepared(
            root,
            terms,
            ast,
            prepared,
            t0,
            &downset::Options {
                budget: opts.budget,
                max_visits: u64::MAX,
                jobs: opts.jobs,
            },
        )?;
        match &d.declined {
            None => return Ok(Ok(of_downset(d))),
            Some(downset::Declined::Budget { layer, .. }) => {
                return Ok(Err((
                    format!(
                        "R was not described: route 1 stopped at its budget of {} facts, \
                         before layer {layer}",
                        opts.budget
                    ),
                    true,
                )));
            }
            // The cube contradicted the static verdict: take the cube.
            Some(_) => {}
        }
    }
    if n > opts.limit as usize {
        return Ok(Err((
            format!(
                "R was not described: the program is not monotone and n = {n} is past \
                 the oracle's limit of {}",
                opts.limit
            ),
            monotone,
        )));
    }
    // A fresh preparation: the one above went to route 1, or was never used.
    let prepared = prepare(root, terms, ast)?;
    let outcome = region::compute_prepared(
        root,
        terms,
        ast,
        prepared,
        t0,
        &region::Options {
            limit: opts.limit,
            keep_states: false,
            jobs: opts.jobs,
        },
        &mut |_| {},
    )?;
    Ok(match outcome {
        region::Outcome::Region(r) => Ok(of_cube(*r)),
        region::Outcome::Refused { n, limit, .. } => Err((
            format!("R was not described: n = {n} is past the oracle's limit of {limit}"),
            monotone,
        )),
    })
}

/// R, printed as the claim that states it — the form `ein solve` prints and a
/// program reads back (ruling 1 of 2026-10-16: one form, both directions). It
/// names the candidates' relations, so every state is told apart. `None` when
/// no route described R.
///
/// `(false)` when no node is alive; `(region (owing …))` when R is empty and
/// alive nodes owe; otherwise every state, then a `(merged …)` for every state
/// more than one commitment reaches.
pub fn print(
    root: &mut Kb,
    terms: &mut Terms,
    ast: &Ast,
    opts: &Options,
) -> Result<Option<String>, SolveError> {
    let Ok(l) = lattice(root, terms, ast, opts)? else {
        return Ok(None);
    };
    if !l.any_alive {
        return Ok(Some("(false)".to_string()));
    }
    let named: BTreeSet<String> = l
        .candidates
        .iter()
        .map(|c| relation_of(&c.text).to_string())
        .collect();
    let model = |s: &Seen| describe(s, &named);
    if l.states.is_empty() {
        let frontier = fixpoints(root, terms, ast, &l.candidates, &l.frontier, opts)?;
        let mut items: Vec<String> = frontier.iter().map(model).collect();
        items.sort();
        return Ok(Some(format!("(region (owing {}))", items.join(" "))));
    }
    let reps: Vec<Vec<u32>> = l.states.iter().map(|s| s.rep.clone()).collect();
    let states = fixpoints(root, terms, ast, &l.candidates, &reps, opts)?;
    let mut items: Vec<String> = states.iter().map(model).collect();
    items.sort();
    let mut merged: Vec<String> = Vec::new();
    for (st, seen) in l.states.iter().zip(&states) {
        if st.sets.len() < 2 {
            continue;
        }
        let mut gens: Vec<String> = minimal_generators(st)
            .into_iter()
            .map(|g| {
                let mut facts: Vec<String> = g
                    .iter()
                    .map(|&b| l.candidates[b as usize].text.clone())
                    .collect();
                facts.sort();
                if facts.is_empty() {
                    "(model)".to_string()
                } else {
                    format!("(model {})", facts.join(" "))
                }
            })
            .collect();
        gens.sort();
        let from = if gens.len() == 1 {
            gens.remove(0)
        } else {
            format!("(or {})", gens.join(" "))
        };
        merged.push(format!("(merged {} {from})", model(seen)));
    }
    merged.sort();
    items.extend(merged);
    Ok(Some(format!("(region {})", items.join(" "))))
}

/// A state's minimal generating commitments: those of its class with no proper
/// subset in the class.
fn minimal_generators(st: &State) -> Vec<&Vec<u32>> {
    st.sets
        .iter()
        .filter(|s| {
            !st.sets
                .iter()
                .any(|o| o.len() < s.len() && o.iter().all(|b| s.contains(b)))
        })
        .collect()
}

fn of_downset(d: downset::DownSet) -> Lattice {
    let alive: FxHashSet<&[u32]> = d
        .points
        .iter()
        .filter(|p| p.status & DEAD == 0)
        .map(|p| &p.set[..])
        .collect();
    let n = d.candidates.len() as u32;
    let mut by_class: BTreeMap<u32, State> = BTreeMap::new();
    for p in d.points.iter().filter(|p| p.in_r()) {
        by_class
            .entry(p.class)
            .or_insert_with(|| State {
                rep: p.closure.to_vec(),
                sets: Vec::new(),
            })
            .sets
            .push(p.set.to_vec());
    }
    let frontier = d
        .points
        .iter()
        .filter(|p| p.status & DEAD == 0 && p.status & OWING != 0)
        .filter(|p| {
            !(0..n).any(|b| {
                p.set.binary_search(&b).is_err() && {
                    let mut s = p.set.to_vec();
                    let at = s.partition_point(|&x| x < b);
                    s.insert(at, b);
                    alive.contains(&s[..])
                }
            })
        })
        .map(|p| p.set.to_vec())
        .collect();
    Lattice {
        any_alive: !alive.is_empty(),
        candidates: d.candidates,
        route: "route 1",
        monotone: true,
        states: by_class.into_values().collect(),
        frontier,
    }
}

fn of_cube(r: region::Region) -> Lattice {
    let n = r.n();
    let set_of = |m: u64| -> Vec<u32> { (0..n as u32).filter(|&b| m >> b & 1 == 1).collect() };
    let mut by_class: BTreeMap<u64, State> = BTreeMap::new();
    for m in 0..r.status.len() as u64 {
        if !r.in_r(m) {
            continue;
        }
        let class = r.nodes[m as usize].class;
        by_class
            .entry(class)
            .or_insert_with(|| State {
                rep: set_of(class),
                sets: Vec::new(),
            })
            .sets
            .push(set_of(m));
    }
    let alive = |m: u64| r.status[m as usize] & DEAD == 0;
    let frontier = (0..r.status.len() as u64)
        .filter(|&m| alive(m) && r.status[m as usize] & OWING != 0)
        .filter(|&m| !(0..n).any(|b| m >> b & 1 == 0 && alive(m | 1 << b)))
        .map(set_of)
        .collect();
    Lattice {
        any_alive: (0..r.status.len() as u64).any(alive),
        route: "route 2",
        monotone: r.monotonicity.is_monotone(),
        candidates: r.candidates,
        states: by_class.into_values().collect(),
        frontier,
    }
}

/// Every listed fact as the checker reads it.
struct Want {
    closes: BTreeMap<String, BTreeSet<String>>,
    negatives: Vec<String>,
}

fn want(ast: &Ast, m: &Model) -> Want {
    let mut closes: BTreeMap<String, BTreeSet<String>> = BTreeMap::new();
    let mut negatives = Vec::new();
    for &f in &m.facts {
        let Ok(f) = ein_ir::expect::fact(ast, f) else {
            continue;
        };
        if f.negated {
            negatives.push(f.rendered);
        } else {
            closes
                .entry(f.relation.to_string())
                .or_default()
                .insert(f.rendered);
        }
    }
    Want { closes, negatives }
}

/// A state's fixpoint, as the comparison reads it.
struct Seen {
    all: BTreeSet<String>,
    by_rel: BTreeMap<String, BTreeSet<String>>,
}

fn relation_of(fact: &str) -> &str {
    fact.trim_start_matches('(')
        .split([' ', ')'])
        .next()
        .unwrap_or("")
}

fn seen(facts: BTreeSet<String>) -> Seen {
    let mut by_rel: BTreeMap<String, BTreeSet<String>> = BTreeMap::new();
    for f in &facts {
        let r = relation_of(f);
        if r != "not" {
            by_rel.entry(r.to_string()).or_default().insert(f.clone());
        }
    }
    Seen { all: facts, by_rel }
}

/// `w` against `s`, every relation in `closed` at its complete extent.
fn fits(w: &Want, s: &Seen, closed: &BTreeSet<String>) -> bool {
    let none = BTreeSet::new();
    closed
        .iter()
        .all(|rel| w.closes.get(rel).unwrap_or(&none) == s.by_rel.get(rel).unwrap_or(&none))
        && w.negatives.iter().all(|n| s.all.contains(n))
}

/// A perfect matching between listed models and states, or `None`.
fn matching(wants: &[Want], states: &[Seen], closed: &BTreeSet<String>) -> Option<Vec<usize>> {
    if wants.len() != states.len() {
        return None;
    }
    let mut owner: Vec<Option<usize>> = vec![None; states.len()];
    fn augment(
        i: usize,
        wants: &[Want],
        states: &[Seen],
        closed: &BTreeSet<String>,
        seen: &mut [bool],
        owner: &mut [Option<usize>],
    ) -> bool {
        for j in 0..states.len() {
            if seen[j] || !fits(&wants[i], &states[j], closed) {
                continue;
            }
            seen[j] = true;
            if owner[j].is_none_or(|k| augment(k, wants, states, closed, seen, owner)) {
                owner[j] = Some(i);
                return true;
            }
        }
        false
    }
    for i in 0..wants.len() {
        let mut seen = vec![false; states.len()];
        if !augment(i, wants, states, closed, &mut seen, &mut owner) {
            return None;
        }
    }
    // `owner[j]` is the model state j matched; the inverse is what the
    // caller wants.
    let mut of_model = vec![0; wants.len()];
    for (j, o) in owner.iter().enumerate() {
        if let Some(i) = o {
            of_model[*i] = j;
        }
    }
    Some(of_model)
}

/// The fixpoints of `sets`, root's facts included, rendered.
fn fixpoints(
    root: &mut Kb,
    terms: &mut Terms,
    ast: &Ast,
    candidates: &[Candidate],
    sets: &[Vec<u32>],
    opts: &Options,
) -> Result<Vec<Seen>, SolveError> {
    if sets.is_empty() {
        return Ok(Vec::new());
    }
    let prepared = prepare(root, terms, ast)?;
    let axes = Axes::new(root, candidates, prepared.hypothesised.clone(), true);
    let facts: Vec<Vec<FactId>> = sets
        .iter()
        .map(|s| s.iter().map(|&b| candidates[b as usize].fact).collect())
        .collect();
    let pool = pool_of(opts.jobs);
    let outs = fan_out(
        pool.as_ref(),
        root,
        &prepared.snapshot,
        terms,
        ast,
        &prepared.memo,
        &axes,
        &facts,
    )?;
    let root_text: BTreeSet<String> = root.facts().map(|f| sexpr(terms, f)).collect();
    Ok(outs
        .into_iter()
        .map(|o| {
            let mut all = root_text.clone();
            all.extend(o.delta);
            seen(all)
        })
        .collect())
}

fn describe(s: &Seen, closed: &BTreeSet<String>) -> String {
    let facts: Vec<&String> = closed
        .iter()
        .filter_map(|r| s.by_rel.get(r))
        .flatten()
        .collect();
    if facts.is_empty() {
        "(model)".to_string()
    } else {
        format!(
            "(model {})",
            facts
                .iter()
                .map(|f| f.as_str())
                .collect::<Vec<_>>()
                .join(" ")
        )
    }
}

fn compare(
    root: &mut Kb,
    terms: &mut Terms,
    ast: &Ast,
    claim: &Claim,
    l: &Lattice,
    opts: &Options,
) -> Result<Report, SolveError> {
    let k_s = l.states.len();
    let failed = |lines: Vec<String>| Report {
        outcome: Outcome::Failed,
        lines,
    };
    let held = Report {
        outcome: Outcome::Held,
        lines: Vec::new(),
    };
    let stated = match claim {
        Claim::False => {
            return Ok(if !l.any_alive {
                held
            } else if k_s > 0 {
                failed(vec![format!(
                    "(false) says every node is dead, and R has {k_s} state{}",
                    if k_s == 1 { "" } else { "s" }
                )])
            } else {
                failed(vec![
                    "(false) says every node is dead; R is empty, but alive nodes owe — \
                     that is `(region (owing …))`"
                        .to_string(),
                ])
            });
        }
        Claim::Region(r) => r,
    };
    // The relations the claim closes: every one a model, a `<to>` or an
    // owing model names positively.
    let mut closed: BTreeSet<String> = BTreeSet::new();
    let models_named = stated
        .models
        .iter()
        .chain(stated.merged.iter().map(|m| &m.to))
        .chain(stated.owing.iter());
    for m in models_named {
        closed.extend(want(ast, m).closes.into_keys());
    }
    if !stated.owing.is_empty() || stated.models.is_empty() {
        return owing_or_empty(root, terms, ast, stated, l, &closed, opts);
    }
    if k_s == 0 {
        return Ok(failed(vec![format!(
            "the claim lists {} state{} and R is empty",
            stated.models.len(),
            if stated.models.len() == 1 { "" } else { "s" }
        )]));
    }
    let reps: Vec<Vec<u32>> = l.states.iter().map(|s| s.rep.clone()).collect();
    let states = fixpoints(root, terms, ast, &l.candidates, &reps, opts)?;
    let wants: Vec<Want> = stated.models.iter().map(|m| want(ast, m)).collect();
    let Some(of_model) = matching(&wants, &states, &closed) else {
        let mut lines = vec![format!(
            "the claim lists {} state{} and R has {k_s}",
            wants.len(),
            if wants.len() == 1 { "" } else { "s" }
        )];
        if wants.len() == k_s {
            lines[0] = "R's states and the listed models do not match one to one".to_string();
        }
        for s in &states {
            if !wants.iter().any(|w| fits(w, s, &closed)) {
                lines.push(format!(
                    "  a state of R no model lists: {}",
                    describe(s, &closed)
                ));
            }
        }
        for (w, m) in wants.iter().zip(&stated.models) {
            if !states.iter().any(|s| fits(w, s, &closed)) {
                let text: Vec<String> = m
                    .facts
                    .iter()
                    .map(|&f| ein_ir::expect::render(ast, f))
                    .collect();
                lines.push(format!(
                    "  a listed model no state of R is: (model {})",
                    text.join(" ")
                ));
            }
        }
        return Ok(failed(lines));
    };
    if stated.merged.is_empty() {
        return Ok(held);
    }
    // `(merged …)`, all or nothing: every state with more than one commitment,
    // with exactly its minimal ones.
    let text = |set: &[u32]| -> BTreeSet<String> {
        set.iter()
            .map(|&b| l.candidates[b as usize].text.clone())
            .collect()
    };
    let mut lines = Vec::new();
    let mut covered = vec![false; k_s];
    for m in &stated.merged {
        let w = want(ast, &m.to);
        let Some(i) = (0..stated.models.len()).find(|&i| fits(&w, &states[of_model[i]], &closed))
        else {
            continue; // the loader holds every `<to>` to a listed model
        };
        let st = &l.states[of_model[i]];
        covered[of_model[i]] = true;
        let minimal: BTreeSet<BTreeSet<String>> = minimal_generators(st)
            .into_iter()
            .map(|s| text(s))
            .collect();
        let from: BTreeSet<BTreeSet<String>> = m
            .from
            .iter()
            .map(|g| {
                g.facts
                    .iter()
                    .map(|&f| ein_ir::expect::render(ast, f))
                    .collect()
            })
            .collect();
        if st.sets.len() < 2 {
            lines.push(format!(
                "a (merged …) names {}, which one commitment alone reaches",
                describe(&states[of_model[i]], &closed)
            ));
        } else if from != minimal {
            lines.push(format!(
                "{} is reached by {} commitments, and its minimal ones are not the listed ones",
                describe(&states[of_model[i]], &closed),
                st.sets.len()
            ));
        }
    }
    for (j, st) in l.states.iter().enumerate() {
        if st.sets.len() > 1 && !covered[j] {
            lines.push(format!(
                "{} is reached by {} commitments and no (merged …) lists it",
                describe(&states[j], &closed),
                st.sets.len()
            ));
        }
    }
    Ok(if lines.is_empty() {
        held
    } else {
        failed(lines)
    })
}

fn owing_or_empty(
    root: &mut Kb,
    terms: &mut Terms,
    ast: &Ast,
    stated: &Stated,
    l: &Lattice,
    closed: &BTreeSet<String>,
    opts: &Options,
) -> Result<Report, SolveError> {
    let k_s = l.states.len();
    let failed = |lines: Vec<String>| Report {
        outcome: Outcome::Failed,
        lines,
    };
    if k_s > 0 {
        return Ok(failed(vec![format!(
            "the claim says R is empty, and R has {k_s} state{}",
            if k_s == 1 { "" } else { "s" }
        )]));
    }
    if stated.owing.is_empty() {
        // `(region)`: R is empty, of either kind.
        return Ok(Report {
            outcome: Outcome::Held,
            lines: Vec::new(),
        });
    }
    if !l.any_alive {
        return Ok(failed(vec![
            "(owing …) says alive nodes owe, and every node is dead — that is `(false)`"
                .to_string(),
        ]));
    }
    let states = fixpoints(root, terms, ast, &l.candidates, &l.frontier, opts)?;
    let wants: Vec<Want> = stated.owing.iter().map(|m| want(ast, m)).collect();
    Ok(match matching(&wants, &states, closed) {
        Some(_) => Report {
            outcome: Outcome::Held,
            lines: Vec::new(),
        },
        None => {
            let mut lines = vec![format!(
                "(owing …) lists {} state{}, and the alive owing nodes with no alive node one \
                 fact up are {}",
                wants.len(),
                if wants.len() == 1 { "" } else { "s" },
                states.len()
            )];
            for s in &states {
                if !wants.iter().any(|w| fits(w, s, closed)) {
                    lines.push(format!(
                        "  an owing state no model lists: {}",
                        describe(s, closed)
                    ));
                }
            }
            failed(lines)
        }
    })
}
